// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Structures
//==============================================================================

/// IO Queue Type
#[repr(u32)]
#[derive(FromPrimitive, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum QType {
    UdpSocket = 0x0001,
    TcpSocket = 0x0002,
}

impl From<QType> for u32 {
    fn from(qtoken: QType) -> Self {
        qtoken.into()
    }
}
