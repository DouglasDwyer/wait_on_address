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
                let wait_timespec = libc::_umtx_time {
                    _clockid: libc::CLOCK_MONOTONIC as u32,
                    _flags: libc::UMTX_ABSTIME,
                    _timeout: libc::timespec {
                        tv_sec: time.as_secs() as i64,
                        tv_nsec: time.subsec_nanos() as i64,
                    },
                };

                libc::_umtx_op(
                    self as *const _ as *mut _,
                    libc::UMTX_OP_WAIT_UINT_PRIVATE,
                    value as u64,
                    size_of::<libc::_umtx_time>() as *mut _,
                    &wait_timespec as *const _ as *mut _,
                );
            } else {
                libc::_umtx_op(
                    self as *const _ as *mut _,
                    libc::UMTX_OP_WAIT_UINT_PRIVATE,
                    value as u64,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            }
        };
    }

    fn notify_all(&self) {
        unsafe {
            libc::_umtx_op(
                self as *const _ as *mut _,
                libc::UMTX_OP_WAKE_PRIVATE,
                i32::MAX as libc::c_ulong,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::_umtx_op(
                self as *const _ as *mut _,
                libc::UMTX_OP_WAKE_PRIVATE,
                1 as libc::c_ulong,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        };
    }
}

impl AtomicWaitImpl for AtomicU64 {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        unsafe {
            if let Some(time) = timeout {
                let wait_timespec = libc::_umtx_time {
                    _clockid: libc::CLOCK_MONOTONIC as u32,
                    _flags: libc::UMTX_ABSTIME,
                    _timeout: libc::timespec {
                        tv_sec: time.as_secs() as i64,
                        tv_nsec: time.subsec_nanos() as i64,
                    },
                };

                libc::_umtx_op(
                    self as *const _ as *mut _,
                    libc::UMTX_OP_WAIT,
                    value,
                    size_of::<libc::_umtx_time>() as *mut _,
                    &wait_timespec as *const _ as *mut _,
                );
            } else {
                libc::_umtx_op(
                    self as *const _ as *mut _,
                    libc::UMTX_OP_WAIT,
                    value,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            }
        };
    }

    fn notify_all(&self) {
        unsafe {
            libc::_umtx_op(
                self as *const _ as *mut _,
                libc::UMTX_OP_WAKE_PRIVATE,
                i32::MAX as libc::c_ulong,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::_umtx_op(
                self as *const _ as *mut _,
                libc::UMTX_OP_WAKE_PRIVATE,
                1 as libc::c_ulong,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        };
    }
}
