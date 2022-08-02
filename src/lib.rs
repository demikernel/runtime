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

//==============================================================================
// Exports
//==============================================================================

pub mod collections;
pub mod fail;
pub mod logging;
pub mod memory;
pub mod network;
pub mod queue;
pub mod timer;
pub mod types;
pub mod watched;
pub use queue::{
    QDesc,
    QResult,
    QToken,
    QType,
};

pub use scheduler;

#[cfg(feature = "perftools")]
pub use perftools;

#[cfg(feature = "liburing")]
pub use liburing;

#[cfg(feature = "libdpdk")]
pub use dpdk_rs as libdpdk;

//==============================================================================
// Traits
//==============================================================================

/// Demikernel Runtime
pub trait Runtime: Clone + Unpin + 'static {}
