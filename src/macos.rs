//! 'os_sync_wait_on_address' and its equivalents are only available on macOS 14.4+ /
//! iOS 17.4+ / watchOS 10.4+.
//! To stay compatible with earlier OS versions we resolve the symbols at runtime
//! with 'dlsym' (cached once) and fall back to the portable 'condvar_table'
//! implementation when they are unavailable.

use std::{
    ffi::c_void,
    sync::{
        OnceLock,
        atomic::{AtomicU32, AtomicU64, Ordering},
    },
    time::Duration,
};

use crate::{condvar_table, private::AtomicWaitImpl};

type WaitFn = unsafe extern "C" fn(*mut c_void, u64, usize, u32) -> libc::c_int;
type WaitTimeoutFn =
    unsafe extern "C" fn(*mut c_void, u64, usize, u32, libc::clockid_t, u64) -> libc::c_int;
type WakeFn = unsafe extern "C" fn(*mut c_void, usize, u32) -> libc::c_int;

/// The macOS 14.4+ futex functions, resolved at runtime.
struct FutexFns {
    wait: WaitFn,
    wait_timeout: WaitTimeoutFn,
    wake_all: WakeFn,
    wake_any: WakeFn,
}

/// Returns the futex functions if the running OS provides them, or `None` on
/// macOS version < 14.4 (and equivalent). The lookup is performed once and cached.
fn futex_fns() -> Option<&'static FutexFns> {
    static CACHE: OnceLock<Option<FutexFns>> = OnceLock::new();
    CACHE
        .get_or_init(|| unsafe {
            let wait = libc::dlsym(libc::RTLD_DEFAULT, c"os_sync_wait_on_address".as_ptr());
            let wait_timeout = libc::dlsym(
                libc::RTLD_DEFAULT,
                c"os_sync_wait_on_address_with_timeout".as_ptr(),
            );
            let wake_all = libc::dlsym(libc::RTLD_DEFAULT, c"os_sync_wake_by_address_all".as_ptr());
            let wake_any = libc::dlsym(libc::RTLD_DEFAULT, c"os_sync_wake_by_address_any".as_ptr());

            if wait.is_null() || wait_timeout.is_null() || wake_all.is_null() || wake_any.is_null()
            {
                return None;
            }

            Some(FutexFns {
                wait: std::mem::transmute::<*mut c_void, WaitFn>(wait),
                wait_timeout: std::mem::transmute::<*mut c_void, WaitTimeoutFn>(wait_timeout),
                wake_all: std::mem::transmute::<*mut c_void, WakeFn>(wake_all),
                wake_any: std::mem::transmute::<*mut c_void, WakeFn>(wake_any),
            })
        })
        .as_ref()
}

impl AtomicWaitImpl for AtomicU32 {
    type AtomicInner = u32;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        if let Some(fns) = futex_fns() {
            unsafe {
                if let Some(time) = timeout {
                    (fns.wait_timeout)(
                        self as *const _ as *mut _,
                        value as u64,
                        size_of::<Self>(),
                        libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                        libc::CLOCK_MONOTONIC,
                        time.as_nanos().min(u64::MAX as u128) as u64,
                    );
                } else {
                    (fns.wait)(
                        self as *const _ as *mut _,
                        value as u64,
                        size_of::<Self>(),
                        libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                    );
                }
            }
        } else {
            condvar_table::wait(
                self as *const _ as *const _,
                || self.load(Ordering::Acquire) == value,
                timeout,
            );
        }
    }

    fn notify_all(&self) {
        if let Some(fns) = futex_fns() {
            unsafe {
                (fns.wake_all)(
                    self as *const _ as *mut _,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
                );
            };
        } else {
            condvar_table::notify_all(self as *const _ as *const _);
        }
    }

    fn notify_one(&self) {
        if let Some(fns) = futex_fns() {
            unsafe {
                (fns.wake_any)(
                    self as *const _ as *mut _,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
                );
            };
        } else {
            condvar_table::notify_one(self as *const _ as *const _);
        }
    }
}

impl AtomicWaitImpl for AtomicU64 {
    type AtomicInner = u64;

    fn wait_timeout(&self, value: Self::AtomicInner, timeout: Option<Duration>) {
        if let Some(fns) = futex_fns() {
            unsafe {
                if let Some(time) = timeout {
                    (fns.wait_timeout)(
                        self as *const _ as *mut _,
                        value,
                        size_of::<Self>(),
                        libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                        libc::CLOCK_MONOTONIC,
                        time.as_nanos().min(u64::MAX as u128) as u64,
                    );
                } else {
                    (fns.wait)(
                        self as *const _ as *mut _,
                        value,
                        size_of::<Self>(),
                        libc::OS_SYNC_WAIT_ON_ADDRESS_NONE,
                    );
                }
            }
        } else {
            condvar_table::wait(
                self as *const _ as *const _,
                || self.load(Ordering::Acquire) == value,
                timeout,
            );
        }
    }

    fn notify_all(&self) {
        if let Some(fns) = futex_fns() {
            unsafe {
                (fns.wake_all)(
                    self as *const _ as *mut _,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
                );
            };
        } else {
            condvar_table::notify_all(self as *const _ as *const _);
        }
    }

    fn notify_one(&self) {
        if let Some(fns) = futex_fns() {
            unsafe {
                (fns.wake_any)(
                    self as *const _ as *mut _,
                    size_of::<Self>(),
                    libc::OS_SYNC_WAKE_BY_ADDRESS_NONE,
                );
            };
        } else {
            condvar_table::notify_one(self as *const _ as *const _);
        }
    }
}
