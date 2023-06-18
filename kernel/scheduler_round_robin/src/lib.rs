//! This crate picks the next task in round robin fashion.
//! Each time the task at the front of the queue is picked.
//! This task is then moved to the back of the queue.

#![no_std]

extern crate alloc;

use log::error;
use runqueue_round_robin::RunQueue;
use task::TaskRef;

/// This defines the round robin scheduler policy.
/// Returns None if there is no schedule-able task
pub fn select_next_task(apic_id: u8) -> Option<TaskRef> {
    let mut runqueue_locked = match RunQueue::get_runqueue(apic_id) {
        Some(rq) => rq.write(),
        _ => {
            error!("BUG: select_next_task_round_robin(): couldn't get runqueue for core {apic_id}");
            return None;
        }
    };

    let mut idle_task_index: Option<usize> = None;
    let mut chosen_task_index: Option<usize> = None;

    for (i, t) in runqueue_locked.iter().enumerate() {
        // we skip the idle task, and only choose it if no other tasks are runnable
        if t.is_an_idle_task {
            idle_task_index = Some(i);
            continue;
        }

        // must be runnable
        if !t.is_runnable() {
            continue;
        }

        // found a runnable task!
        chosen_task_index = Some(i);
        // debug!("select_next_task(): AP {} chose Task {:?}", apic_id, &*t);
        break;
    }

    // idle task is a backup iff no other task has been chosen
    chosen_task_index
        .or(idle_task_index)
        .and_then(|index| runqueue_locked.move_to_end(index))
}

pub fn set_priority(_: &TaskRef, _: u8) -> Result<(), &'static str> {
    Err("")
}

pub fn get_priority(_task: &TaskRef) -> Option<u8> {
    None
}
