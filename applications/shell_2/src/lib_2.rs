extern crate alloc;

use crate::{event::Event, history, job};

use crate::error::Result;
use alloc::sync::Arc;
use dfqueue::{DFQueue, DFQueueConsumer, DFQueueProducer};
use hashbrown::HashMap;
use keycodes_ascii::{KeyAction, KeyEvent, Keycode};
use log::{error, warn};
use mutex_sleep::MutexSleep as Mutex;
use stdio::{KeyEventQueue, KeyEventQueueReader, KeyEventQueueWriter};
use task::KillReason;

pub struct Shell<'a, A> {
    jobs: HashMap<usize, job::Job>,
    key_event_producer: KeyEventQueueWriter,
    key_event_consumer: Arc<Mutex<Option<KeyEventQueueReader>>>,
    foreground_job: Option<usize>,
    history: history::History,
    frontend: DFQueueProducer<Event>,
    input_buffer: fn(A) -> &'a str,
}

impl<'a, A> Shell<'a, A> {
    pub fn new(input_buffer: fn(A) -> &'a str) -> (Self, DFQueueConsumer<Event>) {
        let key_event_queue = KeyEventQueue::new();
        let key_event_producer = key_event_queue.get_writer();
        let key_event_consumer = Arc::new(Mutex::new(Some(key_event_queue.get_reader())));

        let frontend = DFQueue::new();
        let frontend_consumer = frontend.into_consumer();
        let frontend_producer = frontend_consumer.obtain_producer();

        (
            Self {
                jobs: HashMap::new(),
                key_event_producer,
                key_event_consumer,
                foreground_job: None,
                history: history::History::new(),
                frontend: frontend_producer,
                input_buffer,
            },
            frontend_consumer,
        )
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.action != KeyAction::Pressed {
            return Ok(());
        }

        let control = key_event.modifiers.is_control();
        let shift = key_event.modifiers.is_shift();

        match key_event.keycode {
            Keycode::C if control => {
                let foreground_job = if let Some(job) = self.foreground_job {
                    job
                } else {
                    self.frontend.enqueue(Event::CtrlC { clear: true });
                    return Ok(());
                };

                if let Some(job) = self.jobs.get(&foreground_job) {
                    let tasks = &job.tasks;
                    app_io::lock_and_execute(&|_fg, _sg| {
                        for task in tasks {
                            if task.rref.has_exited() {
                                continue;
                            }
                            match task.rref.kill(KillReason::Requested) {
                                Ok(_) => {
                                    if let Err(e) = runqueue::remove_task_from_all(&task.rref) {
                                        error!(
                                            "killed task but could not remove it from runqueue: {}",
                                            e
                                        );
                                    }
                                }
                                Err(e) => error!("could not kill task: {e}"),
                            }

                            loop {
                                scheduler::schedule();
                                if !task.rref.is_running() {
                                    break;
                                }
                            }
                        }
                    });
                    self.frontend.enqueue(Event::CtrlC { clear: false });
                    Ok(())
                } else {
                    warn!("foreground job not found in job map");
                    self.frontend.enqueue(Event::CtrlC { clear: true });
                    Ok(())
                }
            }
            Keycode::Z if control => {
                let foreground_job = if let Some(job) = self.foreground_job {
                    job
                } else {
                    return Ok(());
                };

                if let Some(job) = self.jobs.get(&foreground_job) {
                    let tasks = &job.tasks;
                    app_io::lock_and_execute(&|_fg, _sg| {
                        for task in tasks {
                            if task.rref.has_exited() {
                                continue;
                            }
                            task.rref.block();

                            loop {
                                scheduler::schedule();
                                if !task.rref.is_running() {
                                    break;
                                }
                            }
                        }
                    });
                    Ok(())
                } else {
                    warn!("foreground job not found in job map");
                    Ok(())
                }
            }
            Keycode::D if control => {
                if let Some(foreground_job) = self.foreground_job {
                    if let Some(job) = self.jobs.get(&foreground_job) {
                        job.stdin.lock().set_eof();
                    }
                }

                Ok(())
            }
            Keycode::Home if control => {
                self.frontend.enqueue(Event::ScreenBegin);
                Ok(())
            }
            Keycode::End if control => {
                self.frontend.enqueue(Event::ScreenEnd);
                Ok(())
            }
            Keycode::PageUp if shift => {
                self.frontend.enqueue(Event::PageUp);
                Ok(())
            }
            Keycode::PageDown if shift => {
                self.frontend.enqueue(Event::PageDown);
                Ok(())
            }
            Keycode::Up if control && shift => {
                self.frontend.enqueue(Event::LineUp);
                Ok(())
            }
            Keycode::Down if control && shift => {
                self.frontend.enqueue(Event::LineDown);
                Ok(())
            }
            Keycode::Tab => self.auto_complete(),
            Keycode::Backspace => {
                self.frontend.enqueue(Event::Backspace);
                Ok(())
            }
            Keycode::Delete => {
                self.frontend.enqueue(Event::Delete);
                Ok(())
            }
            Keycode::Enter => {
                if key_event.keycode.to_ascii(key_event.modifiers).is_some() {
                    todo!();
                } else {
                    todo!();
                }
            }
            Keycode::Home => {
                self.frontend.enqueue(Event::CursorLeftmost);
                Ok(())
            }
            Keycode::End => {
                self.frontend.enqueue(Event::CursorRightmost);
                Ok(())
            }
            Keycode::Up => {
                self.history.previous();
                todo!("enqueue event");
            }
            Keycode::Down => {
                self.history.next();
                todo!("enqueue event");
            }
            Keycode::Left => {
                self.frontend.enqueue(Event::CursorLeft);
                Ok(())
            }
            Keycode::Right => {
                self.frontend.enqueue(Event::CursorRight);
                Ok(())
            }
            _ => {
                todo!();
            }
        }
    }

    fn auto_complete(&mut self) -> Result<()> {
        todo!();
    }
}
