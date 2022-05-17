// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use ::std::{
    any::Any,
    fmt::Debug,
    ops::{
        Deref,
        DerefMut,
    },
};

//==============================================================================
// Traits
//==============================================================================

/// Memory Buffer
pub trait Buffer: Debug + Deref<Target = [u8]> + DerefMut<Target = [u8]> + Unpin {
    /// Clones the target [Buffer].
    fn clone(&self) -> Box<dyn Buffer>;

    /// Removes the first `nbytes` bytes of the target [Buffer].
    fn adjust(&mut self, nbytes: usize);

    /// Removes the last `nbytes` bytes of the target [Buffer].
    fn trim(&mut self, nbytes: usize);

    // Coerces the target [Buffer] into any data type.
    fn as_any(&self) -> &dyn Any;
}
