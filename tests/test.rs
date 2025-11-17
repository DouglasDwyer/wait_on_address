use ecmascript_futex::AtomicWait;
use std::{
    sync::atomic::{AtomicU32, Ordering::Relaxed},
    thread::sleep,
    time::{Duration, Instant},
};

#[test]
fn wake_nothing() {
    let a = AtomicU32::new(0);
    a.notify_one();
    a.notify_all();
}

#[test]
fn wait_unexpected() {
    let t = Instant::now();
    let a = AtomicU32::new(0);
    a.wait(1);
    assert!(t.elapsed().as_millis() < 100);
}

#[test]
fn wait_wake() {
    let t = Instant::now();
    let a = AtomicU32::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            sleep(Duration::from_millis(100));
            a.store(1, Relaxed);
            a.notify_one();
        });
        while a.load(Relaxed) == 0 {
            a.wait(0);
        }
        assert_eq!(a.load(Relaxed), 1);
        assert!((90..400).contains(&t.elapsed().as_millis()));
    });
}

#[test]
fn wait_timeout() {
    let a = AtomicU32::new(0);
    a.wait_timeout(0, Duration::from_millis(1));
}

#[test]
fn stress_many_waiters_notify_all() {
    use std::sync::Arc;
    let a = Arc::new(AtomicU32::new(0));
    let woke = Arc::new(AtomicU32::new(0));

    let threads = 64;
    std::thread::scope(|s| {
        for _ in 0..threads {
            let a = a.clone();
            let woke = woke.clone();
            s.spawn(move || {
                while a.load(Relaxed) == 0 {
                    a.wait(0);
                }
                woke.fetch_add(1, Relaxed);
            });
        }

        // Give threads time to start waiting
        sleep(Duration::from_millis(50));
        a.store(1, Relaxed);
        a.notify_all();
    });

    assert_eq!(woke.load(Relaxed), threads);
}

#[test]
fn stress_ping_pong_many_iters() {
    use std::sync::Arc;
    let state = Arc::new(AtomicU32::new(0));
    let iters = 5_000u32;

    std::thread::scope(|s| {
        let state_c = state.clone();
        s.spawn(move || {
            // Consumer: wait for 1, reset to 0, and notify producer.
            for _ in 0..iters {
                while state_c.load(Relaxed) != 1 {
                    // Wait while the state is 0; use a short timeout to be resilient to spurious wakes.
                    state_c.wait_timeout(0, Duration::from_millis(10));
                }
                state_c.store(0, Relaxed);
                state_c.notify_one();
            }
        });

        // Producer: set to 1, notify consumer, then wait until it resets to 0.
        for _ in 0..iters {
            state.store(1, Relaxed);
            state.notify_one();
            while state.load(Relaxed) != 0 {
                state.wait_timeout(1, Duration::from_millis(10));
            }
        }
    });

    // Final state should be 0 after a complete ping-pong.
    assert_eq!(state.load(Relaxed), 0);
}
