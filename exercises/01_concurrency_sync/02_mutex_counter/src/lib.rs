//! # Mutex Shared State
//! # Mutex 共享状态
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//! 在本练习中，你将使用 `Arc<Mutex<T>>` 在多个线程之间安全地共享和修改数据。
//!
//! ## Concepts
//! ## 概念
//! - `Mutex<T>` mutex protects shared data
//! - `Mutex<T>` 互斥锁保护共享数据
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `Arc<T>` 原子引用计数支持跨线程共享
//! - `lock()` acquires the lock and accesses data
//! - `lock()` 获取锁并访问数据

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// 使用 `n_threads` 个线程并发地递增计数器。
/// Each thread increments the counter `count_per_thread` times.
/// 每个线程将计数器递增 `count_per_thread` 次。
/// Returns the final counter value.
/// 返回最终的计数器值。
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
/// 提示：使用 `Arc<Mutex<usize>>` 作为共享计数器。
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    // TODO: 创建初始值为 0 的 Arc<Mutex<usize>>
    // TODO: Spawn n_threads threads
    // TODO: 生成 n_threads 个线程
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: 在每个线程中，调用 lock() 并递增 count_per_thread 次
    // TODO: Join all threads, return final value
    // TODO: 等待所有线程结束，返回最终值
    //todo!()
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..n_threads {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += count_per_thread;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let num = *counter.lock().unwrap();
    num
}

/// Add elements to a shared vector concurrently using multiple threads.
/// 使用多个线程并发地向共享向量添加元素。
/// Each thread pushes its own id (0..n_threads) to the vector.
/// 每个线程将其自身的 id（0..n_threads）推入向量。
/// Returns the sorted vector.
/// 返回排序后的向量。
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
/// 提示：使用 `Arc<Mutex<Vec<usize>>>`。
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: 创建 Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: 每个线程推入其自身的 id
    // TODO: After joining all threads, sort the result and return
    // TODO: 等待所有线程结束后，对结果排序并返回
    //todo!()
    let mutex = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..n_threads {
        let mutex = mutex.clone();
        handles.push(thread::spawn(move || {
            let mut mutex = mutex.lock().unwrap();
            mutex.push(i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut queue = mutex.lock().unwrap();
    queue.sort();
    queue.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
