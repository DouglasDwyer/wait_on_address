use std::{
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::Duration,
};

use crate::{condvar_table, private::AtomicWaitImpl};

impl AtomicWaitImpl for AtomicU32 {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            let wait_timespec = timeout.map(|x| libc::timespec {
                tv_sec: x.as_secs() as i64,
                tv_nsec: x.subsec_nanos() as i64,
            });

            libc::syscall(
                libc::SYS_futex,
                self as *const _,
                libc::FUTEX_WAIT | libc::FUTEX_PRIVATE_FLAG,
                value,
                wait_timespec
                    .as_ref()
                    .map(|x| x as *const _)
                    .unwrap_or(std::ptr::null()),
            );
        }
    }

    fn notify_all(&self) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                self as *const _,
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                i32::MAX,
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                self as *const _,
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                1i32,
            );
        };
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
