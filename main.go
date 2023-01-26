package main

import (
	_ "embed"
	"fmt"
	"time"

	cgo_wrapper "github.com/wasm0/zkwasm-wasm-instrument-cgo/cgo-wrapper"
)

//go:embed cgo-wrapper/testdata/fixtures/stack-height/simple.wat
var wasmData string

func main() {
	timeStart := time.Now().UnixMilli()
	for i := 1; i <= 1; i++ {
		// TODO convert WAT file or binary wasm to bytes slice
		moduleBytes := []byte(wasmData)
		//fmt.Printf("moduleBytes before '%s'\n", moduleBytes)
		moduleBytesRes, err := cgo_wrapper.Inject(
			moduleBytes,
			cgo_wrapper.InjectTypeBoth,
			cgo_wrapper.InjectGasTypeMutGlobal,
			1,
			0,
			1,
			1024,
			cgo_wrapper.ReturnFormatWat,
		)
		if err != nil {
			fmt.Printf("failed to Inject, reason '%s'", err)
		}
		// TODO try to convert wasm binary to WAT
		//fmt.Printf("moduleBytes after '%s'\n", moduleBytesRes)
		fmt.Printf("iteration '%v' len(moduleBytesRes) '%v' \n", i, len(moduleBytesRes))
		// TODO module bytes to WAT
		//time.Sleep(300 * time.Millisecond)
	}
	timeEnd := time.Now().UnixMilli()
	timeTook := timeEnd - timeStart
	fmt.Printf("timeTook %v\n", timeTook)
}
