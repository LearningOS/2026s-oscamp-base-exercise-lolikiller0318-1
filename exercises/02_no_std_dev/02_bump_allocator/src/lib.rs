//! # Bump Allocator (no_std)
//! 
//! Implement the simplest heap memory allocator: a Bump Allocator (bump pointer allocator).
//! 
//! ## How It Works
//! 
//! A Bump Allocator maintains a pointer `next` to the "next available address".
//! On each allocation, it aligns `next` to the requested alignment, then advances by `size` bytes.
//! It does not support freeing individual objects (`dealloc` is a no-op).
//! 
//! ```text
//! heap_start                              heap_end
//! |----[allocated]----[allocated]----| next |---[free]---|
//!                                        ^
//!                                    next allocation starts here
//! ```
//! 
//! ## Task
//! 
//! Implement `BumpAllocator`'s `GlobalAlloc::alloc` method:
//! 1. Align the current `next` up to `layout.align()`
//!    Hint: `align_up(addr, align) = (addr + align - 1) & !(align - 1)`
//! 2. Check if the aligned address plus `layout.size()` exceeds `heap_end`
//! 3. If it exceeds, return `null_mut()`; otherwise atomically update `next` with `compare_exchange`
//! 
//! ## Key Concepts
//! 
//! - `core::alloc::{GlobalAlloc, Layout}`
//! - Memory alignment calculation
//! - `AtomicUsize` and `compare_exchange` (CAS loop)
//! 
//! # Bump 分配器 (no_std)
//! 
//! 实现最简单的堆内存分配器：Bump 分配器（ bump 指针分配器）。
//! 
//! ## 工作原理
//! 
//! Bump 分配器维护一个指向“下一个可用地址”的指针 `next`。
//! 在每次分配时，它将 `next` 对齐到请求的对齐方式，然后前进 `size` 字节。
//! 它不支持释放单个对象（`dealloc` 是一个空操作）。
//! 
//! ```text
//! heap_start                              heap_end
//! |----[allocated]----[allocated]----| next |---[free]---|
//!                                        ^
//!                                    next allocation starts here
//! ```
//! 
//! ## 任务
//! 
//! 实现 `BumpAllocator` 的 `GlobalAlloc::alloc` 方法：
//! 1. 将当前的 `next` 向上对齐到 `layout.align()`
//!    提示：`align_up(addr, align) = (addr + align - 1) & !(align - 1)`
//! 2. 检查对齐后的地址加上 `layout.size()` 是否超过 `heap_end`
//! 3. 如果超过，返回 `null_mut()`；否则使用 `compare_exchange` 原子更新 `next`
//! 
//! ## 关键概念
//! 
//! - `core::alloc::{GlobalAlloc, Layout}`
//! - 内存对齐计算
//! - `AtomicUsize` 和 `compare_exchange`（CAS 循环）

#![cfg_attr(not(test), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl BumpAllocator {
    /// Create a new BumpAllocator.
    ///
    /// # Safety
    /// `heap_start..heap_end` must be a valid, readable and writable memory region,
    /// and must not be used by other code during this allocator's lifetime.
    ///
    /// 创建一个新的 BumpAllocator。
    ///
    /// # 安全性
    /// `heap_start..heap_end` 必须是一个有效、可读且可写的内存区域，
    /// 并且在该分配器的生命周期内不得被其他代码使用。
    pub const unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self {
            heap_start,
            heap_end,
            next: AtomicUsize::new(heap_start),
        }
    }

    /// Reset the allocator (free all allocated memory).
    ///
    /// 重置分配器（释放所有已分配的内存）。
    pub fn reset(&self) {
        self.next.store(self.heap_start, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: Implement bump allocation
        //
        // Steps:
        // 1. Load current next (use Ordering::SeqCst)
        // 2. Align next up to layout.align()
        //    Hint: align_up(addr, align) = (addr + align - 1) & !(align - 1)
        // 3. Compute allocation end = aligned + layout.size()
        // 4. If end > heap_end, return null_mut()
        // 5. Atomically update next to end using compare_exchange
        //    (if CAS fails, another thread raced — retry in a loop)
        // 6. Return the aligned address as a pointer
        //
        // TODO: 实现 bump 分配
        //
        // 步骤：
        // 1. 加载当前的 next（使用 Ordering::SeqCst）
        // 2. 将 next 向上对齐到 layout.align()
        //    提示：align_up(addr, align) = (addr + align - 1) & !(align - 1)
        // 3. 计算分配结束地址 = 对齐后的地址 + layout.size()
        // 4. 如果结束地址 > heap_end，返回 null_mut()
        // 5. 使用 compare_exchange 原子更新 next 为结束地址
        //    （如果 CAS 失败，说明有其他线程竞争 —— 在循环中重试）
        // 6. 将对齐后的地址作为指针返回
        //todo!()
        let align = layout.align();
        let size = layout.size();
        loop{
            let next = self.next.load(Ordering::SeqCst);
            let aligned = (next + align - 1) & !(align - 1);
            let end = aligned + size;
            if end > self.heap_end{
                return null_mut();
            }
            if self.next.compare_exchange(next, end, Ordering::SeqCst, Ordering::SeqCst).is_ok(){
                return aligned as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not reclaim individual objects — leave empty
        // Bump 分配器不回收单个对象 —— 保持为空
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: usize = 4096;

    fn make_allocator() -> (BumpAllocator, Vec<u8>) {
        let mut heap = vec![0u8; HEAP_SIZE];
        let start = heap.as_mut_ptr() as usize;
        let alloc = unsafe { BumpAllocator::new(start, start + HEAP_SIZE) };
        (alloc, heap)
    }

    #[test]
    fn test_alloc_basic() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(16, 8).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(!ptr.is_null(), "allocation should succeed");
    }

    #[test]
    fn test_alloc_alignment() {
        let (alloc, _heap) = make_allocator();
        for align in [1, 2, 4, 8, 16, 64] {
            let layout = Layout::from_size_align(1, align).unwrap();
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null());
            assert_eq!(
                ptr as usize % align,
                0,
                "returned address must satisfy align={align}"
            );
        }
    }

    #[test]
    fn test_alloc_no_overlap() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();
        let p1 = unsafe { alloc.alloc(layout) } as usize;
        let p2 = unsafe { alloc.alloc(layout) } as usize;
        assert!(
            p1 + 64 <= p2 || p2 + 64 <= p1,
            "two allocations must not overlap"
        );
    }

    #[test]
    fn test_alloc_oom() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "should return null when exceeding heap");
    }

    #[test]
    fn test_alloc_fill_heap() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(256, 1).unwrap();
        for i in 0..16 {
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null(), "allocation #{i} should succeed");
        }
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "should return null when heap is full");
    }

    #[test]
    fn test_reset() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE, 1).unwrap();
        let p1 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null());
        alloc.reset();
        let p2 = unsafe { alloc.alloc(layout) };
        assert!(!p2.is_null(), "should be able to allocate after reset");
        assert_eq!(
            p1, p2,
            "address after reset should match the first allocation"
        );
    }
}
