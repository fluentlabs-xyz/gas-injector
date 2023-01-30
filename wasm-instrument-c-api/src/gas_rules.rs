use std::num::NonZeroU32;
use wasm_instrument::gas_metering::{MemoryGrowCost, Rules};
use wasm_instrument::parity_wasm::elements::Instruction;

pub struct CustomConstantCostRules {
    instruction_cost: u32,
    memory_grow_cost: u32,
    call_per_local_cost: u32,
}

impl CustomConstantCostRules {
    pub fn new(instruction_cost: u32, memory_grow_cost: u32, call_per_local_cost: u32) -> Self {
        Self { instruction_cost, memory_grow_cost, call_per_local_cost }
    }
}

impl Rules for CustomConstantCostRules {
    fn instruction_cost(&self, instruction: &Instruction) -> Option<u32> {
        match instruction {
            Instruction::Unreachable => Some(1),
            Instruction::Nop => Some(1),
            Instruction::Block(_) => Some(1),
            Instruction::Loop(_) => Some(1),
            Instruction::If(_) => Some(1),
            Instruction::Else => Some(90),
            Instruction::End => Some(1),
            Instruction::Br(_) => Some(90),
            Instruction::BrIf(_) => Some(90),
            Instruction::BrTable(_) => Some(120),
            Instruction::Return => Some(90),

            // Call operators
            Instruction::Call(_) => Some(90),
            Instruction::CallIndirect(_,_) => Some(10000),

            // Parametric operators
            Instruction::Drop => Some(120),
            Instruction::Select => Some(120),

            // Variable access
            Instruction::GetLocal(_) => Some(120),
            Instruction::SetLocal(_) => Some(120),
            Instruction::TeeLocal(_)=> Some(120),
            Instruction::GetGlobal(_) => Some(120),
            Instruction::SetGlobal(_) => Some(120),

            // Memory-related operators
            Instruction::I32Load(_,_) => Some(120),
            Instruction::I64Load(_,_) => Some(120),
            Instruction::F32Load(_,_) => Some(120),
            Instruction::F64Load(_,_) => Some(120),
            Instruction::I32Load8S(_,_) => Some(120),
            Instruction::I32Load8U(_,_) => Some(120),
            Instruction::I32Load16S(_,_) => Some(120),
            Instruction::I32Load16U(_,_) => Some(120),
            Instruction::I64Load8S(_,_) => Some(120),
            Instruction::I64Load8U(_,_) => Some(120),
            Instruction::I64Load16S(_,_) => Some(120),
            Instruction::I64Load16U(_,_) => Some(120),
            Instruction::I64Load32S(_,_) => Some(120),
            Instruction::I64Load32U(_,_) => Some(120),
            Instruction::I32Store(_,_) => Some(120),
            Instruction::I64Store(_,_) => Some(120),
            Instruction::F32Store(_,_) => Some(120),
            Instruction::F64Store(_,_) => Some(120),
            Instruction::F64Abs => Some(120),
            Instruction::F64Neg => Some(120),
            Instruction::F64Ceil => Some(120),
            Instruction::F64Floor => Some(120),
            Instruction::F64Trunc => Some(120),
            Instruction::F64Nearest => Some(120),
            Instruction::F64Sqrt => Some(120),
            Instruction::F64Add => Some(120),
            Instruction::F64Sub => Some(120),
            Instruction::F64Mul => Some(120),
            Instruction::F64Div => Some(120),
            Instruction::F64Min => Some(120),
            Instruction::F64Max => Some(120),
            Instruction::F64Copysign => Some(120),
            Instruction::I32Store8(_,_) => Some(120),
            Instruction::I32Store16(_,_) => Some(120),
            Instruction::I64Store8(_,_) => Some(120),
            Instruction::I64Store16(_,_) => Some(120),
            Instruction::I64Store32(_,_) => Some(120),
            Instruction::CurrentMemory(_) => Some(100),
            Instruction::GrowMemory(_) => Some(10000),

            // Constants opcodes
            Instruction::I32Const(_) => Some(1),
            Instruction::I64Const(_) => Some(1),
            Instruction::F32Const(_) => Some(1),
            Instruction::F64Const(_) => Some(1),

            // Comparison operators
            Instruction::I32Eqz => Some(45),
            Instruction::I32Eq => Some(45),
            Instruction::I32Ne => Some(45),
            Instruction::I32LtS => Some(45),
            Instruction::I32LtU => Some(45),
            Instruction::I32GtS => Some(45),
            Instruction::I32GtU => Some(45),
            Instruction::I32LeS => Some(45),
            Instruction::I32LeU => Some(45),
            Instruction::I32GeS => Some(45),
            Instruction::I32GeU => Some(45),
            Instruction::I64Eqz => Some(45),
            Instruction::I64Eq => Some(45),
            Instruction::I64Ne => Some(45),
            Instruction::I64LtS => Some(45),
            Instruction::I64LtU => Some(45),
            Instruction::I64GtS => Some(45),
            Instruction::I64GtU => Some(45),
            Instruction::I64LeS => Some(45),
            Instruction::I64LeU => Some(45),
            Instruction::I64GeS => Some(45),
            Instruction::I64GeU => Some(45),
            Instruction::F32Eq => Some(45),
            Instruction::F32Ne => Some(45),
            Instruction::F32Lt => Some(45),
            Instruction::F32Gt => Some(45),
            Instruction::F32Le => Some(45),
            Instruction::F32Ge => Some(45),
            Instruction::F64Eq => Some(45),
            Instruction::F64Ne => Some(45),
            Instruction::F64Lt => Some(45),
            Instruction::F64Gt => Some(45),
            Instruction::F64Le => Some(45),
            Instruction::F64Ge => Some(45),

            // Numeric operators
            Instruction::I32Clz => Some(45),
            Instruction::I32Ctz => Some(45),
            Instruction::I32Popcnt => Some(100),
            Instruction::I32Add => Some(45),
            Instruction::I32Sub => Some(45),
            Instruction::I32Mul => Some(45),
            Instruction::I32DivS => Some(36000),
            Instruction::I32DivU => Some(36000),
            Instruction::I32RemS => Some(36000),
            Instruction::I32RemU => Some(36000),
            Instruction::I32And => Some(45),
            Instruction::I32Or => Some(45),
            Instruction::I32Xor => Some(45),
            Instruction::I32Shl => Some(67),
            Instruction::I32ShrS => Some(67),
            Instruction::I32ShrU => Some(67),
            Instruction::I32Rotl => Some(90),
            Instruction::I32Rotr => Some(90),
            Instruction::I64Clz => Some(45),
            Instruction::I64Ctz => Some(45),
            Instruction::I64Popcnt => Some(100),
            Instruction::I64Add => Some(45),
            Instruction::I64Sub => Some(45),
            Instruction::I64Mul => Some(45),
            Instruction::I64DivS => Some(36000),
            Instruction::I64DivU => Some(36000),
            Instruction::I64RemS => Some(36000),
            Instruction::I64RemU => Some(36000),
            Instruction::I64And => Some(45),
            Instruction::I64Or => Some(45),
            Instruction::I64Xor => Some(45),
            Instruction::I64Shl => Some(67),
            Instruction::I64ShrS => Some(67),
            Instruction::I64ShrU => Some(67),
            Instruction::I64Rotl => Some(90),
            Instruction::I64Rotr => Some(90),
            Instruction::F32Abs => Some(90),
            Instruction::F32Neg => Some(90),
            Instruction::F32Ceil => Some(90),
            Instruction::F32Floor => Some(90),
            Instruction::F32Trunc => Some(90),
            Instruction::F32Nearest => Some(90),
            Instruction::F32Sqrt => Some(90),
            Instruction::F32Add => Some(90),
            Instruction::F32Sub => Some(90),
            Instruction::F32Mul => Some(90),
            Instruction::F32Div => Some(90),
            Instruction::F32Min => Some(90),
            Instruction::F32Max => Some(90),
            Instruction::F32Copysign => Some(90),

            // Conversions
            Instruction::I32WrapI64 => Some(90),
            Instruction::I32TruncSF32 => Some(90),
            Instruction::I32TruncUF32 => Some(90),
            Instruction::I32TruncSF64 => Some(90),
            Instruction::I32TruncUF64 => Some(90),
            Instruction::I64ExtendSI32 => Some(90),
            Instruction::I64ExtendUI32 => Some(90),
            Instruction::I64TruncSF32 => Some(90),
            Instruction::I64TruncUF32 => Some(90),
            Instruction::I64TruncSF64 => Some(90),
            Instruction::I64TruncUF64 => Some(90),
            Instruction::F32ConvertSI32 => Some(90),
            Instruction::F32ConvertUI32 => Some(90),
            Instruction::F32ConvertSI64 => Some(90),
            Instruction::F32ConvertUI64 => Some(90),
            Instruction::F32DemoteF64 => Some(90),
            Instruction::F64ConvertSI32 => Some(90),
            Instruction::F64ConvertUI32 => Some(90),
            Instruction::F64ConvertSI64 => Some(90),
            Instruction::F64ConvertUI64 => Some(90),
            Instruction::F64PromoteF32 => Some(90),

            // Reinterpretations
            Instruction::I32ReinterpretF32 => Some(90),
            Instruction::I64ReinterpretF64 => Some(90),
            Instruction::F32ReinterpretI32 => Some(90),
            Instruction::F64ReinterpretI64 => Some(90),

            #[allow(unreachable_patterns)]
            _ => Some(self.instruction_cost),
        }
    }

    fn memory_grow_cost(&self) -> MemoryGrowCost {
        NonZeroU32::new(self.memory_grow_cost).map_or(MemoryGrowCost::Free, MemoryGrowCost::Linear)
    }

    fn call_per_local_cost(&self) -> u32 {
        self.call_per_local_cost
    }
}
