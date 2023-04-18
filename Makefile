CARGO_BINARY ?= cargo
CARGO_TARGET ?=
CARGO_TARGET_FLAG ?=

ifneq ($(CARGO_TARGET),)
CARGO_TARGET_FLAG := --target $(CARGO_TARGET)
endif

.PHONY: build-apple-amd64 build-apple-aarch64 build-linux-amd64 build run test clean
build-apple-amd64:
	rustup target add x86_64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=x86_64-apple-darwin --manifest-path wasm-instrument-c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	cp wasm-instrument-c-api/target/x86_64-apple-darwin/release/libgas_injector.dylib packaged/lib/darwin-amd64/
build-apple-aarch64:
	rustup target add aarch64-apple-darwin
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build --target=aarch64-apple-darwin --manifest-path wasm-instrument-c-api/Cargo.toml --release #--no-default-features $(capi_compiler_features)
	cp wasm-instrument-c-api/target/aarch64-apple-darwin/release/libgas_injector.dylib packaged/lib/darwin-aarch64/
build-linux-amd64:
	rustup target add x86_64-unknown-linux-gnu
#	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
	$(CARGO_BINARY) build --target=x86_64-unknown-linux-gnu --manifest-path wasm-instrument-c-api/Cargo.toml --release
	mkdir -p packaged/lib/linux-amd64/
	cp wasm-instrument-c-api/target/x86_64-unknown-linux-gnu/release/libgas_injector.so packaged/lib/linux-amd64/
build-linux-amd64-docker:
	docker build -t rust/gas-injector--linux-amd64 -f Dockerfile .
	docker run --rm --platform=linux/amd64 -v "$(shell pwd):/build" -it rust/gas-injector--linux-amd64 bash -c "cd /build ; make build-linux-amd64"
build-linux-aarch64:
	rustup target add aarch64-unknown-linux-gnu
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc $(CARGO_BINARY) build --target=x86_64-unknown-linux-gnu --manifest-path wasm-instrument-c-api/Cargo.toml --release
	cp wasm-instrument-c-api/target/x86_64-unknown-linux-gnu/release/libgas_injector.so packaged/lib/linux-aarch64/
build-linux-aarch64-docker:
	docker build -t rust/gas-injector--linux-aarch64 -f Dockerfile .
	docker run --rm --platform=linux/aarch64 -v "$(shell pwd):/build" -it rust/gas-injector--linux-aarch64 bash -c "cd /build && make build-linux-aarch64"

#build-apple: build-apple-aarch64 build-apple-amd64
#build: build-apple-amd64 build-apple-aarch64 build-linux-amd64-docker build-linux-aarch64-docker

build:
	go build -o build/main main.go
run:
	LD_LIBRARY_PATH="$(PWD)/libs${LD_LIBRARY_PATH:+":$LD_LIBRARY_PATH"}" go run -v main.go
test:
	go test ./...
clean:
	cd wasm-instrument-c-api && make clean
	go clean -testcache
