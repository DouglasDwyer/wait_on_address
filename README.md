# wait_on_address

[![Crates.io](https://img.shields.io/crates/v/wait_on_address.svg)](https://crates.io/crates/wait_on_address)
[![Docs.rs](https://docs.rs/wait_on_address/badge.svg)](https://docs.rs/wait_on_address)

Cross platform atomic wait and wake (aka futex) functionality. This crate is a fork of [`atomic-wait`](https://github.com/m-ou-se/atomic-wait), and extends the original code with the following functionality:

- Support for `AtomicI32`, `AtomicI64`, and `AtomicU64`
- Support for waiting with a timeout
- Support for `wasm32` on nightly using `std::arch`
- Polyfill for all other platforms

Natively-supported platforms:

- Windows 8+, Windows Server 2012+
- macOS 14.4+, iOS 17.4+, watchOS 10.4+
- Linux 2.6.22+ (using fallback for 64-bit futexes)
- wasm32
- All other platforms with `std` support (using fallback)

## Usage

```rust
use std::{sync::atomic::AtomicU64, time::Duration};
use ecmascript_futex::AtomicWait;

let a = AtomicU64::new(0);

a.wait(1); // If the value is 1, wait.

a.wait_timeout(2, Duration::from_millis(100));  // If the value is 2, wait at most 100 milliseconds

a.notify_one(); // Wake one waiting thread.

a.notify_all(); // Wake all waiting threads.
```

## Implementation

On Linux, this uses the `SYS_futex` syscall.

On FreeBSD, this uses the `_umtx_op` syscall.

On Windows, this uses the `WaitOnAddress` and `WakeByAddress` APIs.

On macOS (and iOS and watchOS), this uses the `os_sync_wait_on_address` and `os_sync_wake_by_address` APIs.

On wasm32 with `nightly`, this uses `memory_atomic_wait32`, `memory_atomic_wait64`, and `memory_atomic_notify` instructions.

All other platforms with `std` support fall back to a fixed-size hashmap of `Condvar`s, similar to `libstdc++`'s implementation for `std::atomic<T>`.
