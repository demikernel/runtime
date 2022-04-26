// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//==============================================================================
// Imports
//==============================================================================

use ::catwalk::{
    SchedulerFuture,
    SchedulerHandle,
};
use ::std::{
    future::Future,
    time::{
        Duration,
        Instant,
    },
};

//==============================================================================
// Traits
//==============================================================================

/// Scheduler Runtime
pub trait SchedulerRuntime {
    type WaitFuture: Future<Output = ()>;

    /// Creates a future on which one should wait a given duration.
    fn wait(&self, duration: Duration) -> Self::WaitFuture;

    /// Creates a future on which one should until a given point in time.
    fn wait_until(&self, when: Instant) -> Self::WaitFuture;

    /// Returns the current runtime clock.
    fn now(&self) -> Instant;

    /// Advances the runtime clock to some point in time.
    fn advance_clock(&self, now: Instant);

    /// Spawns a new task.
    fn spawn<F: SchedulerFuture>(&self, future: F) -> SchedulerHandle;

    /// Schedules a task for execution.
    fn schedule<F: SchedulerFuture>(&self, future: F) -> SchedulerHandle;

    /// Gets the handle of a task.
    fn get_handle(&self, key: u64) -> Option<SchedulerHandle>;

    /// Takes out a given task from the scheduler.
    fn take(&self, handle: SchedulerHandle) -> Box<dyn SchedulerFuture>;

    /// Polls the scheduler.
    fn poll(&self);
}
