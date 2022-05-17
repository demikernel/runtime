// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

mod buffer;
mod databuffer;

//==============================================================================
// Imports
//==============================================================================

use crate::{
    fail::Fail,
    types::demi_sgarray_t,
};

//==============================================================================
// Exports
//==============================================================================

pub use self::{
    buffer::Buffer,
    databuffer::DataBuffer,
};

//==============================================================================
// Traits
//==============================================================================

/// Memory Runtime
pub trait MemoryRuntime {
    /// Creates a [demi_sgarray_t] from a [Buffer].
    fn into_sgarray(&self, buf: Box<dyn Buffer>) -> Result<demi_sgarray_t, Fail>;

    /// Allocates a [demi_sgarray_t].
    fn alloc_sgarray(&self, size: usize) -> Result<demi_sgarray_t, Fail>;

    /// Releases a [demi_sgarray_t].
    fn free_sgarray(&self, sga: demi_sgarray_t) -> Result<(), Fail>;

    /// Clones a [demi_sgarray_t] into a [Buffer].
    fn clone_sgarray(&self, sga: &demi_sgarray_t) -> Result<Box<dyn Buffer>, Fail>;
}
