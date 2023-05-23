use crate::context::setjmp::adapter::{Context, GeneratorState};
use crate::now;
use once_cell::sync::Lazy;
use open_coroutine_queue::{LocalQueue, WorkStealQueue};
use std::time::Duration;
use uuid::Uuid;

/// 用户协程
pub type SchedulableCoroutine = Context<'static, usize>;

static QUEUE: Lazy<WorkStealQueue<SchedulableCoroutine>> = Lazy::new(WorkStealQueue::default);

#[derive(Debug)]
pub struct Scheduler {
    name: &'static str,
    ready: LocalQueue<'static, SchedulableCoroutine>,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        Self::with_name(Box::from(Uuid::new_v4().to_string()))
    }

    pub fn with_name(name: Box<str>) -> Self {
        Scheduler {
            name: Box::leak(name),
            ready: QUEUE.local_queue(),
        }
    }

    pub fn submit(&self, f: impl FnOnce(&Context<usize>) -> usize + 'static + 'static) {
        let coroutine = SchedulableCoroutine::new(f);
        self.ready.push_back(coroutine);
    }

    pub fn try_schedule(&self) {
        _ = self.try_timeout_schedule(Duration::MAX.as_secs());
    }

    pub fn try_timeout_schedule(&self, timeout_time: u64) -> u64 {
        loop {
            let left_time = timeout_time.saturating_sub(now());
            if left_time == 0 {
                return 0;
            }
            match self.ready.pop_front() {
                Some(coroutine) => {
                    match coroutine.resume() {
                        GeneratorState::Yielded => {
                            //放入就绪队列尾部
                            self.ready.push_back(coroutine);
                        }
                        GeneratorState::Complete(_) => {}
                    };
                }
                None => return left_time,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let scheduler = Scheduler::new();
        _ = scheduler.submit(|_| {
            println!("1");
            1
        });
        _ = scheduler.submit(|_| {
            println!("2");
            2
        });
        scheduler.try_schedule();
    }

    #[test]
    fn test_backtrace() {
        let scheduler = Scheduler::new();
        _ = scheduler.submit(|_| 1);
        _ = scheduler.submit(|_| {
            println!("{:?}", backtrace::Backtrace::new());
            2
        });
        scheduler.try_schedule();
    }

    #[test]
    fn with_suspend() {
        let scheduler = Scheduler::new();
        _ = scheduler.submit(|suspender| {
            println!("[coroutine1] suspend");
            suspender.suspend();
            println!("[coroutine1] back");
            1
        });
        // _ = scheduler.submit(|suspender| {
        //     println!("[coroutine2] suspend");
        //     suspender.suspend();
        //     println!("[coroutine2] back");
        //     2
        // });
        scheduler.try_schedule();
    }
}
