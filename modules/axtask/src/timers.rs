<<<<<<< HEAD
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    cmp::Reverse,
    hash::{Hash, Hasher},
};

use axhal::time::{TimeValue, wall_time};
use foldhash::fast::RandomState;
use kernel_guard::{NoOp, NoPreemptIrqSave};
use kspin::{SpinNoIrq, SpinRaw};
use priority_queue::PriorityQueue;
use spin::Lazy;

use crate::{AxTaskRef, WeakAxTaskRef, select_run_queue};

static TIMER_CALLBACKS: SpinRaw<Vec<Box<dyn Fn(TimeValue) + Send + Sync>>> =
    SpinRaw::new(Vec::new());

=======
use alloc::{boxed::Box, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};

use kernel_guard::{NoOp, NoPreemptIrqSave};
use kspin::{SpinNoIrq, SpinRaw};
use weak_map::WeakMap;

use axhal::time::{TimeValue, wall_time};

use crate::{AxTaskRef, WeakAxTaskRef, select_run_queue};

type TimerCb = Box<dyn Fn(TimeValue) + Send + Sync>;

static TIMER_CALLBACKS: SpinRaw<Vec<TimerCb>> = SpinRaw::new(Vec::new());

/// Registers a callback function to be called on each timer tick.
>>>>>>> arceos-c53fb41
pub fn register_timer_callback<F>(callback: F)
where
    F: Fn(TimeValue) + Send + Sync + 'static,
{
    let _g = NoPreemptIrqSave::new();
    TIMER_CALLBACKS.lock().push(Box::new(callback));
}

<<<<<<< HEAD
struct TaskPtr(WeakAxTaskRef);

impl TaskPtr {
    fn new(task: &AxTaskRef) -> Self {
        TaskPtr(Arc::downgrade(task))
    }

    fn upgrade(&self) -> Option<AxTaskRef> {
        self.0.upgrade()
=======
static TIMER_KEY: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TimerKey {
    deadline: TimeValue,
    key: u64,
}

static TIMER_WHEEL: SpinNoIrq<WeakMap<TimerKey, WeakAxTaskRef>> = SpinNoIrq::new(WeakMap::new());

pub(crate) fn set_timer(deadline: TimeValue, task: &AxTaskRef) -> Option<TimerKey> {
    if deadline <= wall_time() {
        return None;
>>>>>>> arceos-c53fb41
    }

    let mut wheel = TIMER_WHEEL.lock();
    let key = TimerKey {
        deadline,
        key: TIMER_KEY.fetch_add(1, Ordering::AcqRel),
    };
    wheel.insert(key, task);

    Some(key)
}

<<<<<<< HEAD
impl PartialEq for TaskPtr {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for TaskPtr {}

impl Hash for TaskPtr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

static TIMER_LIST: Lazy<SpinNoIrq<PriorityQueue<TaskPtr, Reverse<TimeValue>, RandomState>>> =
    Lazy::new(|| SpinNoIrq::new(PriorityQueue::with_default_hasher()));

pub fn set_alarm_wakeup(deadline: TimeValue, task: &AxTaskRef) {
    TIMER_LIST
        .lock()
        .push(TaskPtr::new(task), Reverse(deadline));
}

pub fn clear_alarm_wakeup(task: &AxTaskRef) {
    TIMER_LIST.lock().remove(&TaskPtr::new(task));
}

pub fn check_events() {
    let now = wall_time();
    for callback in TIMER_CALLBACKS.lock().iter() {
        callback(now);
    }
    let mut timer_list = TIMER_LIST.lock();
    while let Some((task_ptr, _)) = timer_list.pop_if(|_, Reverse(deadline)| *deadline < now) {
        if let Some(task) = task_ptr.upgrade() {
            select_run_queue::<NoOp>(&task).unblock_task(task, true);
=======
pub(crate) fn cancel_timer(key: &TimerKey) {
    let mut wheel = TIMER_WHEEL.lock();
    wheel.remove(key);
}

pub(crate) fn has_timer(key: &TimerKey) -> bool {
    TIMER_WHEEL.lock().contains_key(key)
}

pub fn check_events() {
    for callback in TIMER_CALLBACKS.lock().iter() {
        callback(wall_time());
    }

    let mut wheel = TIMER_WHEEL.lock();
    for (key, maybe_task) in &mut *wheel {
        if key.deadline <= wall_time() {
            if let Some(task) = maybe_task.upgrade() {
                select_run_queue::<NoOp>(&task).unblock_task(task, true);
                core::mem::take(maybe_task);
            }
        } else {
            break;
>>>>>>> arceos-c53fb41
        }
    }
}
