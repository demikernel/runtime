// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![cfg_attr(feature = "strict", deny(clippy:all))]
#![deny(clippy::all)]
#![feature(new_uninit)]
#![feature(test)]

pub mod fail;
pub mod memory;

pub use self::memory::Buffer as RuntimeBuf;
