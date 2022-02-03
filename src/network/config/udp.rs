// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Constants & Structures
//==============================================================================

/// UDP Configuration Descriptor
#[derive(Clone, Debug)]
pub struct UdpConfig {
    /// Offload Checksum to Hardware When Receiving?
    rx_checksum: bool,
    /// Offload Checksum to Hardware When Sending?
    tx_checksum: bool,
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate functions for UDP Configuration Descriptor
///
/// TODO: Create Setters for Member Fields
impl UdpConfig {
    /// Creates a UDP Configuration Descriptor.
    ///
    /// TODO: Enable Optional Parameters
    /// TODO: Rely on Setters for Member Fields
    pub fn new(rx_checksum: bool, tx_checksum: bool) -> Self {
        Self {
            rx_checksum,
            tx_checksum,
        }
    }

    /// Gets the RX hardware checksum offload option in the target [UdpConfig].
    pub fn get_rx_checksum(&self) -> bool {
        self.rx_checksum
    }

    /// Gets the XX hardware checksum offload option in the target [UdpConfig].
    pub fn get_tx_checksum(&self) -> bool {
        self.tx_checksum
    }
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// Default Trait Implementation for UDP Configuration Descriptor
impl Default for UdpConfig {
    /// Creates a UDP Configuration Descriptor with the default values.
    fn default() -> Self {
        UdpConfig {
            rx_checksum: false,
            tx_checksum: false,
        }
    }
}

//==============================================================================
// Unit Tests
//==============================================================================

#[cfg(test)]
mod tests {
    use super::UdpConfig;

    /// Tests instantiations flavors for [UdpConfig].
    #[test]
    fn test_udp_options() {
        //Default options.
        let options_default = UdpConfig::default();
        assert!(!options_default.get_rx_checksum());
        assert!(!options_default.get_tx_checksum());

        // Custom options.
        let options_custom = UdpConfig::new(true, true);
        assert!(options_custom.get_rx_checksum());
        assert!(options_custom.get_tx_checksum());
    }
}
