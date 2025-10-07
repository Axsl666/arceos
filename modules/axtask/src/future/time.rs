use core::{
    fmt,
    pin::{Pin, pin},
    task::{Context, Poll},
    time::Duration,
};

use axerrno::AxError;
use axhal::time::TimeValue;
use futures_util::FutureExt;
use pin_project::pin_project;

use crate::{
    current,
    timers::{TimerKey, cancel_timer, has_timer, set_timer},
};

/// Future returned by `sleep` and `sleep_until`.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct TimerFuture(Option<TimerKey>);

impl TimerFuture {
    fn new(deadline: Option<Duration>) -> Self {
        if let Some(deadline) = deadline {
            let key = set_timer(deadline, &current());
            debug!("TimerFuture created key={:?}", key);
            Self(key)
        } else {
            Self(None)
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(key) = self.0
            && !has_timer(&key)
        {
            debug!("TimerFuture fired key={:?}", key);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for TimerFuture {
    fn drop(&mut self) {
        if let Some(key) = self.0.take() {
            cancel_timer(&key);
        }
    }
}

/// Waits until `duration` has elapsed.
pub fn sleep(duration: Duration) -> TimerFuture {
    sleep_until(axhal::time::wall_time() + duration)
}

/// Waits until `deadline` is reached.
pub fn sleep_until(deadline: TimeValue) -> TimerFuture {
    TimerFuture::new(Some(deadline))
}

/// Error returned by [`timeout`] and [`timeout_at`].
#[derive(Debug, PartialEq, Eq)]
pub struct Elapsed;

impl fmt::Display for Elapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "deadline elapsed")
    }
}

impl core::error::Error for Elapsed {}

impl From<Elapsed> for AxError {
    fn from(_: Elapsed) -> Self {
        AxError::TimedOut
    }
}

/// Future returned by [`timeout`] and [`timeout_at`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project]
pub struct Timeout<F> {
    #[pin]
    inner: F,
    delay: TimerFuture,
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, Elapsed>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Poll::Ready(()) = this.delay.poll_unpin(cx) {
            return Poll::Ready(Err(Elapsed));
        }

        this.inner.poll(cx).map(Ok)
    }
}

/// Requires a `Future` to complete before the specified duration has elapsed.
pub fn timeout<F: IntoFuture>(duration: Option<Duration>, f: F) -> Timeout<F::IntoFuture> {
    timeout_at(
        duration.and_then(|x| x.checked_add(axhal::time::wall_time())),
        f,
    )
}

/// Requires a `Future` to complete before the specified deadline.
pub fn timeout_at<F: IntoFuture>(deadline: Option<TimeValue>, f: F) -> Timeout<F::IntoFuture> {
    Timeout {
        inner: f.into_future(),
        delay: TimerFuture::new(deadline),
    }
}
