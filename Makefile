# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#=======================================================================================================================
# Default Paths
#=======================================================================================================================

export PREFIX ?= $(HOME)
export PKG_CONFIG_PATH ?= $(shell find $(PREFIX)/lib/ -name '*pkgconfig*' -type d | xargs | sed -e 's/\s/:/g')
export LD_LIBRARY_PATH ?= $(HOME)/lib:$(shell find $(PREFIX)/lib/ -name '*x86_64-linux-gnu*' -type d | xargs | sed -e 's/\s/:/g')

#=======================================================================================================================
# Toolchain Configuration
#=======================================================================================================================

export CARGO ?= $(HOME)/.cargo/bin/cargo

# Switches:
# - TEST    Test to run.
# - BENCH   Microbenchmark to run.
# - FLAGS   Flags passed to cargo.

# Switch for Perftools
ifeq ($(PERFTOOLS),yes)
export FLAGS += --features=perftools
endif

# Switch for Liburing
ifeq ($(LIBURING),yes)
export FLAGS += --features=liburing
endif

# Switch for Libdpdk
ifeq ($(LIBDPDK),yes)
DRIVER ?= $(shell [ ! -z "`lspci | grep -E "ConnectX-[4,5]"`" ] && echo mlx5 || echo mlx4)
export FLAGS += --features=libdpdk --features=$(DRIVER)
endif

# Set build mode.
ifneq ($(DEBUG),yes)
export BUILD = release
else
export BUILD = dev
endif
export FLAGS += --profile $(BUILD)

#=======================================================================================================================

# Builds source code.
all:
	$(CARGO) build --all $(FLAGS)

# Runs regression tests.
test:
	$(CARGO) test $(FLAGS) $(TEST) -- --nocapture

# Runs microbenchmarks.
bench:
	$(CARGO) bench $(FLAGS) $(BENCH) -- --nocapture

# Check code style formatting.
check-fmt: check-fmt-rust

# Check code style formatting for Rust.
check-fmt-rust:
	$(CARGO) fmt --all -- --check

# Builds documentation.
doc:
	$(CARGO) doc $(FLAGS) --no-deps

# Cleans up all build artifacts.
clean:
	rm -rf target && \
	$(CARGO) clean && \
	rm -f Cargo.lock
