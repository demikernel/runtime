// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use ::std::net::IpAddr;

use crate::{
    fail::Fail,
    network::types::Port16,
    QDesc,
};

//==============================================================================
// Enumerations
//==============================================================================

/// Result for IO Queue Operations
pub enum QResult {
    Connect,
    Accept(QDesc),
    Push,
    PushTo,
    Pop(Option<(IpAddr, Port16)>, Vec<u8>),
    Failed(Fail),
}
