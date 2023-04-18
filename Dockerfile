FROM rust
WORKDIR /build

COPY rust-toolchain /build/
RUN rustup show

COPY wasm-instrument-c-api/Cargo.toml /build/wasm-instrument-c-api/Cargo.toml
COPY wasm-instrument-c-api/Cargo.lock /build/wasm-instrument-c-api/Cargo.lock
RUN mkdir -p wasm-instrument-c-api/src && touch wasm-instrument-c-api/src/lib.rs
RUN cd wasm-instrument-c-api && cargo fetch --locked -v

CMD echo "specify a command to run. nothing todo"
