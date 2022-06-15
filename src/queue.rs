// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use slab::Slab;

//==============================================================================
// Structures
//==============================================================================

/// IO Queue Types
///
/// TODO: we should drop this and make queue type specialization be outsourced.
#[deprecated]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoQueueType {
    TcpSocket,
    UdpSocket,
}

//==============================================================================
// Structures
//==============================================================================

/// IO Queue Descriptor
#[derive(From, Into, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct IoQueueDescriptor(usize);

/// IO Queue Table
pub struct IoQueueTable {
    table: Slab<IoQueueType>,
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate Functions for IO Queue Tables
impl IoQueueTable {
    /// Creates an IO queue table.
    pub fn new() -> Self {
        Self { table: Slab::new() }
    }

    /// Allocates a new entry in the target [IoQueueTable].
    pub fn alloc(&mut self, qtype: IoQueueType) -> IoQueueDescriptor {
        let ix: usize = self.table.insert(qtype);
        IoQueueDescriptor(ix)
    }

    /// Gets the file associated with an IO queue descriptor.
    pub fn get(&self, qd: IoQueueDescriptor) -> Option<IoQueueType> {
        if !self.table.contains(qd.clone().into()) {
            return None;
        }

        self.table.get(qd.into()).cloned()
    }

    /// Releases an entry in the target [IoQueueTable].
    pub fn free(&mut self, qd: IoQueueDescriptor) -> Option<IoQueueType> {
        if !self.table.contains(qd.clone().into()) {
            return None;
        }

        Some(self.table.remove(qd.into()))
    }
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// Convert Trait Implementation for Signed 32-bit Integers
impl From<IoQueueDescriptor> for i32 {
    fn from(val: IoQueueDescriptor) -> Self {
        val.0 as i32
    }
}

/// Convert Trait Implementation for IO Queue Descriptors
impl From<i32> for IoQueueDescriptor {
    fn from(val: i32) -> Self {
        IoQueueDescriptor(val as usize)
    }
}

//==============================================================================
// Unit Tests
//==============================================================================

#[cfg(test)]
mod tests {
    use super::{IoQueueDescriptor, IoQueueTable, IoQueueType};
    use ::test::{black_box, Bencher};

    #[bench]
    fn bench_alloc_free(b: &mut Bencher) {
        let mut ioqueue_table: IoQueueTable = IoQueueTable::new();

        b.iter(|| {
            let qd: IoQueueDescriptor = ioqueue_table.alloc(IoQueueType::TcpSocket);
            black_box(qd);
            let qtype: Option<IoQueueType> = ioqueue_table.free(qd);
            black_box(qtype);
        });
    }
}
