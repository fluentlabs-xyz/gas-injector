
.PHONY: build
build:
	(cd zkwasm-wasm-instrument-binding; make build)
	go build -o build/main main.go
.PHONY: run
run: build
	LD_LIBRARY_PATH="./zkwasm-wasm-instrument-binding:$LD_LIBRARY_PATH" go run -v main.go
.PHONY: test
test:
	LD_LIBRARY_PATH="$(PWD)/zkwasm-wasm-instrument-binding:$LD_LIBRARY_PATH" go test ./...
.PHONY: clean
clean:
	rm -rf build
	(cd zkwasm-wasm-instrument-binding; make clean)
