//! # Async Channel
//!
//! In this exercise, you will use `tokio::sync::mpsc` async channels to implement producer-consumer pattern.
//! 在本练习中，您将使用 `tokio::sync::mpsc` 异步通道来实现生产者-消费者模式。
//!
//! ## Concepts
//! - `tokio::sync::mpsc::channel` creates bounded async channels
//! - `tokio::sync::mpsc::channel` 创建有界异步通道
//! - Async `send` and `recv`
//! - 异步的 `send` 和 `recv`
//! - Channel closing mechanism (receiver returns None after all senders are dropped)
//! - 通道关闭机制（所有发送者被 drop 后，接收者返回 None）

use tokio::sync::mpsc::{self, Receiver};

/// Async producer-consumer:
/// 异步生产者-消费者：
/// - Create a producer task that sends each element from items sequentially
/// - 创建一个生产者任务，按顺序发送 items 中的每个元素
/// - Create a consumer task that receives all elements and collects them into Vec for return
/// - 创建一个消费者任务，接收所有元素并收集到 Vec 中返回
///
/// Hint: Set channel capacity to items.len().max(1)
/// 提示：将通道容量设置为 items.len().max(1)
pub async fn producer_consumer(items: Vec<String>) -> Vec<String> {
    // TODO: Create channel with mpsc::channel
    // TODO: 使用 mpsc::channel 创建通道
    // TODO: Spawn producer task: iterate through items, send each one
    // TODO: 派生生产者任务：遍历 items，发送每个元素
    // TODO: Spawn consumer task: loop recv until channel closes, collect results
    // TODO: 派生消费者任务：循环 recv 直到通道关闭，收集结果
    // TODO: Wait for consumer to complete and return results
    // TODO: 等待消费者完成并返回结果
    //todo!()
    let (sender,mut receiver) = mpsc::channel(items.len().max(1));
    let producer = tokio::spawn(async move {
        for item in items{
            sender.send(item).await.unwrap();
        }
    });
    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(item) = receiver.recv().await{
            results.push(item);
        }
        results
    });
    consumer.await.unwrap()
}

/// Fan‑in pattern: multiple producers, one consumer.
/// Fan‑in 模式：多个生产者，一个消费者。
/// Create `n_producers` producers, each sending `"producer {id}: message"`.
/// 创建 `n_producers` 个生产者，每个发送 `"producer {id}: message"`。
/// Consumer collects all messages, sorts them, and returns.
/// 消费者收集所有消息，对其排序后返回。
pub async fn fan_in(n_producers: usize) -> Vec<String> {
    // TODO: Create mpsc channel
    // TODO: 创建 mpsc 通道
    // TODO: Spawn n_producers producer tasks
    // TODO: 派生 n_producers 个生产者任务
    //       Each sends format!("producer {id}: message")
    //       每个发送 format!("producer {id}: message")
    // TODO: Drop the original sender (important! otherwise channel won't close)
    // TODO: Drop 原始发送者（这很重要！否则通道不会关闭）
    // TODO: Consumer loops receiving, collects and sorts
    // TODO: 消费者循环接收，收集并排序
    //todo!()
    let (sender,mut receiver) = mpsc::channel(n_producers);
    for i in 0..n_producers{
        let sender_clone = sender.clone();
        let producer = tokio::spawn(async move {
            let message =format!("producer {}: message", i);
            sender_clone.send(message).await.unwrap();
            
        });
    }
    drop(sender);
    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(item) = receiver.recv().await{
            results.push(item);
        }
        results
    });
    consumer.await.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_producer_consumer() {
        let items = vec!["hello".into(), "async".into(), "world".into()];
        let result = producer_consumer(items.clone()).await;
        assert_eq!(result, items);
    }

    #[tokio::test]
    async fn test_producer_consumer_empty() {
        let result = producer_consumer(vec![]).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_fan_in() {
        let result = fan_in(3).await;
        assert_eq!(
            result,
            vec![
                "producer 0: message",
                "producer 1: message",
                "producer 2: message",
            ]
        );
    }

    #[tokio::test]
    async fn test_fan_in_single() {
        let result = fan_in(1).await;
        assert_eq!(result, vec!["producer 0: message"]);
    }
}
