Cross platform atomic wait and wake (aka futex) functionality.

This crate only supports functionality that's available on all of
Linux, Windows, and macOS. That is:

- Only the "wait", "wake one", and "wake all" operations are supported.
  (Linux supports more operations, but Windows and macOS don't.)
- No timeouts.
  (macOS doesn't have a stable/public API for timeouts.)
- The wake operations don't return the number of threads woken up.
  (Only Linux supports this.)

Supported platforms:
   Linux 2.6.22+ (32-bit atomics only),
   Linux 5.???+ (full support),
   Windows 8+, Windows Server 2012+,
   macOS 11+, iOS 14+, watchOS 7+.

## Usage

```
use std::sync::atomic::AtomicU32;
use atomic_wait::AtomicWait;

let a = AtomicU32::new(0);

a.wait(1); // If the value is 1, wait.

a.wake_one(); // Wake one waiting thread.

a.wake_all(); // Wake all waiting threads.
```

## Implementation

On Linux, this uses the `SYS_futex` syscall.
For atomics of other sizes than 32 bit,
the `SYS_futex_waitv` syscall is used too.

On Windows, this uses the `WaitOnAddress` and `WakeByAddress` APIs.

On macOS (and iOS and watchOS), this uses `libc++`, making use of the same
(ABI-stable) functions behind C++20's `atomic_wait` and `atomic_notify` functions.
