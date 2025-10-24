#![doc = include_str!("../README.md")]
#![cfg_attr(
    all(nightly, target_arch = "wasm32"),
    feature(stdarch_wasm_atomic_wait)
)]

use std::{
    sync::atomic::{AtomicI32, AtomicI64, AtomicU32, AtomicU64},
    time::Duration,
};

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
#[allow(unused)]
mod condvar_table;

/// A type that supports atomic waits.
pub trait AtomicWait: private::AtomicWaitImpl {
    /// If the value is `value`, wait until woken up.
    ///
    /// This function might also return spuriously,
    /// without a corresponding wake operation.
    fn wait(&self, value: Self::AtomicInner) {
        private::AtomicWaitImpl::wait_timeout(self, value, None);
    }

    /// If the value is `value`, wait until timeout elapses
    /// or notify is called.
    ///
    /// This function might also return spuriously,
    /// without a corresponding wake operation.
    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Duration) {
        private::AtomicWaitImpl::wait_timeout(self, value, Some(timeout));
    }

    /// Wake one thread that is waiting on this atomic.
    fn notify_one(&self) {
        private::AtomicWaitImpl::notify_one(self);
    }

    /// Wake all threads that are waiting on this atomic.
    fn notify_all(&self) {
        private::AtomicWaitImpl::notify_all(self);
    }
}

impl AtomicWait for AtomicU32 {}
impl AtomicWait for AtomicU64 {}
impl AtomicWait for AtomicI32 {}
impl AtomicWait for AtomicI64 {}

impl private::AtomicWaitImpl for AtomicI32 {
    type AtomicInner = i32;

    fn notify_all(&self) {
        unsafe {
            private::AtomicWaitImpl::notify_all(std::mem::transmute::<&AtomicI32, &AtomicU32>(
                self,
            ));
        }
    }

    fn notify_one(&self) {
        unsafe {
            private::AtomicWaitImpl::notify_one(std::mem::transmute::<&AtomicI32, &AtomicU32>(
                self,
            ));
        }
    }

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            private::AtomicWaitImpl::wait_timeout(
                std::mem::transmute::<&AtomicI32, &AtomicU32>(self),
                value as u32,
                timeout,
            );
        }
    }
}

impl private::AtomicWaitImpl for AtomicI64 {
    type AtomicInner = i64;

    fn notify_all(&self) {
        unsafe {
            private::AtomicWaitImpl::notify_all(std::mem::transmute::<&AtomicI64, &AtomicU64>(
                self,
            ));
        }
    }

    fn notify_one(&self) {
        unsafe {
            private::AtomicWaitImpl::notify_one(std::mem::transmute::<&AtomicI64, &AtomicU64>(
                self,
            ));
        }
    }

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            private::AtomicWaitImpl::wait_timeout(
                std::mem::transmute::<&AtomicI64, &AtomicU64>(self),
                value as u64,
                timeout,
            );
        }
    }
}

/// Private implementation details.
mod private {
    use std::time::Duration;

    /// A trait that cannot be implemented by other crates.
    pub trait AtomicWaitImpl {
        /// The underlying integer type for the atomic.
        type AtomicInner;

        /// Wake all threads that are waiting on this atomic.
        fn notify_all(&self);

        /// Wake one thread that is waiting on this atomic.
        fn notify_one(&self);

        /// If the value is `value`, wait until woken up.
        ///
        /// This function might also return spuriously,
        /// without a corresponding wake operation.
        fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>);
    }
}
