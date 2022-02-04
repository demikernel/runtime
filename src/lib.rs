// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Crate Configuration
//==============================================================================

#![cfg_attr(feature = "strict", deny(clippy:all))]
#![deny(clippy::all)]
#![feature(new_uninit)]
#![feature(test)]

//==============================================================================
// Imports
//==============================================================================

#[macro_use]
extern crate derive_more;

use task::SchedulerRuntime;

//==============================================================================
// Exports
//==============================================================================

pub mod fail;
pub mod memory;
pub mod network;
pub mod queue;
pub mod task;
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

//==============================================================================
// Traits
//==============================================================================

/// Demikernel Runtime
pub trait Runtime:
    Clone + Unpin + SchedulerRuntime + UtilsRuntime + NetworkRuntime + MemoryRuntime + 'static
{
}
