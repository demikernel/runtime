// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use ::rand::distributions::{Distribution, Standard};

//==============================================================================
// Traits
//==============================================================================

/// Utilities Runtime
pub trait UtilsRuntime {
    /// Returns a random value supporting the [Standard] distribution.
    fn rng_gen<T>(&self) -> T
    where
        Standard: Distribution<T>;

    /// Shuffles a a mutable slice in place.
    fn rng_shuffle<T>(&self, slice: &mut [T]);
}
