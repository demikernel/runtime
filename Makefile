# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#=======================================================================================================================

# This is a trick to enable portability across MAKE and NMAKE.
# MAKE recognizes line continuation in comments but NMAKE doesn't.
# NMAKE               \
!ifndef 0 #           \
!include windows.mk # \
!else
include linux.mk
#                     \
!endif

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
	$(CARGO) clean
	$(RM) Cargo.lock