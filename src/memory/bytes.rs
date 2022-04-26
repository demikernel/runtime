// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use crate::memory::Buffer;
use ::std::{
    fmt::{
        self,
        Debug,
    },
    ops::Deref,
    sync::Arc,
};

//==============================================================================
// Structures
//==============================================================================

/// Non-Mutable Buffer
#[derive(Clone, Default)]
pub struct Bytes {
    /// Underlying Data
    data: Option<Arc<[u8]>>,

    /// Data Offset
    offset: usize,

    /// Data Length
    len: usize,
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate Functions for Non-Mutable Buffers
impl Bytes {
    /// Creates a non-mutable buffer.
    pub fn new(data: Option<Arc<[u8]>>, offset: usize, len: usize) -> Self {
        Self { data, offset, len }
    }
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// Partial Equality Trait Implementation for Non-Mutable Buffers
impl PartialEq for Bytes {
    fn eq(&self, rhs: &Self) -> bool {
        self[..] == rhs[..]
    }
}

/// Equality Trait Implementation for Non-Mutable Buffers
impl Eq for Bytes {}

/// Debug Trait Implementation for Non-Mutable Buffers
impl Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bytes({:?})", &self[..])
    }
}

/// De-Reference Trait Implementation for Non-Mutable Buffers
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self.data {
            None => &[],
            Some(ref buf) => &buf[self.offset..(self.offset + self.len)],
        }
    }
}

/// Buffer Trait Implementation for Non-Mutable Buffers
impl Buffer for Bytes {
    /// Creates an empty [Buffer].
    fn empty() -> Self {
        Self::default()
    }

    /// Creates a [Buffer] from a slice.
    fn from_slice(src: &[u8]) -> Self {
        let buf: Arc<[u8]> = src.into();
        Self {
            data: Some(buf),
            offset: 0,
            len: src.len(),
        }
    }

    /// Removes the first `nbytes` bytes of the target [Buffer].
    fn adjust(&mut self, nbytes: usize) {
        if nbytes > self.len {
            panic!("Adjusting past end of buffer: {} vs. {}", nbytes, self.len);
        }
        self.offset += nbytes;
        self.len -= nbytes;
    }

    /// Removes the last `nbytes` bytes of the target [Buffer].
    fn trim(&mut self, nbytes: usize) {
        if nbytes > self.len {
            panic!("Trimming past beginning of buffer: {} vs. {}", nbytes, self.len);
        }
        self.len -= nbytes;
    }
}

//==============================================================================
// Unit Tests
//==============================================================================

#[cfg(test)]
mod tests {
    use super::{
        Buffer,
        Bytes,
    };
    use std::sync::Arc;

    /// Tests for buffer adjust.
    #[test]
    fn buf_adjust() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let mut buf: Bytes = Bytes::new(Some(Arc::new(data)), 0, 4);
        buf.adjust(2);
        assert_eq!(*buf, data[2..]);
    }

    /// Tests for buffer trim.
    #[test]
    fn buf_trim() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let mut buf: Bytes = Bytes::new(Some(Arc::new(data)), 0, 4);
        buf.trim(2);
        assert_eq!(*buf, data[..2]);
    }
}
