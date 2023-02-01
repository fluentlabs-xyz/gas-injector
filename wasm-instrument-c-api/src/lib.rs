extern crate core;

mod gas_rules;

use core::slice;
use std::ffi::c_int;
use std::os::raw::c_uchar;
use std::ptr::{null_mut};

use wasm_instrument::gas_metering;
use wasm_instrument::gas_metering::{mutable_global, host_function};
use wasm_instrument::inject_stack_limiter;
use wasm_instrument::parity_wasm::elements;

pub fn inject_into_utf8_wat_or_binary_wasm(
    utf8_wat_or_binary_wasm_vector: Vec<u8>,
    inject_type: u32,
    inject_gas_type: u32,
    default_instruction_cost: u32,
    default_memory_grow_cost: u32,
    call_per_local_cost: u32,
    stack_limit: u32,
) -> Result<Vec<u8>, String> {
    let utf8_wat_or_binary_wasm_bytes = match wat::parse_bytes(&utf8_wat_or_binary_wasm_vector[..]) {
        Ok(v) => v,
        Err(_) => return Err("Failed to parse bytes wat or wasm bytes".to_string())
    };

    let module: elements::Module = match elements::deserialize_buffer(utf8_wat_or_binary_wasm_bytes.as_ref()) {
        Ok(v) => v,
        Err(_) => return Err("Failed to deserialize wat or wasm bytes into module".to_string())
    };
    let mut res_module: elements::Module = module;
    if inject_type == 1 || inject_type == 3 {
        res_module = match res_module.parse_names() {
            Ok(v) => v,
            Err(_) => return Err("Failed to parse names".to_string())
        };
        let rules = gas_rules::CustomConstantCostRules::new(
            default_instruction_cost,
            default_memory_grow_cost,
            call_per_local_cost
        );
        if inject_gas_type == 1 {
            let backend = mutable_global::Injector::new("gas_left");
            res_module = match gas_metering::inject(
                res_module,
                backend,
                &rules,
            ) {
                Ok(v) => v,
                Err(_) => return Err("Failed to instrument with gas metering (global export version)".to_string())
            };
        } else {
            let backend = host_function::Injector::new("env", "gas");
            res_module = match gas_metering::inject(
                res_module,
                backend,
                &rules,
            ) {
                Ok(v) => v,
                Err(_) => return Err("Failed to instrument with gas metering (host function version)".to_string())
            };
        }
    }
    if inject_type == 2 || inject_type == 3 {
        res_module = match inject_stack_limiter(
            res_module,
            stack_limit,
        ) {
            Ok(v) => v,
            Err(_) => return Err("Failed to instrument with stack limiting".to_string())
        };
    }

    match elements::serialize(res_module) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string())
    }
}

#[repr(C)]
pub struct ResultStruct {
    len: usize,
    data: *mut c_uchar,
    exit_code: usize
}

// A utf-8 string which is a *.wat file to be parsed.
// A binary WebAssembly file starting with b"\0asm"
#[no_mangle]
pub unsafe extern "C" fn inject_into_utf8_wat_or_binary_wasm_external(
    utf8_wat_or_binary_wasm_bytes_ptr_c: *mut c_uchar,
    utf8_wat_or_binary_wasm_bytes_length_c: usize,
    inject_type: c_int,
    inject_gas_type: c_int,
    instruction_cost: c_int,
    memory_grow_cost: c_int,
    call_per_local_cost: c_int,
    stack_limit: c_int,
    return_format: c_int
) -> ResultStruct {
    let bytes = unsafe {slice::from_raw_parts(utf8_wat_or_binary_wasm_bytes_ptr_c, utf8_wat_or_binary_wasm_bytes_length_c)};
    let utf8_wat_or_binary_wasm_vector: Vec<u8> = Vec::from(bytes);

    let res_wasm_bytes_vec = match inject_into_utf8_wat_or_binary_wasm(
        utf8_wat_or_binary_wasm_vector,
        inject_type as u32,
        inject_gas_type as u32,
        instruction_cost as u32,
        memory_grow_cost as u32,
        call_per_local_cost as u32,
        stack_limit as u32,
    ) {
        Ok(v) => v,
        Err(_) => {
            return ResultStruct {
                data: null_mut(),
                len: 0,
                exit_code: 1
            };
        },
    };
    let mut res_bytes;
    if return_format == 1 {
        let res_str = match wasmprinter::print_bytes(&res_wasm_bytes_vec) {
            Ok(v) => v,
            Err(_) => {
                return ResultStruct {
                    data: null_mut(),
                    len: 0,
                    exit_code: 1
                };
            },
        };
        res_bytes = res_str.into_bytes();
    } else {
        res_bytes = res_wasm_bytes_vec;
    }

    let inject_result = ResultStruct {
        data: res_bytes.as_mut_ptr(),
        len: res_bytes.len().clone(),
        exit_code: 0
    };

    std::mem::forget(res_bytes);

    inject_result
}

#[cfg(test)]
mod tests {
    use crate::inject_into_utf8_wat_or_binary_wasm;
    use std::{fs, str};

    #[test]
    fn test_hello_wat() {
        let data = fs::read_to_string("../testdata/fixtures/gas/hello.wat").expect("Unable to read file");
        let res = inject_into_utf8_wat_or_binary_wasm(
            data.into_bytes(),
            3,
            0,
            1,
            10000,
            1,
            1024,
        );
        match &res {
            Ok(v) => {
                println!("v.len(): {}", v.len());
                assert_eq!(v.len(), 4820);
                let res_str = match wasmprinter::print_bytes(&v) {
                    Ok(v) => v,
                    Err(e) => { println!("ERROR: Invalid UTF-8 sequence: {}", e); "".to_string() },
                };
                assert_eq!(res_str.len(), 39390);
                let res_bytes = res_str.into_bytes();
                let s = match str::from_utf8(res_bytes.as_ref()) {
                    Ok(v) => v,
                    Err(e) => { println!("ERROR: Invalid UTF-8 sequence: {}", e); "" }
                };
                // println!("{}", s);
            },
            Err(e_text) => println!("ERROR: {}", e_text),
        }
    }
}
