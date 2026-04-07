//! # Atomic Operations Basics
//! 
//! In this exercise, you will use atomic types to implement a lock-free thread‑safe counter.
//! 
//! ## Key Concepts
//! - `std::sync::atomic::AtomicU64`
//! - `fetch_add`, `fetch_sub`, `load`, `store` operations
//! - `compare_exchange` lock‑free primitive
//! - `Ordering` memory ordering
//! 
//! # 原子操作基础
//! 
//! 在本练习中，你将使用原子类型来实现一个无锁的线程安全计数器。
//! 
//! ## 关键概念
//! - `std::sync::atomic::AtomicU64`
//! - `fetch_add`、`fetch_sub`、`load`、`store` 操作
//! - `compare_exchange` 无锁原语
//! - `Ordering` 内存顺序

use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicCounter {
    value: AtomicU64,
}

impl AtomicCounter {
    pub const fn new(init: u64) -> Self {
        Self {
            value: AtomicU64::new(init),
        }
    }

    /// Atomically increments by 1, returns the value **before** increment.
    ///
    /// Hint: use `fetch_add` with `Ordering::Relaxed`
    ///
    /// 原子地增加 1，返回增加**之前**的值。
    ///
    /// 提示：使用 `fetch_add` 配合 `Ordering::Relaxed`
    pub fn increment(&self) -> u64 {
        // TODO
        //todo!()
        self.value.fetch_add(1, Ordering::Relaxed)
    }

    /// Atomically decrements by 1, returns the value **before** decrement.
    ///
    /// 原子地减少 1，返回减少**之前**的值。
    pub fn decrement(&self) -> u64 {
        // TODO
        //todo!()
        self.value.fetch_sub(1, Ordering::Relaxed)
    }

    /// Gets the current value.
    ///
    /// 获取当前值。
    pub fn get(&self) -> u64 {
        // TODO
        //todo!()
        self.value.load(Ordering::Relaxed)
    }

    /// Atomic CAS (Compare-And-Swap) operation.
    /// If current value equals `expected`, set to `new_val` and return Ok(expected).
    /// Otherwise return Err(actual current value).
    ///
    /// Hint: use `compare_exchange` with success ordering `Ordering::AcqRel` and failure ordering `Ordering::Acquire`
    ///
    /// 原子 CAS（比较并交换）操作。
    /// 如果当前值等于 `expected`，则设置为 `new_val` 并返回 Ok(expected)。
    /// 否则返回 Err(实际当前值)。
    ///
    /// 提示：使用 `compare_exchange`，成功顺序为 `Ordering::AcqRel`，失败顺序为 `Ordering::Acquire`
    pub fn compare_and_swap(&self, expected: u64, new_val: u64) -> Result<u64, u64> {
        // TODO
        //todo!()
        self.value.compare_exchange(expected, new_val, Ordering::AcqRel, Ordering::Acquire)
    }

    /// Multiply the value atomically using a CAS loop.
    /// Returns the value **before** multiplication.
    ///
    /// Hint: read current value in loop, compute new value, try CAS to update, retry on failure.
    ///
    /// 使用 CAS 循环原子地执行乘法操作。
    /// 返回乘法**之前**的值。
    ///
    /// 提示：在循环中读取当前值，计算新值，尝试使用 CAS 更新，失败时重试。
    pub fn fetch_multiply(&self, multiplier: u64) -> u64 {
        // TODO: CAS loop
        // loop {
        //     let current = ...
        //     let new = current * multiplier;
        //     match self.compare_and_swap(current, new) { ... }
        // }
        //
        // TODO: CAS 循环
        // loop {
        //     let current = ...
        //     let new = current * multiplier;
        //     match self.compare_and_swap(current, new) { ... }
        // }
        //todo!()
        loop{
            let current = self.get();
            let new = current * multiplier;
            match self.compare_and_swap(current, new) {
                Ok(_) => return current,
                Err(_) => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_ops() {
        let c = AtomicCounter::new(0);
        assert_eq!(c.increment(), 0);
        assert_eq!(c.increment(), 1);
        assert_eq!(c.get(), 2);
        assert_eq!(c.decrement(), 2);
        assert_eq!(c.get(), 1);
    }

    #[test]
    fn test_cas_success() {
        let c = AtomicCounter::new(10);
        assert_eq!(c.compare_and_swap(10, 20), Ok(10));
        assert_eq!(c.get(), 20);
    }

    #[test]
    fn test_cas_failure() {
        let c = AtomicCounter::new(10);
        assert_eq!(c.compare_and_swap(5, 20), Err(10));
        assert_eq!(c.get(), 10);
    }

    #[test]
    fn test_fetch_multiply() {
        let c = AtomicCounter::new(3);
        let old = c.fetch_multiply(4);
        assert_eq!(old, 3);
        assert_eq!(c.get(), 12);
    }

    #[test]
    fn test_concurrent_increment() {
        let counter = Arc::new(AtomicCounter::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    c.increment();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(), 10000);
    }
}
