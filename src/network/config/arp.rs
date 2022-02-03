// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use crate::network::types::{Ipv4Addr, MacAddress};
use ::std::{collections::HashMap, time::Duration};

//==============================================================================
// Structures
//==============================================================================

/// ARP Configuration Descriptor
#[derive(Clone, Debug)]
pub struct ArpConfig {
    /// Time to Live for ARP Cache
    cache_ttl: Duration,
    /// Timeout for ARP Requests
    request_timeout: Duration,
    /// Retry Count for ARP Requests
    retry_count: usize,
    /// Initial Values for ARP Cache
    initial_values: HashMap<Ipv4Addr, MacAddress>,
    /// Disable ARP?
    disable_arp: bool,
}

//==============================================================================
// Associate Functions
//==============================================================================

/// Associate Functions for ARP Configuration Descriptor
///
/// TODO: Create Setters for Member Fields
impl ArpConfig {
    /// Creates an ARP Configuration Descriptor.
    ///
    /// TODO: Enable Optional Parameters
    /// TODO: Rely on Setters for Member Fields
    pub fn new(
        cache_ttl: Duration,
        request_timeout: Duration,
        retry_count: usize,
        initial_values: HashMap<Ipv4Addr, MacAddress>,
        disable_arp: bool,
    ) -> Self {
        ArpConfig {
            cache_ttl,
            request_timeout,
            retry_count,
            initial_values,
            disable_arp,
        }
    }

    /// Gets the time to live for entries of the ARP Cache in the target [ArpConfig].
    pub fn get_cache_ttl(&self) -> Duration {
        self.cache_ttl
    }

    /// Gets the request timeout for ARP requests in the target [ArpConfig].
    pub fn get_request_timeout(&self) -> Duration {
        self.request_timeout
    }

    /// Gets the retry count for ARP requests in the target [ArpConfig].
    pub fn get_retry_count(&self) -> usize {
        self.retry_count
    }

    /// Gets the initial values for the ARP Cache in the target [ArpConfig].
    pub fn get_initial_values(&self) -> &HashMap<Ipv4Addr, MacAddress> {
        &self.initial_values
    }

    /// Gets the disable option of the ARP in the target [ArpConfig].
    pub fn get_disable_arp(&self) -> bool {
        self.disable_arp
    }
}

//==============================================================================
// Trait Implementations
//==============================================================================

/// Default Trait Implementation for ARP Configuration Descriptor
impl Default for ArpConfig {
    /// Creates a ARP Configuration Descriptor with the default values.
    fn default() -> Self {
        ArpConfig {
            cache_ttl: Duration::from_secs(15),
            request_timeout: Duration::from_secs(20),
            retry_count: 5,
            initial_values: HashMap::new(),
            disable_arp: false,
        }
    }
}
