use std::{hint::spin_loop, time::Duration};

use ecmascript_atomics::{Ordering, Racy};

use crate::private::AtomicWaitImpl;

/// Whether this thread is allowed to block and use synchronization primitives.
#[inline(always)]
fn can_block() -> bool {
    thread_local! {
        static CAN_BLOCK: bool = web_sys::window().is_none();
    }

    CAN_BLOCK.with(|x| *x)
}

#[cfg(not(nightly))]
impl AtomicWaitImpl for Racy<'_, u32> {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        if can_block() {
            crate::condvar_table::wait(
                self.addr(),
                || self.load(std::sync::atomic::Ordering::Acquire) == value,
                timeout,
            );
        } else {
            spin_loop();
        }
    }

    fn notify_all(&self) {
        crate::condvar_table::notify_all(self.addr());
    }

    fn notify_one(&self) {
        crate::condvar_table::notify_one(self.addr());
    }
}

#[cfg(not(nightly))]
impl AtomicWaitImpl for Racy<'_, u64> {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        if can_block() {
            crate::condvar_table::wait(
                self.addr(),
                || self.load(std::sync::atomic::Ordering::Acquire) == value,
                timeout,
            );
        } else {
            spin_loop();
        }
    }

    fn notify_all(&self) {
        crate::condvar_table::notify_all(self.addr());
    }

    fn notify_one(&self) {
        crate::condvar_table::notify_one(self.addr());
    }
}

#[cfg(nightly)]
impl AtomicWaitImpl for Racy<'_, u32> {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            if can_block() {
                std::arch::wasm32::memory_atomic_wait32(
                    self.addr(),
                    value as i32,
                    timeout
                        .map(|x| x.as_nanos().min(i64::MAX as u128) as i64)
                        .unwrap_or(i64::MAX),
                );
            } else {
                spin_loop();
            }
        }
    }

    fn notify_all(&self) {
        unsafe {
            std::arch::wasm32::memory_atomic_notify(self.addr(), u32::MAX);
        };
    }

    fn notify_one(&self) {
        unsafe {
            std::arch::wasm32::memory_atomic_notify(self.addr(), 1);
        };
    }
}

#[cfg(nightly)]
impl AtomicWaitImpl for Racy<'_, u64> {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            if can_block() {
                std::arch::wasm32::memory_atomic_wait64(
                    self.addr(),
                    value as i64,
                    timeout
                        .map(|x| x.as_nanos().min(i64::MAX as u128) as i64)
                        .unwrap_or(i64::MAX),
                );
            } else {
                spin_loop();
            }
        }
    }

    fn notify_all(&self) {
        unsafe {
            std::arch::wasm32::memory_atomic_notify(self.addr(), u32::MAX);
        };
    }

    fn notify_one(&self) {
        unsafe {
            std::arch::wasm32::memory_atomic_notify(self.addr(), 1);
        };
    }
}
