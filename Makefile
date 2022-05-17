# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#===============================================================================
# Default Paths
#===============================================================================

export PREFIX ?= $(HOME)
export PKG_CONFIG_PATH ?= $(shell find $(PREFIX)/lib/ -name '*pkgconfig*' -type d | xargs | sed -e 's/\s/:/g')
export LD_LIBRARY_PATH ?= $(HOME)/lib:$(shell find $(PREFIX)/lib/ -name '*x86_64-linux-gnu*' -type d | xargs | sed -e 's/\s/:/g')

#===============================================================================
# Toolchain Configuration
#===============================================================================

# Rust Toolchain
export BUILD ?= --release
export CARGO ?= $(HOME)/.cargo/bin/cargo

# Switch for Liburing
ifeq ($(LIBURING),yes)
export BUILD += --features=liburing
endif

# Switch for Libdpdk
ifeq ($(LIBDPDK),yes)
DRIVER ?= $(shell [ ! -z "`lspci | grep -E "ConnectX-[4,5]"`" ] && echo mlx5 || echo mlx4)
export BUILD += --features=libdpdk --features=$(DRIVER)
endif

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
