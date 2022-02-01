// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use super::Bytes;
use crate::fail::Fail;
use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
    sync::Arc,
};

//==============================================================================
// Structures
//==============================================================================

/// Mutable Buffer
pub struct BytesMut {
    /// Underlying Data
    data: Arc<[u8]>,
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate Functions for Mutable Buffers
impl BytesMut {
    /// Creates a zeroed [BytesMut].
    pub fn zeroed(capacity: usize) -> Result<Self, Fail> {
        if capacity == 0 {
            return Err(Fail::Invalid {
                details: "zero-capacity buffer",
            });
        }
        Ok(Self {
            data: unsafe { Arc::new_zeroed_slice(capacity).assume_init() },
        })
    }

    /// Converts the target [BytesMut] into a [Bytes].
    pub fn freeze(self) -> Bytes {
        let len: usize = self.data.len();
        Bytes::new(Some(self.data), 0, len)
    }
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// Partial Equality Trait Implementation for Mutable Buffers
impl PartialEq for BytesMut {
    fn eq(&self, rhs: &Self) -> bool {
        self[..] == rhs[..]
    }
}

/// Equality Trait Implementation for Mutable Buffers
impl Eq for BytesMut {}

/// Conversion Trait Implementation for Mutable Buffers
impl From<&[u8]> for BytesMut {
    fn from(buf: &[u8]) -> Self {
        let mut b = Self::zeroed(buf.len()).unwrap();
        b[..].copy_from_slice(buf);
        b
    }
}

/// Debug Trait Implementation for Mutable Buffers
impl Debug for BytesMut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BytesMut({:?})", &self[..])
    }
}

/// De-Reference Trait Implementation for Mutable Buffers
impl Deref for BytesMut {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data[..]
    }
}

/// Mutable De-Reference Trait Implementation for Mutable Buffers
impl DerefMut for BytesMut {
    fn deref_mut(&mut self) -> &mut [u8] {
        Arc::get_mut(&mut self.data).unwrap()
    }
}
