//! # Read-Write Lock (Writer-Priority)
//!
//! In this exercise, you will implement a **writer-priority** read-write lock from scratch using atomics.
//! Multiple readers may hold the lock concurrently; a writer holds it exclusively.
//!
//! **Note:** Rust's standard library already provides [`std::sync::RwLock`]. This exercise implements
//! a minimal version for learning the protocol and policy without using the standard one.
//!
//! ## Common policies for read-write locks
//! Different implementations can give different **priority** when both readers and writers are waiting:
//!
//! - **Reader-priority (读者优先)**: New readers are allowed to enter while a writer is waiting, so writers
//!   may be starved if readers keep arriving.
//! - **Writer-priority (写者优先)**: Once a writer is waiting, no new readers are admitted until that writer
//!   has run; this exercise implements this policy.
//! - **Read-write fair (读写公平)**: Requests are served in a fair order (e.g. FIFO or round-robin), so
//!   neither readers nor writers are systematically starved.
//!
//! ## Key Concepts
//! - **Readers**: share access; many threads can hold a read lock at once.
//! - **Writer**: exclusive access; only one writer, and no readers while the writer holds the lock.
//! - **Writer-priority (this implementation)**: when at least one writer is waiting, new readers block
//!   until the writer runs.
//!
//! ## State (single atomic)
//! We use one `AtomicU32`: low bits = reader count, two flags = writer holding / writer waiting.
//! All logic is implemented with compare_exchange and load/store; no use of `std::sync::RwLock`.
//!
//! # 读写锁（写者优先）
//!
//! 在本练习中，你将使用原子操作从头实现一个**写者优先**的读写锁。
//! 多个读者可以同时持有锁；写者则独占持有锁。
//!
//! **注意：** Rust 标准库已经提供了 [`std::sync::RwLock`]。本练习实现的是一个简化版本，
//! 用于学习协议和策略，而不使用标准库的实现。
//!
//! ## 读写锁的常见策略
//! 当读者和写者同时等待时，不同的实现可以给出不同的**优先级**：
//!
//! - **读者优先**：允许新读者在写者等待时进入，因此如果读者不断到来，写者可能会饥饿。
//! - **写者优先**：一旦有写者等待，在该写者运行之前不允许新读者进入；本练习实现的就是此策略。
//! - **读写公平**：请求按公平顺序服务（例如 FIFO 或轮询），因此读者和写者都不会被系统性地饥饿。
//!
//! ## 关键概念
//! - **读者**：共享访问；多个线程可以同时持有读锁。
//! - **写者**：独占访问；只有一个写者，且在写者持有锁期间没有读者。
//! - **写者优先（此实现）**：当至少有一个写者等待时，新读者会被阻塞直到写者运行。
//!
//! ## 状态（单个原子变量）
//! 我们使用一个 `AtomicU32`：低比特位 = 读者计数，两个标志位 = 写者持有 / 写者等待。
//! 所有逻辑都使用 compare_exchange 和 load/store 实现；不使用 `std::sync::RwLock`。

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

/// Maximum number of concurrent readers (fits in state bits).
/// 最大并发读者数量（适应状态比特位）。
const READER_MASK: u32 = (1 << 30) - 1;
/// Bit set when a writer holds the lock.
/// 写者持有锁时设置的比特位。
const WRITER_HOLDING: u32 = 1 << 30;
/// Bit set when at least one writer is waiting (writer-priority: block new readers).
/// 至少有一个写者等待时设置的比特位（写者优先：阻塞新读者）。
const WRITER_WAITING: u32 = 1 << 31;

/// Writer-priority read-write lock. Implemented from scratch; does not use `std::sync::RwLock`.
/// 写者优先的读写锁。完全从头实现；不使用 `std::sync::RwLock`。
pub struct RwLock<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock. Blocks (spins) until no writer holds and no writer is waiting (writer-priority).
    ///
    /// TODO: Implement read lock acquisition
    /// 1. In a loop, load state (Acquire).
    /// 2. If WRITER_HOLDING or WRITER_WAITING is set, spin_loop and continue (writer-priority: no new readers while writer waits).
    /// 3. If reader count (state & READER_MASK) is already READER_MASK, spin and continue.
    /// 4. Try compare_exchange(s, s + 1, AcqRel, Acquire); on success return RwLockReadGuard { lock: self }.
    ///
    /// 获取读锁。阻塞（自旋）直到没有写者持有且没有写者等待（写者优先）。
    ///
    /// TODO: 实现读锁获取
    /// 1. 在循环中，加载状态（Acquire）。
    /// 2. 如果设置了 WRITER_HOLDING 或 WRITER_WAITING，自旋并继续（写者优先：写者等待时不允许新读者）。
    /// 3. 如果读者计数（state & READER_MASK）已经达到 READER_MASK，自旋并继续。
    /// 4. 尝试 compare_exchange(s, s + 1, AcqRel, Acquire)；成功后返回 RwLockReadGuard { lock: self }。
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        // TODO
        //todo!()
        loop{
            let state = self.state.load(Ordering::Acquire);
            if state & (WRITER_HOLDING | WRITER_WAITING) != 0 {
                continue;
            }
            if state & READER_MASK == READER_MASK {
                continue;
            }
            if self.state.compare_exchange(state, state + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                return RwLockReadGuard { lock: self };
            }
        }
    }

    /// Acquire the write lock. Blocks until no readers and no other writer.
    ///
    /// TODO: Implement write lock acquisition (writer-priority)
    /// 1. Set WRITER_WAITING first: fetch_or(WRITER_WAITING, Release) so new readers will block.
    /// 2. In a loop: load state; if any readers (READER_MASK) or WRITER_HOLDING, spin_loop and continue.
    /// 3. Try compare_exchange(WRITER_WAITING, WRITER_HOLDING, ...) to take the lock; or compare_exchange(0, WRITER_HOLDING, ...) if a writer just released.
    /// 4. On success return RwLockWriteGuard { lock: self }.
    ///
    /// 获取写锁。阻塞直到没有读者且没有其他写者。
    ///
    /// TODO: 实现写锁获取（写者优先）
    /// 1. 首先设置 WRITER_WAITING：fetch_or(WRITER_WAITING, Release)，以便新读者将被阻塞。
    /// 2. 在循环中：加载状态；如果有任何读者（READER_MASK）或 WRITER_HOLDING，自旋并继续。
    /// 3. 尝试 compare_exchange(WRITER_WAITING, WRITER_HOLDING, ...) 来获取锁；或者如果一个写者刚释放，则 compare_exchange(0, WRITER_HOLDING, ...)。
    /// 4. 成功后返回 RwLockWriteGuard { lock: self }。
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        // TODO
        //todo!()
        self.state.fetch_or(WRITER_WAITING, Ordering::Release);
        loop{
            let state = self.state.load(Ordering::Acquire);
            if state & (WRITER_HOLDING | READER_MASK) != 0 {
                continue;
            }
            if self.state.compare_exchange(state, WRITER_HOLDING, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                return RwLockWriteGuard { lock: self };
            }
        }
    }
}

/// Guard for a read lock; releases the read lock on drop.
/// 读锁的守卫；在 drop 时释放读锁。
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

// TODO: Implement Deref for RwLockReadGuard
// Return shared reference to data: unsafe { &*self.lock.data.get() }
//
// TODO: 为 RwLockReadGuard 实现 Deref
// 返回数据的共享引用：unsafe { &*self.lock.data.get() }
impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        //todo!()
        unsafe { & *self.lock.data.get() }
    }
}

// TODO: Implement Drop for RwLockReadGuard
// Decrement reader count: self.lock.state.fetch_sub(1, Ordering::Release)
//
// TODO: 为 RwLockReadGuard 实现 Drop
// 减少读者计数：self.lock.state.fetch_sub(1, Ordering::Release)
impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        //todo!()
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

/// Guard for a write lock; releases the write lock on drop.
/// 写锁的守卫；在 drop 时释放写锁。
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

// TODO: Implement Deref for RwLockWriteGuard
// Return shared reference: unsafe { &*self.lock.data.get() }
//
// TODO: 为 RwLockWriteGuard 实现 Deref
// 返回数据的共享引用：unsafe { &*self.lock.data.get() }
impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        //todo!()
        unsafe { & *self.lock.data.get() }
    }
}

// TODO: Implement DerefMut for RwLockWriteGuard
// Return mutable reference: unsafe { &mut *self.lock.data.get() }
//
// TODO: 为 RwLockWriteGuard 实现 DerefMut
// 返回可变引用：unsafe { &mut *self.lock.data.get() }
impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        //todo!()
        unsafe { &mut *self.lock.data.get() }
    }
}

// TODO: Implement Drop for RwLockWriteGuard
// Clear writer bits so lock is free: self.lock.state.fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release)
//
// TODO: 为 RwLockWriteGuard 实现 Drop
// 清除写者比特位以释放锁：self.lock.state.fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release)
impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        //todo!()
        self.lock.state.fetch_and(!(WRITER_HOLDING | WRITER_WAITING), Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_multiple_readers() {
        let lock = Arc::new(RwLock::new(0u32));
        let mut handles = vec![];
        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let g = l.read();
                assert_eq!(*g, 0);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_writer_excludes_readers() {
        let lock = Arc::new(RwLock::new(0u32));
        let lock_w = Arc::clone(&lock);
        let writer = thread::spawn(move || {
            let mut g = lock_w.write();
            *g = 42;
        });
        writer.join().unwrap();
        let g = lock.read();
        assert_eq!(*g, 42);
    }

    #[test]
    fn test_concurrent_reads_after_write() {
        let lock = Arc::new(RwLock::new(Vec::<i32>::new()));
        {
            let mut g = lock.write();
            g.push(1);
            g.push(2);
        }
        let mut handles = vec![];
        for _ in 0..5 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let g = l.read();
                assert_eq!(g.len(), 2);
                assert_eq!(&*g, &[1, 2]);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_writes_serialized() {
        let lock = Arc::new(RwLock::new(0u64));
        let mut handles = vec![];
        for _ in 0..10 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut g = l.write();
                    *g += 1;
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*lock.read(), 1000);
    }
}
