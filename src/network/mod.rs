// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use crate::{
    network::{
        config::{
            ArpConfig,
            TcpConfig,
            UdpConfig,
        },
        consts::RECEIVE_BATCH_SIZE,
        types::{
            Ipv4Addr,
            MacAddress,
        },
    },
    MemoryRuntime,
};
use ::arrayvec::ArrayVec;

//==============================================================================
// Exports
//==============================================================================

pub mod config;
pub mod consts;
pub mod types;

//==============================================================================
// Traits
//==============================================================================

/// Packet Buffer
pub trait PacketBuf<T>: Sized {
    /// Returns the header size of the target [PacketBuf].
    fn header_size(&self) -> usize;
    /// Writes the header of the target [PacketBuf] into a slice.
    fn write_header(&self, buf: &mut [u8]);
    /// Returns the body size of the target [PacketBuf].
    fn body_size(&self) -> usize;
    /// Consumes and returns the body of the target [PacketBuf].
    fn take_body(self) -> Option<T>;
}

/// Network Runtime
pub trait NetworkRuntime: MemoryRuntime {
    /// Transmits a single [PacketBuf].
    fn transmit(&self, pkt: impl PacketBuf<Self::Buf>);

    /// Receives a batch of [PacketBuf].
    fn receive(&self) -> ArrayVec<Self::Buf, RECEIVE_BATCH_SIZE>;

    /// Returns the [MacAddress] of the local endpoint.
    fn local_link_addr(&self) -> MacAddress;

    /// Returns the [Ipv4Addr] of the local endpoint.
    fn local_ipv4_addr(&self) -> Ipv4Addr;

    /// Returns the ARP Configuration Descriptor of the target [NetworkRuntime].
    fn arp_options(&self) -> ArpConfig;

    /// Returns the TCP Configuration Descriptor of the target [NetworkRuntime].
    fn tcp_options(&self) -> TcpConfig;

    /// Returns the UDP Configuration Descriptor of the target [NetworkRuntime].
    fn udp_options(&self) -> UdpConfig;
}
