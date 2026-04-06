//! # no_std Memory Primitives
//! 
//! In a `#![no_std]` environment, you have no standard library — only `core`.
//! These memory operation functions are the most fundamental building blocks in an OS kernel.
//! Functions like memcpy/memset in libc must be implemented by ourselves in bare-metal environments.
//! 
//! ## Task
//! 
//! Implement the following five functions:
//! - Only use the `core` crate, no `std`
//! - Do not call `core::ptr::copy`, `core::ptr::copy_nonoverlapping`, etc. (write your own loops)
//! - Handle edge cases correctly (n=0, overlapping memory regions, etc.)
//! - Pass all tests
//! 
//! # no_std 内存原语
//! 
//! 在 `#![no_std]` 环境中，你没有标准库 —— 只有 `core`。
//! 这些内存操作函数是操作系统内核中最基本的构建块。
//! 像 libc 中的 memcpy/memset 等函数必须在裸机环境中由我们自己实现。
//! 
//! ## 任务
//! 
//! 实现以下五个函数：
//! - 只使用 `core`  crate，不使用 `std`
//! - 不要调用 `core::ptr::copy`、`core::ptr::copy_nonoverlapping` 等（自己编写循环）
//! - 正确处理边界情况（n=0、内存区域重叠等）
//! - 通过所有测试

// Force no_std in production; allow std in tests (cargo test framework requires it)
// 在生产环境中强制使用 no_std；在测试中允许使用 std（cargo 测试框架需要它）
#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]

/// Copy `n` bytes from `src` to `dst`.
/// 
/// - `dst` and `src` must not overlap (use `my_memmove` for overlapping regions)
/// - Returns `dst`
/// 
/// # Safety
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
/// 
/// 将 `n` 字节从 `src` 复制到 `dst`。
/// 
/// - `dst` 和 `src` 不得重叠（重叠区域使用 `my_memmove`）
/// - 返回 `dst`
/// 
/// # 安全性
/// `dst` 和 `src` 必须各自指向至少 `n` 字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: Implement memcpy
    // Hint: read bytes from src one by one and write to dst
    // TODO: 实现 memcpy
    // 提示：从 src 逐个读取字节并写入 dst
    //todo!()
    let mut d=dst;
    let mut s=src;
    let length=n;
    for i in 0..length{
        *d = *s;
        d = d.add(1);
        s = s.add(1);
        
    }
    d
}

/// Set `n` bytes starting at `dst` to the value `c`.
/// 
/// Returns `dst`.
/// 
/// # Safety
/// `dst` must point to at least `n` bytes of valid writable memory.
/// 
/// 将从 `dst` 开始的 `n` 字节设置为值 `c`。
/// 
/// 返回 `dst`。
/// 
/// # 安全性
/// `dst` 必须指向至少 `n` 字节的有效可写内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memset(dst: *mut u8, c: u8, n: usize) -> *mut u8 {
    // TODO: Implement memset
    // TODO: 实现 memset
    //todo!()
    let mut d=dst;
    let length=n;
    for i in 0..length{
        *d = c;
        d = d.add(1);
        
    }
    d

}

/// Copy `n` bytes from `src` to `dst`, correctly handling overlapping memory.
/// 
/// Returns `dst`.
/// 
/// # Safety
/// `dst` and `src` must each point to at least `n` bytes of valid memory.
/// 
/// 将 `n` 字节从 `src` 复制到 `dst`，正确处理内存重叠情况。
/// 
/// 返回 `dst`。
/// 
/// # 安全性
/// `dst` 和 `src` 必须各自指向至少 `n` 字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: Implement memmove
    // Hint: when dst > src and regions overlap, copy backwards (from end to start)
    // TODO: 实现 memmove
    // 提示：当 dst > src 且区域重叠时，从后向前复制（从末尾到开始）
    //todo!()
    let mut d=dst;
    let mut s=src;
    d=d.add(n-1);
    s=s.add(n-1);

    let length=n;
    for i in 0..length{
        *d = *s;
        d = d.sub(1);
        s = s.sub(1);
    }
    d
}

/// Return the length of a null-terminated byte string, excluding the trailing null.
/// 
/// # Safety
/// `s` must point to a valid null-terminated byte string.
/// 
/// 返回以 null 结尾的字节字符串的长度，不包括结尾的 null。
/// 
/// # 安全性
/// `s` 必须指向有效的以 null 结尾的字节字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strlen(s: *const u8) -> usize {
    // TODO: Implement strlen
    // TODO: 实现 strlen
    //todo!()
    let mut ptr=s;
    
    while *ptr != 0{
        ptr = ptr.add(1);
        
    }
    ptr.offset_from(s) as usize
}

/// Compare two null-terminated byte strings.
/// 
/// Returns:
/// - `0`  : strings are equal
/// - `< 0`: `s1` is lexicographically less than `s2`
/// - `> 0`: `s1` is lexicographically greater than `s2`
/// 
/// # Safety
/// `s1` and `s2` must each point to a valid null-terminated byte string.
/// 
/// 比较两个以 null 结尾的字节字符串。
/// 
/// 返回：
/// - `0` ：字符串相等
/// - `< 0`：`s1` 在字典序上小于 `s2`
/// - `> 0`：`s1` 在字典序上大于 `s2`
/// 
/// # 安全性
/// `s1` 和 `s2` 必须各自指向有效的以 null 结尾的字节字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    // TODO: Implement strcmp
    // TODO: 实现 strcmp
    //todo!()
    let mut ptr1=s1;
    let mut ptr2=s2;
    while *ptr1 == *ptr2 && *ptr1 != 0{
        if *ptr1 != *ptr2{
            return *ptr1 as i32 - *ptr2 as i32;
        }
        ptr1 = ptr1.add(1);
        ptr2 = ptr2.add(1);
        
    }
    return *ptr1 as i32 - *ptr2 as i32;
}

// ============================================================
// Tests (std is available under #[cfg(test)])
// ============================================================
// ============================================================
// 测试（在 #[cfg(test)] 下可以使用 std）
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memcpy_basic() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 5) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memcpy_zero_len() {
        let src = [0xFFu8; 4];
        let mut dst = [0u8; 4];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 0) };
        assert_eq!(dst, [0u8; 4]);
    }

    #[test]
    fn test_memset_basic() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xAB, 8) };
        assert!(buf.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_memset_partial() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xFF, 4) };
        assert_eq!(&buf[..4], &[0xFF; 4]);
        assert_eq!(&buf[4..], &[0x00; 4]);
    }

    #[test]
    fn test_memmove_no_overlap() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        unsafe { my_memmove(dst.as_mut_ptr(), src.as_ptr(), 4) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memmove_overlap_forward() {
        // Copy buf[0..4] to buf[1..5], shifting right by 1
        let mut buf = [1u8, 2, 3, 4, 5];
        unsafe { my_memmove(buf.as_mut_ptr().add(1), buf.as_ptr(), 4) };
        assert_eq!(buf, [1, 1, 2, 3, 4]);
    }

    #[test]
    fn test_strlen_basic() {
        let s = b"hello\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 5);
    }

    #[test]
    fn test_strlen_empty() {
        let s = b"\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_equal() {
        let a = b"hello\0";
        let b = b"hello\0";
        assert_eq!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_less() {
        let a = b"abc\0";
        let b = b"abd\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } < 0);
    }

    #[test]
    fn test_strcmp_greater() {
        let a = b"abd\0";
        let b = b"abc\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } > 0);
    }
}
