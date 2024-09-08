use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Mutex {
            locked: AtomicBool::new(UNLOCKED),
            data: UnsafeCell::new(data),
        }
    }

    fn jp_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // spin lock is implemented here
        // while self.locked.load(Ordering::Relaxed) != UNLOCKED {
        //     // keep spinning until lock is release
        // }

        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // keep spinning until lock is release
        }

        self.locked.store(LOCKED, Ordering::Relaxed); // acquire lock
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed); // release lock
        res
    }
}

pub fn run_mutex() {
    let mutex: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles = (0..10)
        .map(|_| {
            std::thread::spawn(move || {
                for _ in 0..1_000_000 {
                    mutex.jp_lock(|data| {
                        *data += 1;
                    })
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", mutex.jp_lock(|data| *data));
    assert!(mutex.jp_lock(|data| *data == 10 * 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutex::Mutex;
    use std::sync::atomic::AtomicUsize;
    use std::thread::spawn;

    #[test]
    fn test_mutex() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let t1 = spawn(move || {
            let r1 = y.load(Ordering::Relaxed);
            x.store(r1, Ordering::Relaxed);
            r1
        });

        let t2 = spawn(move || {
            let r2 = x.load(Ordering::Relaxed);
            y.store(42, Ordering::Relaxed);
            r2
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
