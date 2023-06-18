//! This scheduler implements the Rate Monotonic Scheduling algorithm.
//!
//! Because the [`runqueue_realtime::RunQueue`] internally sorts the tasks
//! in increasing order of periodicity, it's trivially easy to choose the next
//! task.

#![no_std]

extern crate alloc;

use log::error;
use runqueue_priority::RunQueue;
use task::TaskRef;

/// This defines the realtime scheduler policy.
/// Returns None if there is no schedule-able task
pub fn select_next_task(apic_id: u8) -> Option<TaskRef> {
    let mut runqueue_locked = match RunQueue::get_runqueue(apic_id) {
        Some(rq) => rq.write(),
        _ => {
            error!(
                "BUG: select_next_task_round_robin(): couldn't get runqueue for core {}",
                apic_id
            );
            return None;
        }
    };

    let mut idle_task_index: Option<usize> = None;
    let mut chosen_task_index: Option<usize> = None;

    for (i, taskref) in runqueue_locked.iter().enumerate() {
        let t = taskref;

        // we skip the idle task, and only choose it if no other tasks are runnable
        if t.is_an_idle_task {
            idle_task_index = Some(i);
            continue;
        }

        // must be runnable
        if !t.is_runnable() {
            continue;
        }

        // found a runnable task
        chosen_task_index = Some(i);
        break;
    }

    // idle task is backup iff no other task has been chosen
    chosen_task_index
        .or(idle_task_index)
        .and_then(|index| runqueue_locked.update_and_reinsert(index))
}

pub fn set_priority(_: &TaskRef, _: u8) -> Result<(), &'static str> {
    Err("")
}

pub fn get_priority(_task: &TaskRef) -> Option<u8> {
    None
}
