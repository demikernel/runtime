// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use slab::Slab;

//==============================================================================
// Constants & Structures
//==============================================================================

/// IO Queue Types
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoQueueType {
    TcpSocket,
    UdpSocket,
}

/// IO Queue Descriptor
#[derive(From, Into, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct IoQueueDescriptor(usize);

/// IO Queue Table
pub struct IoQueueTable {
    table: Slab<IoQueueType>,
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// [From<IoQueueDescriptor>] trait for [i32]
impl From<IoQueueDescriptor> for i32 {
    fn from(val: IoQueueDescriptor) -> Self {
        val.0 as i32
    }
}

/// [From<i32>] trait for [IoQueueDescriptor]
impl From<i32> for IoQueueDescriptor {
    fn from(val: i32) -> Self {
        IoQueueDescriptor(val as usize)
    }
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate functions for [IO QueueTable].
impl IoQueueTable {
    /// Creates an IO queue table.
    pub fn new() -> Self {
        Self { table: Slab::new() }
    }

    /// Allocates a new entry in the target IO queue descriptor table.
    pub fn alloc(&mut self, file: IoQueueType) -> IoQueueDescriptor {
        let ix = self.table.insert(file);
        IoQueueDescriptor(ix)
    }

    /// Gets the file associated with an IO queue descriptor.
    pub fn get(&self, fd: IoQueueDescriptor) -> Option<IoQueueType> {
        if !self.table.contains(fd.into()) {
            return None;
        }

        self.table.get(fd.into()).cloned()
    }

    /// Releases an entry in the target IO queue descriptor table.
    pub fn free(&mut self, fd: IoQueueDescriptor) -> Option<IoQueueType> {
        if !self.table.contains(fd.into()) {
            return None;
        }

        let file = self.table.remove(fd.into());

        Some(file)
    }
}
