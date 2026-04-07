//! # Free-List Allocator
//! 
//! Building on the bump allocator, implement a Free-List Allocator that supports memory reclamation.
//! 
//! ## How It Works
//! 
//! A Free-List Allocator uses a linked list to track all freed memory blocks.
//! On allocation, it first searches the list for a suitable block (first-fit strategy);
//! if none is found, it falls back to allocating from the unused region.
//! On deallocation, the block is inserted at the head of the list.
//! 
//! ```text
//! free_list -> [block A: 64B] -> [block B: 128B] -> [block C: 32B] -> null
//! ```
//! 
//! Each free block stores a `FreeBlock` struct at its head (containing block size and next pointer).
//! 
//! ## Task
//! 
//! Implement `FreeListAllocator`'s `alloc` and `dealloc` methods:
//! 
//! ### alloc
//! 1. Traverse the free_list, find the first block with `size >= layout.size()` and proper alignment (first-fit)
//! 2. If found, remove it from the list and return it
//! 3. If not found, allocate from the `bump` region (same as bump allocator)
//! 
//! ### dealloc
//! 1. Write `FreeBlock` header info at the freed block
//! 2. Insert it at the head of free_list
//! 
//! ## Key Concepts
//! 
//! - Intrusive linked list
//! - `*mut T` read/write: `ptr.write(val)` / `ptr.read()`
//! - Memory alignment checks
//! 
//! # 空闲链表分配器
//! 
//! 在 bump 分配器的基础上，实现一个支持内存回收的空闲链表分配器。
//! 
//! ## 工作原理
//! 
//! 空闲链表分配器使用链表来跟踪所有已释放的内存块。
//! 在分配时，它首先在链表中搜索合适的块（首次适应策略）；
//! 如果没有找到，则从未使用的区域分配。
//! 在释放时，块被插入到链表的头部。
//! 
//! ```text
//! free_list -> [block A: 64B] -> [block B: 128B] -> [block C: 32B] -> null
//! ```
//! 
//! 每个空闲块在其头部存储一个 `FreeBlock` 结构体（包含块大小和下一个指针）。
//! 
//! ## 任务
//! 
//! 实现 `FreeListAllocator` 的 `alloc` 和 `dealloc` 方法：
//! 
//! ### alloc
//! 1. 遍历 free_list，找到第一个大小 `>= layout.size()` 且对齐正确的块（首次适应）
//! 2. 如果找到，将其从链表中移除并返回
//! 3. 如果没有找到，从 `bump` 区域分配（与 bump 分配器相同）
//! 
//! ### dealloc
//! 1. 在释放的块中写入 `FreeBlock` 头部信息
//! 2. 将其插入到 free_list 的头部
//! 
//! ## 关键概念
//! 
//! - 侵入式链表
//! - `*mut T` 读写：`ptr.write(val)` / `ptr.read()`
//! - 内存对齐检查

#![cfg_attr(not(test), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::Ordering;

/// Free block header, stored at the beginning of each free memory block
/// 空闲块头部，存储在每个空闲内存块的开头
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

pub struct FreeListAllocator {
    heap_start: usize,
    heap_end: usize,
    /// Bump pointer: unallocated region starts here
    /// Bump 指针：未分配区域从这里开始
    bump_next: core::sync::atomic::AtomicUsize,
    /// Free list head (protected by Mutex in test, UnsafeCell otherwise)
    /// 空闲链表头部（测试中由 Mutex 保护，其他情况下由 UnsafeCell 保护）
    #[cfg(test)]
    free_list: std::sync::Mutex<*mut FreeBlock>,
    #[cfg(not(test))]
    free_list: core::cell::UnsafeCell<*mut FreeBlock>,
}

#[cfg(test)]
unsafe impl Send for FreeListAllocator {}
#[cfg(test)]
unsafe impl Sync for FreeListAllocator {}
#[cfg(not(test))]
unsafe impl Send for FreeListAllocator {}
#[cfg(not(test))]
unsafe impl Sync for FreeListAllocator {}

impl FreeListAllocator {
    /// # Safety
    /// `heap_start..heap_end` must be a valid readable and writable memory region.
    ///
    /// # 安全性
    /// `heap_start..heap_end` 必须是一个有效的可读可写内存区域。
    pub unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self {
            heap_start,
            heap_end,
            bump_next: core::sync::atomic::AtomicUsize::new(heap_start),
            #[cfg(test)]
            free_list: std::sync::Mutex::new(null_mut()),
            #[cfg(not(test))]
            free_list: core::cell::UnsafeCell::new(null_mut()),
        }
    }

    #[cfg(test)]
    fn free_list_head(&self) -> *mut FreeBlock {
        *self.free_list.lock().unwrap()
    }

    #[cfg(test)]
    fn set_free_list_head(&self, head: *mut FreeBlock) {
        *self.free_list.lock().unwrap() = head;
    }

    #[cfg(not(test))]
    fn free_list_head(&self) -> *mut FreeBlock {
        unsafe { *self.free_list.get() }
    }

    #[cfg(not(test))]
    fn set_free_list_head(&self, head: *mut FreeBlock) {
        unsafe { *self.free_list.get() = head }
    }
}

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ensure block is at least large enough to hold a FreeBlock header (for future dealloc)
        // 确保块至少足够大以容纳 FreeBlock 头部（用于将来的释放）
        let size = layout.size().max(core::mem::size_of::<FreeBlock>());
        let align = layout.align().max(core::mem::align_of::<FreeBlock>());

        // TODO: Step 1 — traverse free_list, find a suitable block (first-fit)
        //
        // Hints:
        // - Use prev_ptr and curr to traverse the list
        // - Check if curr address satisfies align, and (*curr).size >= size
        // - If found, remove it from the list (update prev's next or the free_list head)
        // - Return curr as *mut u8
        //
        // TODO: 步骤 1 — 遍历 free_list，找到合适的块（首次适应）
        //
        // 提示：
        // - 使用 prev_ptr 和 curr 遍历链表
        // - 检查 curr 地址是否满足对齐要求，且 (*curr).size >= size
        // - 如果找到，将其从链表中移除（更新 prev 的 next 或 free_list 头部）
        // - 将 curr 作为 *mut u8 返回
        let mut prev_ptr : *mut FreeBlock = core::ptr::null_mut();
        let mut curr = self.free_list_head();
        
        while !curr.is_null() {
            
            if (*curr).size >= size && (*curr).size % align == 0 {
                if prev_ptr.is_null() {
                    self.set_free_list_head(curr);
                } else {
                    prev_ptr.write(FreeBlock {
                        size: prev_ptr.read().size,
                        next: curr,
                    });
                    
                }
                return curr as *mut u8;
            }
            prev_ptr = curr;
            curr = (*curr).next;
        }
        // TODO: Step 2 — no suitable block in free_list, allocate from bump region
        //
        // Same logic as 02_bump_allocator's alloc
        //
        // TODO: 步骤 2 — free_list 中没有合适的块，从 与 02_bump_allocator 的 alloc 逻辑相同
        //todo!()
        loop{
            let next = self.bump_next.load(Ordering::SeqCst);
            let align = (next+align-1)&!(align-1);
            let end = align+size;
            if end >= self.heap_end {
                return core::ptr::null_mut();
            }
            self.bump_next.compare_exchange(next, end, Ordering::SeqCst, Ordering::SeqCst).unwrap();
            return align as *mut u8;
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(core::mem::size_of::<FreeBlock>());

        // TODO: Insert the freed block at the head of free_list
        //
        // Steps:
        // 1. Cast ptr to *mut FreeBlock
        // 2. Write FreeBlock { size, next: current list head }
        // 3. Update free_list head to ptr
        //
        // TODO: 将释放的块插入到 free_list 的头部
        //
        // 步骤：
        // 1. 将 ptr 转换为 *mut FreeBlock
        // 2. 写入 FreeBlock { size, next: 当前链表头部 }
        // 3. 更新 free_list 头部为 ptr
        //todo!()
        let ptr_block = ptr as *mut FreeBlock;
        ptr_block.write(FreeBlock {
            size,
            next: self.free_list_head(),
        });
        self.set_free_list_head(ptr_block);
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: usize = 4096;

    fn make_allocator() -> (FreeListAllocator, Vec<u8>) {
        let mut heap = vec![0u8; HEAP_SIZE];
        let start = heap.as_mut_ptr() as usize;
        let alloc = unsafe { FreeListAllocator::new(start, start + HEAP_SIZE) };
        (alloc, heap)
    }

    #[test]
    fn test_alloc_basic() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(32, 8).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_alloc_alignment() {
        let (alloc, _heap) = make_allocator();
        for align in [1, 2, 4, 8, 16] {
            let layout = Layout::from_size_align(8, align).unwrap();
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % align, 0, "align={align}");
        }
    }

    #[test]
    fn test_dealloc_and_reuse() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null());

        // After freeing, the next allocation should reuse the same block
        unsafe { alloc.dealloc(p1, layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        assert!(!p2.is_null());
        assert_eq!(p1, p2, "should reuse the freed block");
    }

    #[test]
    fn test_multiple_alloc_dealloc() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(128, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        let p3 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null() && !p2.is_null() && !p3.is_null());

        unsafe { alloc.dealloc(p2, layout) };
        unsafe { alloc.dealloc(p1, layout) };

        let q1 = unsafe { alloc.alloc(layout) };
        let q2 = unsafe { alloc.alloc(layout) };
        assert!(!q1.is_null() && !q2.is_null());
    }

    #[test]
    fn test_oom() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "should return null when exceeding heap");
    }
}
