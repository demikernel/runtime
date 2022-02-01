// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![cfg_attr(feature = "strict", deny(clippy:all))]
#![deny(clippy::all)]
#![feature(new_uninit)]
#![feature(test)]

#[macro_use]
extern crate derive_more;

pub mod fail;
pub mod memory;
pub mod queue;

pub use self::memory::Buffer as RuntimeBuf;
