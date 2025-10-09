use std::{
    sync::atomic::{AtomicU32, AtomicU64},
    time::Duration,
};
use windows_sys::Win32::System::Threading::{
    INFINITE, WaitOnAddress, WakeByAddressAll, WakeByAddressSingle,
};

use crate::private::AtomicWaitImpl;

impl AtomicWaitImpl for AtomicU32 {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            WaitOnAddress(
                self as *const _ as *const _,
                &value as *const _ as *const _,
                size_of::<Self>(),
                timeout
                    .map(|x| x.as_millis().max(u64::MAX as u128) as u32)
                    .unwrap_or(INFINITE),
            );
        }
    }

    fn notify_all(&self) {
        unsafe { WakeByAddressAll(self as *const _ as *const _) };
    }

    fn notify_one(&self) {
        unsafe { WakeByAddressSingle(self as *const _ as *const _) };
    }
}

impl AtomicWaitImpl for AtomicU64 {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            WaitOnAddress(
                self as *const _ as *const _,
                &value as *const _ as *const _,
                size_of::<Self>(),
                timeout
                    .map(|x| x.as_millis().max(u64::MAX as u128) as u32)
                    .unwrap_or(INFINITE),
            );
        }
    }

    fn notify_all(&self) {
        unsafe { WakeByAddressAll(self as *const _ as *const _) };
    }

    fn notify_one(&self) {
        unsafe { WakeByAddressSingle(self as *const _ as *const _) };
    }
}
