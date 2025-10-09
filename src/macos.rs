use std::{
    sync::atomic::{AtomicU32, AtomicU64},
    time::Duration,
};

use crate::private::AtomicWaitImpl;

impl AtomicWaitImpl for AtomicU32 {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            if let Some(time) = timeout {
                libc::os_sync_wait_on_address_with_timeout(
                    self as *const _ as *mut _,
                    value as u64,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                    libc::CLOCK_MONOTONIC,
                    time.as_nanos().min(u64::MAX as u128) as u64,
                );
            } else {
                libc::os_sync_wait_on_address(
                    self as *const _ as *mut _,
                    value as u64,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                );
            }
        }
    }

    fn notify_all(&self) {
        unsafe {
            libc::os_sync_wake_by_address_all(
                self as *const _ as *mut _,
                size_of::<Self>(),
                libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::os_sync_wake_by_address_any(
                self as *const _ as *mut _,
                size_of::<Self>(),
                libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
            );
        };
    }
}

impl AtomicWaitImpl for AtomicU64 {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            if let Some(time) = timeout {
                libc::os_sync_wait_on_address_with_timeout(
                    self as *const _ as *mut _,
                    value,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                    libc::CLOCK_MONOTONIC,
                    time.as_nanos().min(u64::MAX as u128) as u64,
                );
            } else {
                libc::os_sync_wait_on_address(
                    self as *const _ as *mut _,
                    value,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                );
            }
        }
    }

    fn notify_all(&self) {
        unsafe {
            libc::os_sync_wake_by_address_all(
                self as *const _ as *mut _,
                size_of::<Self>(),
                libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::os_sync_wake_by_address_any(
                self as *const _ as *mut _,
                size_of::<Self>(),
                libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
            );
        };
    }
}
