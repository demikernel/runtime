# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#===============================================================================

export CARGO ?= $(HOME)/.cargo/bin/cargo

export BUILD ?= --release

#===============================================================================

all:
	$(CARGO) build --all $(BUILD) $(CARGO_FLAGS)

test:
	$(CARGO) test $(BUILD) $(CARGO_FLAGS) $(TEST) -- --nocapture

bench:
	$(CARGO) bench $(CARGO_FLAGS) $(TEST)

clean:
	rm -rf target && \
	$(CARGO) clean && \
	rm -f Cargo.lock
