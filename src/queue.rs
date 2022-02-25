// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use crate::QDesc;
use ::slab::Slab;

//==============================================================================
// Enumerations
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
    pub fn alloc(&mut self, qtype: IoQueueType) -> QDesc {
        let ix: usize = self.table.insert(qtype);
        QDesc::from(ix)
    }

    /// Gets the file associated with an IO queue descriptor.
    pub fn get(&self, qd: QDesc) -> Option<IoQueueType> {
        if !self.table.contains(qd.into()) {
            return None;
        }

        self.table.get(qd.into()).cloned()
    }

    /// Releases an entry in the target [IoQueueTable].
    pub fn free(&mut self, qd: QDesc) -> Option<IoQueueType> {
        if !self.table.contains(qd.into()) {
            return None;
        }

        Some(self.table.remove(qd.into()))
    }
}

//==============================================================================
// Unit Tests
//==============================================================================

#[cfg(test)]
mod tests {
    use super::{IoQueueTable, IoQueueType};
    use crate::QDesc;
    use ::test::{black_box, Bencher};

    #[bench]
    fn bench_alloc_free(b: &mut Bencher) {
        let mut ioqueue_table: IoQueueTable = IoQueueTable::new();

        b.iter(|| {
            let qd: QDesc = ioqueue_table.alloc(IoQueueType::TcpSocket);
            black_box(qd);
            let qtype: Option<IoQueueType> = ioqueue_table.free(qd);
            black_box(qtype);
        });
    }
}
