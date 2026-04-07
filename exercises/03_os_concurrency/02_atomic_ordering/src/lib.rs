//! # Memory Ordering and Synchronization
//! 
//! In this exercise, you will use correct memory ordering to implement thread synchronization primitives.
//! 
//! ## Key Concepts
//! - `Ordering::Relaxed`: No synchronization guarantees
//! - `Ordering::Acquire`: Read operation, prevents subsequent reads/writes from being reordered before this operation
//! - `Ordering::Release`: Write operation, prevents preceding reads/writes from being reordered after this operation
//! - `Ordering::AcqRel`: Both Acquire and Release semantics
//! - `Ordering::SeqCst`: Sequentially consistent (global ordering)
//! 
//! ## Release-Acquire Pairing
//! When thread A writes with Release, and thread B reads the same location with Acquire,
//! thread B will see all writes that thread A performed before the Release.
//! 
//! # 内存顺序与同步
//! 
//! 在本练习中，你将使用正确的内存顺序来实现线程同步原语。
//! 
//! ## 关键概念
//! - `Ordering::Relaxed`：无同步保证
//! - `Ordering::Acquire`：读操作，防止后续的读/写被重排到此操作之前
//! - `Ordering::Release`：写操作，防止之前的读/写被重排到此操作之后
//! - `Ordering::AcqRel`：同时具有 Acquire 和 Release 语义
//! - `Ordering::SeqCst`：顺序一致（全局顺序）
//! 
//! ## Release-Acquire 配对
//! 当线程 A 使用 Release 写入，而线程 B 使用 Acquire 读取同一位置时，
//! 线程 B 将看到线程 A 在 Release 之前执行的所有写入操作。

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Use Release-Acquire semantics to safely pass data between two threads.
/// `produce` writes data first, then sets flag with Release;
/// `consume` reads flag with Acquire, ensuring it sees the data.
///
/// 使用 Release-Acquire 语义在两个线程之间安全地传递数据。
/// `produce` 首先写入数据，然后使用 Release 设置标志；
/// `consume` 使用 Acquire 读取标志，确保它能看到数据。
pub struct FlagChannel {
    data: AtomicU32,
    ready: AtomicBool,
}

impl FlagChannel {
    pub const fn new() -> Self {
        Self {
            data: AtomicU32::new(0),
            ready: AtomicBool::new(false),
        }
    }

    /// Producer: store data first, then set ready flag.
    /// TODO: Choose correct Ordering
    /// - What Ordering should be used for writing data?
    /// - What Ordering should be used for writing ready? (ensuring data writes are visible to consumer)
    /// 生产者：首先存储数据，然后设置就绪标志。
    /// TODO: 选择正确的 Ordering
    /// - 写入数据应该使用什么 Ordering？
    /// - 写入 ready 应该使用什么 Ordering？（确保数据写入对消费者可见）
    pub fn produce(&self, value: u32) {
        // TODO: Store data (choose appropriate Ordering)
        // TODO: Set ready = true (choose appropriate Ordering so data writes complete before this)
        // TODO: 存储数据（选择适当的 Ordering）
        // TODO: 设置 ready = true（选择适当的 Ordering 以确保数据写入在此之前完成）
        //todo!()
        self.data.store(value, Ordering::Release);
        self.ready.store(true, Ordering::Release);
    }

    /// Consumer: spin-wait for ready flag, then read data.
       /// TODO: Choose correct Ordering
    /// - What Ordering should be used for reading ready? (ensuring it sees data writes from produce)
    /// - What Ordering should be used for reading data?
    /// 消费者：自旋等待就绪标志，然后读取数据。
    /// TODO: 选择正确的 Ordering
    /// - 读取 ready 应该使用什么 Ordering？（确保它能看到 produce 中的数据写入）
    /// - 读取数据应该使用什么 Ordering？
    pub fn consume(&self) -> u32 {
        // TODO: Spin-wait for ready to become true (choose appropriate Ordering)
        // TODO: Read data (choose appropriate Ordering)
        //
        // TODO: 自旋等待 ready 变为 true（选择适当的 Ordering）
        // TODO: 读取数据（选择适当的 Ordering）
        //todo!()
        loop {
            match self.ready.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return self.data.load(Ordering::Acquire),
                Err(_) => continue,
            }
        }
    }

    /// Reset channel state
    /// 重置通道状态
    pub fn reset(&self) {
        self.ready.store(false, Ordering::Relaxed);
        self.data.store(0, Ordering::Relaxed);
    }
}

/// A simple once-initializer using SeqCst.
/// Guarantees `init` is executed only once, and all threads see the initialized value.
/// 一个使用 SeqCst 的简单一次性初始化器。
/// 保证 `init` 只执行一次，所有线程都能看到初始化后的值。
pub struct OnceCell {
    initialized: AtomicBool,
    value: AtomicU32,
}

impl OnceCell {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: AtomicU32::new(0),
        }
    }

    /// Attempt initialization. If not yet initialized, store value and return true.
    /// If already initialized, return false.
    ///
    /// Hint: use `compare_exchange` to ensure only one thread succeeds.
    ///
    /// 尝试初始化。如果尚未初始化，则存储值并返回 true。
    /// 如果已经初始化，则返回 false。
    ///
    /// 提示：使用 `compare_exchange` 确保只有一个线程成功。
    pub fn init(&self, val: u32) -> bool {
        // TODO: Use compare_exchange to ensure initialization only once
        // Store value on success
        // TODO: 使用 compare_exchange 确保只初始化一次
        // 成功后存储值
        //todo!()
        match self.initialized.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                self.value.store(val, Ordering::SeqCst);
                true
            }
            Err(_) => false,
               }
    }

    /// Get value. Returns Some if initialized, otherwise None.
    ///
    /// 获取值。如果已初始化则返回 Some，否则返回 None。
    pub fn get(&self) -> Option<u32> {
        // TODO: Check initialized flag, then read value
        // TODO: 检查初始化标志，然后读取值
        //todo!()
        if self.initialized.load(Ordering::Acquire) {
            Some(self.value.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_flag_channel() {
        let ch = Arc::new(FlagChannel::new());
        let ch2 = Arc::clone(&ch);

        let producer = thread::spawn(move || {
            ch2.produce(42);
        });

        let consumer = thread::spawn(move || ch.consume());

        producer.join().unwrap();
        let val = consumer.join().unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn test_flag_channel_large_value() {
        let ch = Arc::new(FlagChannel::new());
        let ch2 = Arc::clone(&ch);

        let producer = thread::spawn(move || {
            ch2.produce(0xDEAD_BEEF);
        });

        let val = ch.consume();
        producer.join().unwrap();
        assert_eq!(val, 0xDEAD_BEEF);
    }

    #[test]
    fn test_once_cell_init_once() {
        let cell = OnceCell::new();
        assert!(cell.init(42));
        assert!(!cell.init(100));
        assert_eq!(cell.get(), Some(42));
    }

    #[test]
    fn test_once_cell_not_initialized() {
        let cell = OnceCell::new();
        assert_eq!(cell.get(), None);
    }

    #[test]
    fn test_once_cell_concurrent() {
        let cell = Arc::new(OnceCell::new());
        let mut handles = vec![];

        for i in 0..10 {
            let c = Arc::clone(&cell);
            handles.push(thread::spawn(move || c.init(i)));
        }

        let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        // Exactly one thread initializes successfully
        //
        // 恰好只有一个线程成功初始化
        assert_eq!(results.iter().filter(|&&r| r).count(), 1);
        assert!(cell.get().is_some());
    }
}
