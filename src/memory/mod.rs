// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

mod buffer;
mod bytes;
mod bytesmut;

//==============================================================================
// Imports
//==============================================================================

use crate::{fail::Fail, types::dmtr_sgarray_t};

//==============================================================================
// Exports
//==============================================================================

pub use self::{buffer::Buffer, bytes::Bytes, bytesmut::BytesMut};

//==============================================================================
// Traits
//==============================================================================

/// Memory Runtime
pub trait MemoryRuntime {
    /// Memory Buffer
    type Buf: Buffer;

    /// Creates a [dmtr_sgarray_t] from a [Buffer].
    fn into_sgarray(&self, buf: Self::Buf) -> Result<dmtr_sgarray_t, Fail>;

    /// Allocates a [dmtr_sgarray_t].
    fn alloc_sgarray(&self, size: usize) -> Result<dmtr_sgarray_t, Fail>;

    /// Releases a [dmtr_sgarray_t].
    fn free_sgarray(&self, sga: dmtr_sgarray_t) -> Result<(), Fail>;

    /// Clones a [dmtr_sgarray_t] into a [Buffer].
    fn clone_sgarray(&self, sga: &dmtr_sgarray_t) -> Result<Self::Buf, Fail>;
}
