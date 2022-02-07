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

use self::{
    memory::MemoryRuntime, network::NetworkRuntime, task::SchedulerRuntime, utils::UtilsRuntime,
};

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

//==============================================================================
// Traits
//==============================================================================

/// Demikernel Runtime
pub trait Runtime:
    Clone + Unpin + SchedulerRuntime + UtilsRuntime + NetworkRuntime + MemoryRuntime + 'static
{
}
