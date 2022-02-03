// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

mod buffer;
mod bytes;
mod bytesmut;

#[cfg(test)]
mod tests;

//==============================================================================
// Imports
//==============================================================================

use crate::types::dmtr_sgarray_t;

//==============================================================================
// Exports
//==============================================================================

pub use self::buffer::Buffer;
pub use self::bytes::Bytes;
pub use self::bytesmut::BytesMut;

//==============================================================================
// Traits
//==============================================================================

/// Memory Runtime
pub trait MemoryRuntime {
    /// Memory Buffer
    type Buf: Buffer;

    /// Creates a [dmtr_sgarray_t] from a [Buffer].
    fn into_sgarray(&self, buf: Self::Buf) -> dmtr_sgarray_t;

    /// Allocates a [dmtr_sgarray_t].
    fn alloc_sgarray(&self, size: usize) -> dmtr_sgarray_t;

    /// Releases a [dmtr_sgarray_t].
    fn free_sgarray(&self, sga: dmtr_sgarray_t);

    /// Clones a [dmtr_sgarray_t] into a [Buffer].
    fn clone_sgarray(&self, sga: &dmtr_sgarray_t) -> Self::Buf;
}
