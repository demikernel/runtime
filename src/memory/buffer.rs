// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use ::std::{
    fmt::Debug,
    ops::Deref,
};

//==============================================================================
// Traits
//==============================================================================

/// Memory Buffer
pub trait Buffer: Clone + Debug + Deref<Target = [u8]> + Sized + Unpin {
    /// Creates an empty [Buffer].
    fn empty() -> Self;

    /// Creates a [Buffer] from a [u8].
    fn from_slice(bytes: &[u8]) -> Self;

    /// Removes the first `nbytes` bytes of the target [Buffer].
    fn adjust(&mut self, nbytes: usize);

    /// Removes the last `nbytes` bytes of the target [Buffer].
    fn trim(&mut self, nbytes: usize);
}
