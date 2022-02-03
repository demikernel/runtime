// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Crate Configuration
//==============================================================================

#![cfg_attr(feature = "strict", deny(clippy:all))]
#![deny(clippy::all)]
#![feature(new_uninit)]
#![feature(test)]
// The following two features are on their way to get implemented.
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]

//==============================================================================
// Imports
//==============================================================================

#[macro_use]
extern crate derive_more;

//==============================================================================
// Exports
//==============================================================================

pub mod fail;
pub mod memory;
pub mod network;
pub mod queue;
pub mod types;
pub mod utils;
pub use self::memory::Buffer as RuntimeBuf;
pub use self::{
    memory::MemoryRuntime,
    network::{
        types::{Ipv4Addr, MacAddress},
        NetworkRuntime, PacketBuf,
    },
    utils::UtilsRuntime,
};
