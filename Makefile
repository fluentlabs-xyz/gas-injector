
.PHONY: build
build:
	go build -o build/main main.go
.PHONY: run
run:
	LD_LIBRARY_PATH="./libs${LD_LIBRARY_PATH:+":$LD_LIBRARY_PATH"}" go run -v main.go
.PHONY: test
test:
	LD_LIBRARY_PATH="$(PWD)/libs{LD_LIBRARY_PATH:+":$LD_LIBRARY_PATH"}" go test ./...
.PHONY: clean
clean:
	rm -rf build
	(cd zkwasm-wasm-instrument-binding; make clean)
