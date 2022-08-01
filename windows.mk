# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

#=======================================================================================================================
# Environment Variables
#=======================================================================================================================

PREFIX = $(USERPROFILE)
!if [set LIBDPDK_PATH=$(PREFIX)\AppData\Local\dpdk]
!endif

#=======================================================================================================================
# Tools
#=======================================================================================================================

CARGO = cargo
RM = del
!if [set CC=clang]
!endif

#=======================================================================================================================
# Switches
#=======================================================================================================================

# Switch for Perftools
!ifdef PERFTOOLS
FLAGS = $(FLAGS) --features=perftools
!endif

# Switch for Libdpdk
!ifdef LIBDPDK
DRIVER = mlx4
FLAGS = $(FLAGS) --features=libdpdk --features=$(DRIVER)
!endif

# Set build mode.
!ifndef DEBUG
BUILD = release
!else
BUILD = dev
!endif

# Set build flags.
FLAGS = $(FLAGS) --profile $(BUILD)
