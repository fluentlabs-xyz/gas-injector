package zkwasm_gas_injector

// #cgo CFLAGS: -I${SRCDIR}/packaged/include
// #cgo LDFLAGS: -lgas_injector
//
// #cgo linux,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-amd64 -L${SRCDIR}/packaged/lib/linux-amd64
// //#cgo linux,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-aarch64 -L${SRCDIR}/packaged/lib/linux-aarch64
// //#cgo darwin,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-amd64 -L${SRCDIR}/packaged/lib/darwin-amd64
// #cgo darwin,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-aarch64 -L${SRCDIR}/packaged/lib/darwin-aarch64
//
// #include <wasm-instrument-c-api/gas_injector.h>
import "C"

import (
	"unsafe"

	"github.com/pkg/errors"

	_ "github.com/wasm0/zkwasm-gas-injector/packaged/include"
	_ "github.com/wasm0/zkwasm-gas-injector/packaged/lib"
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
