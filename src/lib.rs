#![doc = include_str!("../README.md")]
#![cfg_attr(
    all(nightly, target_arch = "wasm32"),
    feature(stdarch_wasm_atomic_wait)
)]

use core::time::Duration;

use ecmascript_atomics::Racy;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "linux.rs"]
mod platform;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
#[path = "macos.rs"]
mod platform;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

#[cfg(target_os = "freebsd")]
#[path = "freebsd.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "wasm32.rs"]
mod platform;

#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "ios",
    target_os = "watchos",
    windows
)))]
#[path = "fallback.rs"]
mod platform;

/// A table of OS synchronization primitives for manually
/// implementing futex functionality on unsupported platforms.
#[cfg(not(any(
    target_os = "freebsd",
    target_os = "macos",
    target_os = "ios",
    target_os = "watchos",
    windows
)))]
mod condvar_table;

/// A type that supports atomic waits.
pub trait ECMAScriptAtomicWait: private::ECMAScriptAtomicWaitImpl {
    /// If the value is `value`, wait until woken up.
    ///
    /// This function might also return spuriously,
    /// without a corresponding wake operation.
    fn wait(&self, value: Self::ECMAScriptAtomicInner) {
        private::ECMAScriptAtomicWaitImpl::wait_timeout(self, value, None);
    }

    /// If the value is `value`, wait until timeout elapses
    /// or notify is called.
    ///
    /// This function might also return spuriously,
    /// without a corresponding wake operation.
    fn wait_timeout(&self, value: Self::ECMAScriptAtomicInner, timeout: Duration) {
        private::ECMAScriptAtomicWaitImpl::wait_timeout(self, value, Some(timeout));
    }

    /// Wake one thread that is waiting on this atomic.
    fn notify_one(&self) {
        private::ECMAScriptAtomicWaitImpl::notify_one(self);
    }

    /// Wake all threads that are waiting on this atomic.
    fn notify_all(&self) {
        private::ECMAScriptAtomicWaitImpl::notify_all(self);
    }
}

impl ECMAScriptAtomicWait for Racy<'_, u32> {}
impl ECMAScriptAtomicWait for Racy<'_, u64> {}

/// Private implementation details.
mod private {
    use core::time::Duration;

    /// A trait that cannot be implemented by other crates.
    pub trait ECMAScriptAtomicWaitImpl {
        /// The underlying integer type for the atomic.
        type ECMAScriptAtomicInner;

        /// Wake all threads that are waiting on this atomic.
        fn notify_all(&self);

        /// Wake one thread that is waiting on this atomic.
        fn notify_one(&self);

        /// If the value is `value`, wait until woken up.
        ///
        /// This function might also return spuriously,
        /// without a corresponding wake operation.
        fn wait_timeout(&self, value: Self::ECMAScriptAtomicInner, timeout: Option<Duration>);
    }
}
