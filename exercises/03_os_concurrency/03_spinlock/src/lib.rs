//! # Spin Lock
//! 
//! In this exercise, you will implement a basic spin lock.
//! Spin locks are one of the most fundamental synchronization primitives in OS kernels.
//! 
//! ## Key Concepts
//! - Spin locks use busy-waiting to acquire the lock
//! - `AtomicBool`'s `compare_exchange` to implement lock acquisition
//! - `core::hint::spin_loop` to reduce CPU power consumption
//! - `UnsafeCell` provides interior mutability
//! 
//! # 自旋锁
//! 
//! 在本练习中，你将实现一个基本的自旋锁。
//! 自旋锁是操作系统内核中最基础的同步原语之一。
//! 
//! ## 关键概念
//! - 自旋锁使用忙等待来获取锁
//! - 使用 `AtomicBool` 的 `compare_exchange` 实现锁获取
//! - 使用 `core::hint::spin_loop` 减少 CPU 功耗
//! - `UnsafeCell` 提供内部可变性

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

/// Basic spin lock
/// 基本的自旋锁
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire lock, returning a mutable reference to inner data.
    ///
    /// TODO: Use compare_exchange to spin until lock is acquired
    /// 1. In a loop, try to change locked from false to true
    /// 2. Success uses Acquire ordering, failure uses Relaxed
    /// 3. On failure call `core::hint::spin_loop()` to hint CPU
    /// 4. On success return `&mut *self.data.get()`
    ///
    /// # Safety
    /// Caller must ensure `unlock` is called after using the data.
    ///
    /// 获取锁，返回内部数据的可变引用。
    ///
    /// TODO: 使用 compare_exchange 自旋直到获取锁
    /// 1. 在循环中，尝试将 locked 从 false 改为 true
    /// 2. 成功使用 Acquire 顺序，失败使用 Relaxed
    /// 3. 失败时调用 `core::hint::spin_loop()` 提示 CPU
    /// 4. 成功时返回 `&mut *self.data.get()`
    ///
    /// # 安全性
    /// 调用者必须确保在使用数据后调用 `unlock`。
    pub fn lock(&self) -> &mut T {
        // TODO
        //todo!()
        loop{
            if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                unsafe { return &mut *self.data.get() }
            } else {
                core::hint::spin_loop();
            }
        }
    }

    /// Release lock.
    /// TODO: Set locked to false (using Release ordering)
    /// 释放锁。
    /// TODO: 将 locked 设置为 false（使用 Release 顺序）
    pub fn unlock(&self) {
        // TODO
        //todo!()
        self.locked.store(false, Ordering::Release);
    }

    /// Try to acquire lock without spinning.
    /// Returns Some(&mut T) on success, None if lock is busy.
    ///
    /// 尝试获取锁而不自旋。
    /// 成功返回 Some(&mut T)，如果锁忙则返回 None。
    pub fn try_lock(&self) -> Option<&mut T> {
        // TODO: Single compare_exchange attempt
        // TODO: 单次 compare_exchange 尝试
        //todo!()
        match self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Some(unsafe { 
                &mut *self.data.get() }),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_basic_lock_unlock() {
        let lock = SpinLock::new(0u32);
        {
            let data = lock.lock();
            *data = 42;
            lock.unlock();
        }
        let data = lock.lock();
        assert_eq!(*data, 42);
        lock.unlock();
    }

    #[test]
    fn test_try_lock() {
        let lock = SpinLock::new(0u32);
        assert!(lock.try_lock().is_some());
        lock.unlock();
    }

    #[test]
    fn test_concurrent_counter() {
        let lock = Arc::new(SpinLock::new(0u64));
        let mut handles = vec![];

        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    let data = l.lock();
                    *data += 1;
                    l.unlock();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let data = lock.lock();
        assert_eq!(*data, 10000);
        lock.unlock();
    }

    #[test]
    fn test_lock_protects_data() {
        let lock = Arc::new(SpinLock::new(Vec::new()));
        let mut handles = vec![];

        for i in 0..5 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let data = l.lock();
                data.push(i);
                l.unlock();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let data = lock.lock();
        let mut sorted = data.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
        lock.unlock();
    }
}
