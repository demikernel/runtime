// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use crate::{
    fail::Fail,
    network::types::{Ipv4Addr, Port16},
    queue::IoQueueDescriptor,
};

//==============================================================================
// Enumerations
//==============================================================================

/// Result for IO Queue Operations
pub enum QResult {
    Connect,
    Accept(IoQueueDescriptor),
    Push,
    PushTo,
    Pop(Option<(Ipv4Addr, Port16)>, Vec<u8>),
    Failed(Fail),
}
