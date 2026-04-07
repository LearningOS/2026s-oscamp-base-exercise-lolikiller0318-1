//! # Manual Future Implementation
//! 手动 Future 实现

//! 在本练习中，你将手动为自定义类型实现 `Future` trait，以理解异步运行时的核心机制。
//! ## Concepts
//! - `std::future::Future` trait
//! - `Poll::Ready` and `Poll::Pending`
//! - The role of `Waker`: notifying the runtime to poll again
//! - `Waker` 的作用：通知运行时再次轮询
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Countdown Future: decrements count by 1 each time it's polled,
/// returns `"liftoff!"` when count reaches 0.
/// 倒计时 Future：每次轮询时将计数减 1，
/// 当计数达到 0 时返回 `"liftoff!"`。
pub struct CountDown {
    pub count: u32,
}

impl CountDown {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

// TODO: Implement Future trait for CountDown
// TODO: 为 CountDown 实现 Future trait
// - Output type is &'static str
// - Output 类型为 &'static str
// - Each poll: if count == 0, return Poll::Ready("liftoff!")
// - 每次轮询时：如果 count == 0，返回 Poll::Ready("liftoff!")
// - Otherwise count -= 1, call cx.waker().wake_by_ref(), return Poll::Pending
// - 否则 count -= 1，调用 cx.waker().wake_by_ref()，返回 Poll::Pending
//
// Hint: Use `self.get_mut()` to get `&mut Self` (since self is Pin<&mut Self>)
// 提示：使用 `self.get_mut()` 获取 `&mut Self`（因为 self 是 Pin<&mut Self>）
impl Future for CountDown {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //todo!()
        let pin = self.get_mut();
        if pin.count == 0{
            Poll::Ready("liftoff!")
        }else {
            pin.count-=1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Yield-only-once Future: first poll returns Pending, second returns Ready(()).
/// This is the minimal example of an asynchronous state machine.
/// Yield-only-once Future：首次轮询返回 Pending，再次轮询返回 Ready(())。
/// 这是异步状态机的最小实现示例。
pub struct YieldOnce {
    yielded: bool,
}

impl YieldOnce {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

// TODO: Implement Future trait for YieldOnce
// TODO: 为 YieldOnce 实现 Future trait
// - Output type is ()
// - Output 类型为 ()
// - First poll: set yielded = true, wake waker, return Pending
// - 首次轮询：设置 yielded = true，唤醒 waker，返回 Pending
// - Second poll: return Ready(())
// - 再次轮询：返回 Ready(())
impl Future for YieldOnce {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        //todo!()
        let pin = self.get_mut();
        if pin.yielded{
            Poll::Ready(())
        }else {
            pin.yielded=true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_countdown_zero() {
        let result = CountDown::new(0).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_countdown_three() {
        let result = CountDown::new(3).await;
        assert_eq!(result, "liftoff!");
    }

    #[tokio::test]
    async fn test_yield_once() {
        YieldOnce::new().await;
    }

    #[tokio::test]
    async fn test_countdown_large() {
        let result = CountDown::new(100).await;
        assert_eq!(result, "liftoff!");
    }
}
