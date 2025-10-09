//! Future support.

use alloc::{sync::Arc, task::Wake};
use core::{
    fmt,
    pin::{Pin, pin},
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};

use axerrno::AxError;
use kernel_guard::NoPreemptIrqSave;
use pin_project::pin_project;

use crate::{AxTaskRef, WeakAxTaskRef, current, current_run_queue, select_run_queue};

#[cfg(feature = "irq")]
mod time;
#[cfg(feature = "irq")]
#[doc(cfg(all(feature = "multitask", feature = "irq")))]
pub use time::*;

struct AxWaker {
    task: WeakAxTaskRef,
    woke: AtomicBool,
}

impl Wake for AxWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(task) = self.task.upgrade() {
            self.woke.store(true, Ordering::Release);
            select_run_queue::<NoPreemptIrqSave>(&task).unblock_task(task, false);
        }
    }
}

/// Blocks the current task until the given future is resolved.
///
/// Note that this doesn't handle interruption and is not recommended for direct
/// use in most cases.
pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
    let mut fut = pin!(f.into_future());

    let curr = current();
    // It's necessary to keep a strong reference to the current task
    // to prevent it from being dropped while blocking.
    let task = curr.clone();

    let waker = Arc::new(AxWaker {
        task: Arc::downgrade(&task),
        woke: AtomicBool::new(false),
    });
    let woke = &waker.woke;
    let waker = Waker::from(waker.clone());
    let mut cx = Context::from_waker(&waker);

    loop {
        woke.store(false, Ordering::Release);
        match fut.as_mut().poll(&mut cx) {
            Poll::Pending => {
                if !woke.load(Ordering::Acquire) {
                    current_run_queue::<NoPreemptIrqSave>().blocked_resched(|_| {});
                } else {
                    // Immediately woken
                    crate::yield_now();
                }
            }
            Poll::Ready(output) => break output,
        }
    }
}

/// Error returned by [`interruptible`].
#[derive(Debug, PartialEq, Eq)]
pub struct Interrupted;

impl fmt::Display for Interrupted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interrupted")
    }
}

impl core::error::Error for Interrupted {}

impl From<Interrupted> for AxError {
    fn from(_: Interrupted) -> Self {
        AxError::Interrupted
    }
}

/// Future returned by [`interruptible`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project]
pub struct Interruptible<F> {
    #[pin]
    inner: F,
    task: AxTaskRef,
}

impl<F> Future for Interruptible<F>
where
    F: Future,
{
    type Output = Result<F::Output, Interrupted>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.task.interrupted() {
            return Poll::Ready(Err(Interrupted));
        } else {
            this.task.on_interrupt(cx.waker());
        }

        this.inner.poll(cx).map(Ok)
    }
}

/// Makes a future interruptible.
pub fn interruptible<F: IntoFuture>(f: F) -> Interruptible<F::IntoFuture> {
    Interruptible {
        inner: f.into_future(),
        task: current().clone(),
    }
}
