use std::{
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::Duration,
};

use crate::{condvar_table, private::AtomicWaitImpl};

impl AtomicWaitImpl for AtomicU32 {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        condvar_table::wait(
            self as *const _ as *const _,
            || self.load(Ordering::Acquire) == value,
            timeout,
        );
    }

    fn notify_all(&self) {
        condvar_table::notify_all(self as *const _ as *const _);
    }

    fn notify_one(&self) {
        condvar_table::notify_one(self as *const _ as *const _);
    }
}

impl AtomicWaitImpl for AtomicU64 {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        condvar_table::wait(
            self as *const _ as *const _,
            || self.load(Ordering::Acquire) == value,
            timeout,
        );
    }

    fn notify_all(&self) {
        condvar_table::notify_all(self as *const _ as *const _);
    }

    fn notify_one(&self) {
        condvar_table::notify_one(self as *const _ as *const _);
    }
}
