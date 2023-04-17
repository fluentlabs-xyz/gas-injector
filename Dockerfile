FROM rust
WORKDIR /build

COPY rust-toolchain /build/
RUN rustup show

COPY wasm-instrument-c-api/Cargo.toml /build/wasm-instrument-c-api/Cargo.toml
COPY wasm-instrument-c-api/Cargo.lock /build/wasm-instrument-c-api/Cargo.lock

CMD echo "specify a command to run. nothing todo"
