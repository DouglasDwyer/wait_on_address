use core::time::Duration;

use ecmascript_atomics::{Ordering, Racy};

use crate::{condvar_table, private::ECMAScriptAtomicWaitImpl};

impl ECMAScriptAtomicWaitImpl for Racy<'_, u32> {
    type ECMAScriptAtomicInner = u32;

    fn wait_timeout(&self, value: Self::ECMAScriptAtomicInner, timeout: Option<Duration>) {
        unsafe {
            let wait_timespec = timeout.map(|x| libc::timespec {
                tv_sec: x.as_secs() as i64,
                tv_nsec: x.subsec_nanos() as i64,
            });

            libc::syscall(
                libc::SYS_futex,
                self.addr(),
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
                self.addr(),
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                i32::MAX,
            );
        };
    }

    fn notify_one(&self) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                self.addr(),
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                1i32,
            );
        };
    }
}

impl ECMAScriptAtomicWaitImpl for Racy<'_, u64> {
    type ECMAScriptAtomicInner = u64;

    fn wait_timeout(&self, value: Self::ECMAScriptAtomicInner, timeout: Option<Duration>) {
        condvar_table::wait(
            self.addr(),
            || self.load(Ordering::SeqCst) == value,
            timeout,
        );
    }

    fn notify_all(&self) {
        condvar_table::notify_all(self.addr());
    }

    fn notify_one(&self) {
        condvar_table::notify_one(self.addr());
    }
}
