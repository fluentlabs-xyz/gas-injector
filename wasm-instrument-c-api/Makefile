CARGO_BINARY ?= cargo
CARGO_TARGET ?=
CARGO_TARGET_FLAG ?=

ifneq ($(CARGO_TARGET),)
CARGO_TARGET_FLAG := --target $(CARGO_TARGET)
endif

.PHONY: build clean
build-test:
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build $(CARGO_TARGET_FLAG) --manifest-path Cargo.toml
build:
	RUSTFLAGS="${RUSTFLAGS}" $(CARGO_BINARY) build $(CARGO_TARGET_FLAG) --manifest-path Cargo.toml --release \
    		--no-default-features $(capi_compiler_features)
clean:
	$(CARGO_BINARY) clean
