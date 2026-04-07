//! # Green Thread Scheduler (riscv64)
//! 绿色线程调度器 (riscv64)
//!
//! In this exercise, you build a simple cooperative (green) thread scheduler on top of context switching.
//! 在本练习中，你将在上下文切换的基础上构建一个简单的协作式（绿色）线程调度器。
//! This crate is **riscv64 only**; run with the repo's normal flow (`./check.sh` / `oscamp`) or natively on riscv64.
//! 此 crate 仅支持 **riscv64**；请使用仓库的正常流程（`./check.sh` / `oscamp`）运行，或在 riscv64 上原生运行。
//!
//! ## Key Concepts
//! 核心概念
//! - Cooperative vs preemptive scheduling
//! 协作式调度 vs 抢占式调度
//! - Thread state: `Ready`, `Running`, `Finished`
//! 线程状态：`Ready`（就绪）、`Running`（运行中）、`Finished`（已完成）
//! - `yield_now()`: current thread voluntarily gives up the CPU
//! `yield_now()`：当前线程主动让出 CPU
//! - Scheduler loop: pick next ready thread and switch to it
//! 调度器循环：选择下一个就绪线程并切换到它
//!
//! ## Design
//! 设计
//! Each green thread has its own stack and `TaskContext`. Threads call `yield_now()` to yield.
//! 每个绿色线程都有自己的栈和 `TaskContext`。线程通过调用 `yield_now()` 来主动让出执行权。
//! The scheduler round-robins among ready threads. User entry is wrapped by `thread_wrapper`, which
//! 调度器在就绪线程之间采用轮询策略。用户入口由 `thread_wrapper` 包装，
//! calls the entry then marks the thread `Finished` and switches back.
//! 它调用入口函数，然后将线程标记为 `Finished` 并切换回调度器。

#![cfg(target_arch = "riscv64")]

use core::arch::naked_asm;

/// Per-thread stack size. Slightly larger to avoid overflow under QEMU / test harness.
///每个线程的栈大小。稍大一些以避免在 QEMU / 测试框架下溢出。
const STACK_SIZE: usize = 1024 * 128;

/// Task context (riscv64); layout must match `01_stack_coroutine::TaskContext` and the asm below.
///任务上下文 (riscv64)；布局必须与 `01_stack_coroutine::TaskContext` 和下面的汇编代码一致。
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TaskContext {
    sp: u64,
    ra: u64,
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Ready,
    Running,
    Finished,
}

struct GreenThread {
    ctx: TaskContext,
    state: ThreadState,
    _stack: Option<Vec<u8>>,
    /// User entry; taken once when the thread is first scheduled and passed to `thread_wrapper`.
    ///用户入口；当线程首次被调度时获取一次，并传递给 `thread_wrapper`。
    entry: Option<extern "C" fn()>,
}

/// Set by the scheduler before switching to a new thread; `thread_wrapper` reads and calls it once.
///在切换到新线程之前由调度器设置；`thread_wrapper` 读取并调用一次。
static mut CURRENT_THREAD_ENTRY: Option<extern "C" fn()> = None;

/// Wrapper run as the initial `ra` for each green thread: call the user entry (from `CURRENT_THREAD_ENTRY`), then mark Finished and switch back.
///作为每个绿色线程的初始 `ra` 运行的包装器：调用用户入口（从 `CURRENT_THREAD_ENTRY` 获取），然后标记为 Finished 并切换回来。
extern "C" fn thread_wrapper() {
    let entry = unsafe { core::ptr::read(&raw const CURRENT_THREAD_ENTRY) };
    if let Some(f) = entry {
        unsafe { CURRENT_THREAD_ENTRY = None };
        f();
    }
    thread_finished();
}

/// Save current callee-saved regs into `old`, load from `new`, then `ret` to `new.ra`.
///将当前被调用者保存的寄存器保存到 `old`，从 `new` 加载，然后 `ret` 到 `new.ra`。
/// Zero `a0`/`a1` before `ret` so we don't leak pointers into the new context.
///在 `ret` 之前将 `a0`/`a1` 清零，这样我们就不会将指针泄露到新上下文中。
///
/// Must be `#[unsafe(naked)]` to prevent the compiler from generating a prologue/epilogue.
///必须使用 `#[unsafe(naked)]` 以防止编译器生成 prologue/epilogue。
#[unsafe(naked)]
unsafe extern "C" fn switch_context(_old: &mut TaskContext, _new: &TaskContext) {
    naked_asm!(
        "sd sp, 0(a0)",
        "sd ra, 8(a0)",
        "sd s0, 16(a0)",
        "sd s1, 24(a0)",
        "sd s2, 32(a0)",
        "sd s3, 40(a0)",
        "sd s4, 48(a0)",
        "sd s5, 56(a0)",
        "sd s6, 64(a0)",
        "sd s7, 72(a0)",
        "sd s8, 80(a0)",
        "sd s9, 88(a0)",
        "sd s10, 96(a0)",
        "sd s11, 104(a0)",
        "ld sp, 0(a1)",
        "ld ra, 8(a1)",
        "ld s0, 16(a1)",
        "ld s1, 24(a1)",
        "ld s2, 32(a1)",
        "ld s3, 40(a1)",
        "ld s4, 48(a1)",
        "ld s5, 56(a1)",
        "ld s6, 64(a1)",
        "ld s7, 72(a1)",
        "ld s8, 80(a1)",
        "ld s9, 88(a1)",
        "ld s10, 96(a1)",
        "ld s11, 104(a1)",
        "li a0, 0",
        "li a1, 0",
        "ret",
    );
}

pub struct Scheduler {
    threads: Vec<GreenThread>,
    current: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        let main_thread = GreenThread {
            ctx: TaskContext::default(),
            state: ThreadState::Running,
            _stack: None,
            entry: None,
        };

        Self {
            threads: vec![main_thread],
            current: 0,
        }
    }

    /// Register a new green thread that will run `entry` when first scheduled.
    ///注册一个新的绿色线程，该线程在首次被调度时将运行 `entry`。
    ///
    /// 1. Allocate a stack of `STACK_SIZE` bytes; compute `stack_top` (high address).
    ///1. 分配 `STACK_SIZE` 字节的栈；计算 `stack_top`（高地址）。
    /// 2. Set up the context: `ra = thread_wrapper` so the first switch jumps to the wrapper;
    ///2. 设置上下文：`ra = thread_wrapper`，这样第一次切换会跳转到包装器；
    ///    `sp` must be 16-byte aligned (e.g. `(stack_top - 16) & !15` to leave headroom).
    ///`sp` 必须 16 字节对齐（例如 `(stack_top - 16) & !15` 以留出余量）。
    /// 3. Push a `GreenThread` with this context, state `Ready`, and `entry` stored for the wrapper to call.
    ///3. 推送一个具有此上下文、状态 `Ready` 和 `entry` 的 `GreenThread`，供包装器调用。
    pub fn spawn(&mut self, entry: extern "C" fn()) {
        //TODO
        //todo!("alloc stack, init ctx with ra=thread_wrapper and aligned sp, push GreenThread(Ready, entry)")
        let buffer = vec![0u8; STACK_SIZE];

    }

    /// Run the scheduler until all threads (except the main one) are `Finished`.
    ///运行调度器直到所有线程（主线程除外）都变为 `Finished`。
    ///
    /// 1. Set the global `SCHEDULER` pointer to `self` so that `yield_now` and `thread_finished` can call back.
    ///1. 将全局 `SCHEDULER` 指针设置为 `self`，以便 `yield_now` 和 `thread_finished` 可以回调。
    /// 2. Loop: if all threads in `threads[1..]` are `Finished`, break; otherwise call `schedule_next()` (which may switch away and later return).
    ///2. 循环：如果 `threads[1..]` 中的所有线程都是 `Finished`，则跳出；否则调用 `schedule_next()`（可能会切换出去，稍后返回）。
    /// 3. Clear `SCHEDULER` when done.
    ///3. 完成后清除 `SCHEDULER`。
    pub fn run(&mut self) {
        //TODO
        todo!("set SCHEDULER to self, loop until threads[1..] all Finished, call schedule_next, then clear SCHEDULER")
    }

    /// Find the next ready thread (starting from `current + 1` round-robin), mark current as `Ready` (if not `Finished`), mark next as `Running`, set `CURRENT_THREAD_ENTRY` if the next thread has an entry, then switch to it.
    ///找到下一个就绪线程（从 `current + 1` 开始轮询），将当前线程标记为 `Ready`（如果尚未 `Finished`），将下一个线程标记为 `Running`，如果下一个线程有入口则设置 `CURRENT_THREAD_ENTRY`，然后切换到它。
    fn schedule_next(&mut self) {
        todo!("round-robin find next Ready, set current Ready (if not Finished), next Running, CURRENT_THREAD_ENTRY, then switch_context")
    }
}

impl TaskContext {
    fn as_mut_ptr(&mut self) -> *mut TaskContext {
        self as *mut TaskContext
    }
    fn as_ptr(&self) -> *const TaskContext {
        self as *const TaskContext
    }
}

static mut SCHEDULER: *mut Scheduler = std::ptr::null_mut();

/// Current thread voluntarily yields; the scheduler will pick the next ready thread.
///当前线程主动让出执行权；调度器将选择下一个就绪线程。
pub fn yield_now() {
    unsafe {
        if !SCHEDULER.is_null() {
            (*SCHEDULER).schedule_next();
        }
    }
}

/// Mark current thread as `Finished` and switch to the next (called by `thread_wrapper` after the user entry returns).
///将当前线程标记为 `Finished` 并切换到下一个（由 `thread_wrapper` 在用户入口返回后调用）。
fn thread_finished() {
    unsafe {
        if !SCHEDULER.is_null() {
            let sched = &mut *SCHEDULER;
            sched.threads[sched.current].state = ThreadState::Finished;
            sched.schedule_next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    /// Tests must run serially: the scheduler uses global state (SCHEDULER, CURRENT_THREAD_ENTRY).
    ///测试必须串行运行：调度器使用全局状态（SCHEDULER、CURRENT_THREAD_ENTRY）。
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    static EXEC_ORDER: AtomicU32 = AtomicU32::new(0);

    extern "C" fn task_a() {
        EXEC_ORDER.fetch_add(1, Ordering::SeqCst);
        yield_now();
        EXEC_ORDER.fetch_add(10, Ordering::SeqCst);
        yield_now();
        EXEC_ORDER.fetch_add(100, Ordering::SeqCst);
    }

    extern "C" fn task_b() {
        EXEC_ORDER.fetch_add(1, Ordering::SeqCst);
        yield_now();
        EXEC_ORDER.fetch_add(10, Ordering::SeqCst);
    }

    #[test]
    fn test_scheduler_runs_all() {
        let _guard = TEST_LOCK.lock().unwrap();
        EXEC_ORDER.store(0, Ordering::SeqCst);

        let mut sched = Scheduler::new();
        sched.spawn(task_a);
        sched.spawn(task_b);
        sched.run();

        let got = EXEC_ORDER.load(Ordering::SeqCst);
        if got != 122 {
            panic!(
                "EXEC_ORDER: expected 122, got {} (run with --nocapture to see stderr)",
                got
            );
        }
    }

    static SIMPLE_FLAG: AtomicU32 = AtomicU32::new(0);

    extern "C" fn simple_task() {
        SIMPLE_FLAG.store(42, Ordering::SeqCst);
    }

    #[test]
    fn test_single_thread() {
        let _guard = TEST_LOCK.lock().unwrap();
        SIMPLE_FLAG.store(0, Ordering::SeqCst);

        let mut sched = Scheduler::new();
        sched.spawn(simple_task);
        sched.run();

        assert_eq!(SIMPLE_FLAG.load(Ordering::SeqCst), 42);
    }
}
