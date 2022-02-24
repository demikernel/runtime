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

extern crate test;

use self::{
    memory::MemoryRuntime, network::NetworkRuntime, task::SchedulerRuntime, utils::UtilsRuntime,
};

//==============================================================================
// Exports
//==============================================================================

mod collections;
pub mod fail;
pub mod logging;
pub mod memory;
pub mod network;
pub mod queue;
pub mod task;
pub mod timer;
pub mod types;
pub mod utils;
pub mod watched;

//==============================================================================
// Traits
//==============================================================================

/// Demikernel Runtime
pub trait Runtime:
    Clone + Unpin + SchedulerRuntime + UtilsRuntime + NetworkRuntime + MemoryRuntime + 'static
{
}
