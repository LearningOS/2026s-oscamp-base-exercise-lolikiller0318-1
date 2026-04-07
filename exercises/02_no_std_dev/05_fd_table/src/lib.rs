//! # File Descriptor Table
//! 
//! Implement a simple file descriptor (fd) table — the core data structure
//! for managing open files in an OS kernel.
//! 
//! ## Background
//! 
//! In the Linux kernel, each process has an fd table that maps integer fds to kernel file objects.
//! User programs perform read/write/close via fds, and the kernel looks up the corresponding
//! file object through the fd table.
//! 
//! ```text
//! fd table:
//!   0 -> Stdin
//!   1 -> Stdout
//!   2 -> Stderr
//!   3 -> File("/etc/passwd")
//!   4 -> (empty)
//!   5 -> Socket(...)
//! ```
//! 
//! ## Task
//! 
//! Implement the following methods on `FdTable`:
//! 
//! - `new()` — create an empty fd table
//! - `alloc(file)` -> `usize` — allocate a new fd, return the fd number
//!   - Prefer reusing the smallest closed fd number
//!   - If no free slot, extend the table
//! - `get(fd)` -> `Option<Arc<dyn File>>` — get the file object for an fd
//! - `close(fd)` -> `bool` — close an fd, return whether it succeeded (false if fd doesn't exist)
//! - `count()` -> `usize` — return the number of currently allocated fds (excluding closed ones)
//! 
//! ## Key Concepts
//! 
//! - Trait objects: `Arc<dyn File>`
//! - `Vec<Option<T>>` as a sparse table
//! - fd number reuse strategy (find smallest free slot)
//! - `Arc` reference counting and resource release
//! 
//! # 文件描述符表
//! 
//! 实现一个简单的文件描述符（fd）表 —— 操作系统内核中管理打开文件的核心数据结构。
//! 
//! ## 背景
//! 
//! 在 Linux 内核中，每个进程都有一个 fd 表，将整数 fd 映射到内核文件对象。
//! 用户程序通过 fd 执行读/写/关闭操作，内核通过 fd 表查找对应的文件对象。
//! 
//! ```text
//! fd 表:
//!   0 -> Stdin
//!   1 -> Stdout
//!   2 -> Stderr
//!   3 -> File("/etc/passwd")
//!   4 -> (空)
//!   5 -> Socket(...)
//! ```
//! 
//! ## 任务
//! 
//! 在 `FdTable` 上实现以下方法：
//! 
//! - `new()` — 创建一个空的 fd 表
//! - `alloc(file)` -> `usize` — 分配一个新的 fd，返回 fd 编号
//!   - 优先重用最小的已关闭 fd 编号
//!   - 如果没有空闲槽位，扩展表
//! - `get(fd)` -> `Option<Arc<dyn File>>` — 获取 fd 对应的文件对象
//! - `close(fd)` -> `bool` — 关闭 fd，返回是否成功（如果 fd 不存在则返回 false）
//! - `count()` -> `usize` — 返回当前已分配的 fd 数量（不包括已关闭的）
//! 
//! ## 关键概念
//! 
//! - 特质对象：`Arc<dyn File>`
//! - 使用 `Vec<Option<T>>` 作为稀疏表
//! - fd 编号重用策略（查找最小的空闲槽位）
//! - `Arc` 引用计数和资源释放

use std::sync::Arc;

/// File abstraction trait — all "files" in the kernel (regular files, pipes, sockets) implement this
/// 文件抽象特质 —— 内核中的所有“文件”（常规文件、管道、套接字）都实现此特质
pub trait File: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> isize;
    fn write(&self, buf: &[u8]) -> isize;
}

/// File descriptor table
/// 文件描述符表
pub struct FdTable {
    // TODO: Design the internal structure
    // Hint: use Vec<Option<Arc<dyn File>>>
    //       the index is the fd number, None means the fd is closed or unallocated
    //
    // TODO: 设计内部结构
    // 提示：使用 Vec<Option<Arc<dyn File>>>
    //       索引是 fd 编号，None 表示 fd 已关闭或未分配
    files: Vec<Option<Arc<dyn File>>>,
    file_count: usize,
}

impl FdTable {
    /// Create an empty fd table
    /// 创建一个空的 fd 表
    pub fn new() -> Self {
        // TODO
        //todo!()
        let new_fd_table = Self {
            files: Vec::new(),
            file_count: 0,
        };
        new_fd_table
    }

    /// Allocate a new fd, return the fd number.
    ///
    /// Prefers reusing the smallest closed fd number; if no free slot, appends to the end.
    ///
    /// 分配一个新的 fd，返回 fd 编号。
    ///
    /// 优先重用最小的已关闭 fd 编号；如果没有空闲槽位，则追加到末尾。
    pub fn alloc(&mut self, file: Arc<dyn File>) -> usize {
        // TODO
        //todo!()
        for i in 0..self.files.len() {
            match self.files[i] {
                None => {
                    self.files[i] = Some(file);
                    return i;
                }
                Some(_) => continue,
            }
        }
        self.files.push(Some(file));
        self.file_count += 1;
        self.files.len() - 1
    }

    /// Get the file object for an fd. Returns None if the fd doesn't exist or is closed.
    /// 获取 fd 对应的文件对象。如果 fd 不存在或已关闭，则返回 None。
    pub fn get(&self, fd: usize) -> Option<Arc<dyn File>> {
        // TODO
        //todo!()
        match self.files.get(fd) {
            Some(file) => file.clone(),
            None => None,
        }
    }

    /// Close an fd. Returns true on success, false if the fd doesn't exist or is already closed.
    /// 关闭一个 fd。成功返回 true，如果 fd 不存在或已经关闭则返回 false。
    pub fn close(&mut self, fd: usize) -> bool {
        // TODO
        //todo!()
        if let Some(file) = self.files.get(fd) {
            if self.files[fd].is_some() {
                self.files[fd] = None;
                self.file_count -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Return the number of currently allocated fds (excluding closed ones)
    /// 返回当前已分配的 fd 数量（不包括已关闭的）
    pub fn count(&self) -> usize {
        // TODO
        //todo!()
        self.file_count
    }
}

impl Default for FdTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Test File implementation
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockFile {
        id: usize,
        write_log: Mutex<Vec<Vec<u8>>>,
    }

    impl MockFile {
        fn new(id: usize) -> Arc<Self> {
            Arc::new(Self {
                id,
                write_log: Mutex::new(vec![]),
            })
        }
    }

    impl File for MockFile {
        fn read(&self, buf: &mut [u8]) -> isize {
            buf[0] = self.id as u8;
            1
        }
        fn write(&self, buf: &[u8]) -> isize {
            self.write_log.lock().unwrap().push(buf.to_vec());
            buf.len() as isize
        }
    }

    #[test]
    fn test_alloc_basic() {
        let mut table = FdTable::new();
        let fd = table.alloc(MockFile::new(0));
        assert_eq!(fd, 0, "first fd should be 0");
        let fd2 = table.alloc(MockFile::new(1));
        assert_eq!(fd2, 1, "second fd should be 1");
    }

    #[test]
    fn test_get() {
        let mut table = FdTable::new();
        let file = MockFile::new(42);
        let fd = table.alloc(file);
        let got = table.get(fd);
        assert!(got.is_some(), "get should return Some");
        let mut buf = [0u8; 1];
        got.unwrap().read(&mut buf);
        assert_eq!(buf[0], 42);
    }

    #[test]
    fn test_get_invalid() {
        let table = FdTable::new();
        assert!(table.get(0).is_none());
        assert!(table.get(999).is_none());
    }

    #[test]
    fn test_close_and_reuse() {
        let mut table = FdTable::new();
        let fd0 = table.alloc(MockFile::new(0)); // fd=0
        let fd1 = table.alloc(MockFile::new(1)); // fd=1
        let fd2 = table.alloc(MockFile::new(2)); // fd=2

        assert!(table.close(fd1), "closing fd=1 should succeed");
        assert!(
            table.get(fd1).is_none(),
            "get should return None after close"
        );

        // Next allocation should reuse fd=1 (smallest free)
        let fd_new = table.alloc(MockFile::new(99));
        assert_eq!(fd_new, fd1, "should reuse the smallest closed fd");

        let _ = (fd0, fd2);
    }

    #[test]
    fn test_close_invalid() {
        let mut table = FdTable::new();
        assert!(
            !table.close(0),
            "closing non-existent fd should return false"
        );
    }

    #[test]
    fn test_count() {
        let mut table = FdTable::new();
        assert_eq!(table.count(), 0);
        let fd0 = table.alloc(MockFile::new(0));
        let fd1 = table.alloc(MockFile::new(1));
        assert_eq!(table.count(), 2);
        table.close(fd0);
        assert_eq!(table.count(), 1);
        table.close(fd1);
        assert_eq!(table.count(), 0);
    }

    #[test]
    fn test_write_through_fd() {
        let mut table = FdTable::new();
        let file = MockFile::new(0);
        let fd = table.alloc(file);
        let f = table.get(fd).unwrap();
        let n = f.write(b"hello");
        assert_eq!(n, 5);
    }
}
