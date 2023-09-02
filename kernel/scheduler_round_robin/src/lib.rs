//! This crate picks the next task in round robin fashion.
//! Each time the task at the front of the queue is picked.
//! This task is then moved to the back of the queue.

#![no_std]

extern crate alloc;

use alloc::collections::VecDeque;

use task::TaskRef;

pub struct RoundRobinScheduler {
    idle_task: TaskRef,
    // TODO: Use regular Vec.
    queue: VecDeque<TaskRef>,
}

impl RoundRobinScheduler {
    pub const fn new(idle_task: TaskRef) -> Self {
        Self {
            idle_task,
            queue: VecDeque::new(),
        }
    }
}

impl task::scheduler_2::Scheduler for RoundRobinScheduler {
    fn next(&mut self) -> TaskRef {
        if let Some((task_index, _)) = self
            .queue
            .iter()
            .enumerate()
            .find(|(_, task)| task.is_runnable())
        {
            let task = self.queue.swap_remove_front(task_index).unwrap();
            self.queue.push_back(task.clone());
            task
        } else {
            self.idle_task.clone()
        }
    }

    fn busyness(&self) -> usize {
        self.queue.len()
    }

    fn push(&mut self, task: TaskRef) {
        self.queue.push_back(task);
    }

    fn remove(&mut self, task: &TaskRef) -> bool {
        let mut task_index = None;
        for (i, t) in self.queue.iter().enumerate() {
            if t == task {
                task_index = Some(i);
                break;
            }
        }

        if let Some(task_index) = task_index {
            self.queue.remove(task_index);
            true
        } else {
            false
        }
    }

    fn as_priority_scheduler(&mut self) -> Option<&mut dyn task::scheduler_2::PriorityScheduler> {
        None
    }
}
