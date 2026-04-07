//! # Tokio Async Tasks
//!
//! In this exercise, you will use `tokio::spawn` to create concurrent asynchronous tasks.
//! 在本练习中，您将使用 `tokio::spawn` 创建并发的异步任务。
//!
//! ## Concepts
//! - `tokio::spawn` creates asynchronous tasks
//! - `tokio::spawn` 创建异步任务
//! - `JoinHandle` waits for task completion
//! - `JoinHandle` 等待任务完成
//! - Concurrent execution between asynchronous tasks
//! - 异步任务之间的并发执行

use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

/// Concurrently compute the square of each number in 0..n, collect results and return in order.
/// 并发计算 0..n 中每个数字的平方，收集结果并按顺序返回。
///
/// Hint: Create `tokio::spawn` task for each i, collect JoinHandle, await them sequentially.
/// 提示：为每个 i 创建 `tokio::spawn` 任务，收集 JoinHandle，按顺序 await 它们。
pub async fn concurrent_squares(n: usize) -> Vec<usize> {
    // TODO: Create n asynchronous tasks, each computing i * i
    // TODO: 创建 n 个异步任务，每个任务计算 i * i
    // TODO: Collect all JoinHandle
    // TODO: 收集所有的 JoinHandle
    // TODO: Await each one to get result
    // TODO: 依次 await 每个任务以获取结果
    //todo!()
    let mut handles = Vec::new();
    for i in 0..n{
        handles.push(tokio::spawn(async move {
            i * i
        }));
    }
    let mut results = Vec::new();
    for handle in handles{
        results.push(handle.await.unwrap());
    }
    results.sort();
    results
}

/// Concurrently execute multiple "time-consuming" tasks (simulated with sleep), return all results.
/// 并发执行多个"耗时的"任务（用 sleep 模拟），返回所有结果。
/// Each task sleeps `duration_ms` milliseconds and then returns its `task_id`.
/// 每个任务 sleep `duration_ms` 毫秒后返回其 `task_id`。
///
/// Key: All tasks should execute concurrently, total duration should be close to single task duration, not sum of all tasks.
/// 关键：所有任务应并发执行，总耗时应接近单个任务的耗时，而非所有任务耗时之和。
pub async fn parallel_sleep_tasks(n: usize, duration_ms: u64) -> Vec<usize> {
    // TODO: Create asynchronous task for each id in 0..n
    // TODO: 为 0..n 中的每个 id 创建异步任务
    // TODO: Each task sleeps specified duration and returns its own id
    // TODO: 每个任务 sleep 指定的时长并返回自己的 id
    // TODO: Collect all results and sort
    // TODO: 收集所有结果并排序
    //todo!()
    let mut handles = Vec::new();
    for i in 0..n{
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(duration_ms)).await;
            i
        }));
    }
    let mut results = Vec::new();
    for handle in handles{
        results.push(handle.await.unwrap());
    }
    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_squares_basic() {
        let result = concurrent_squares(5).await;
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[tokio::test]
    async fn test_squares_zero() {
        let result = concurrent_squares(0).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_squares_one() {
        let result = concurrent_squares(1).await;
        assert_eq!(result, vec![0]);
    }

    #[tokio::test]
    async fn test_parallel_sleep() {
        let start = Instant::now();
        let result = parallel_sleep_tasks(5, 100).await;
        let elapsed = start.elapsed();

        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        // Concurrent execution, total time should be much less than 5 * 100ms
        // 并发执行，总耗时应远小于 5 * 100ms
        assert!(
            elapsed.as_millis() < 400,
            "Tasks should run concurrently, took {}ms",
            elapsed.as_millis()
        );
    }
}
