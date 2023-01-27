package cgo_wrapper

/*
#cgo LDFLAGS: -L "../libs" -lzkwasm_wasm_instrument_c_export
#cgo CFLAGS: -I "../libs"
#include "../libs/libzkwasm_wasm_instrument_c_export.h"
*/
import "C"

import (
	"unsafe"

	"github.com/pkg/errors"
)

type InjectType int32

const (
	InjectTypeNone InjectType = iota
	InjectTypeGas
	InjectTypeStackHeight
	InjectTypeBoth
)

type InjectGasType int32

const (
	InjectGasTypeHost InjectGasType = iota
	InjectGasTypeMutGlobal
)

type ReturnFormat int32

const (
	ReturnFormatWasm ReturnFormat = iota
	ReturnFormatWat
)

func Inject(
	watStrOrBinaryAsm []byte,
	injectType InjectType,
	injectGasType InjectGasType,
	instructionCost int,
	memoryGrowCost int,
	callPerLocalCost int,
	stackLimit int,
	returnFormat ReturnFormat,
) (moduleBytesRes []byte, err error) {
	if watStrOrBinaryAsm == nil {
		return nil, errors.New("parameter [watStrOrBinaryAsm] must be set")
	}
	if len(watStrOrBinaryAsm) <= 0 {
		return nil, nil
	}
	var argv = make([]C.uchar, len(watStrOrBinaryAsm))
	for i, item := range watStrOrBinaryAsm {
		argv[i] = C.uchar(item)
	}

	moduleBytesLen := C.ulong(len(argv))
	cResultStruct := C.inject_into_utf8_wat_or_binary_wasm_external(
		&argv[0],
		moduleBytesLen,
		C.int(injectType),
		C.int(injectGasType),
		C.int(instructionCost),
		C.int(memoryGrowCost),
		C.int(callPerLocalCost),
		C.int(stackLimit),
		C.int(returnFormat),
	)
	if cResultStruct.exit_code != 0 {
		return nil, errors.New("inject failed")
	}
	var sliceRes = unsafe.Slice(cResultStruct.data, int(cResultStruct.len))
	moduleBytesRes = make([]byte, len(sliceRes))
	for i, v := range sliceRes {
		moduleBytesRes[i] = byte(v)
	}

	return
}
