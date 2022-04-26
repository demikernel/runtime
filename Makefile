# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#===============================================================================

export CARGO ?= $(HOME)/.cargo/bin/cargo

export BUILD ?= --release

#===============================================================================

all: check-fmt
	$(CARGO) build --all $(BUILD) $(CARGO_FLAGS)

test: check-fmt
	$(CARGO) test $(BUILD) $(CARGO_FLAGS) $(TEST) -- --nocapture

bench: check-fmt
	$(CARGO) bench $(CARGO_FLAGS) $(TEST)

check-fmt: check-fmt-rust

check-fmt-rust:
	$(CARGO) fmt -- --check

doc:
	$(CARGO) doc $(CARGO_FLAGS) --no-deps

clean:
	rm -rf target && \
	$(CARGO) clean && \
	rm -f Cargo.lock
