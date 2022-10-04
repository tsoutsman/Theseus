#![no_std]

extern crate alloc;

mod error;
mod event;
mod frontend;
mod history;
mod internal;
mod job;

pub use error::{Error, Result};
pub use event::{Event, KeyboardEvent};
pub use frontend::{Cursor, Frontend, Input, Rectangle};

use alloc::{borrow::ToOwned, format, string::String, sync::Arc};
use core2::io::Write;
use hashbrown::HashMap;
use job::Job;
use log::{error, warn};
use mutex_sleep::MutexSleep as Mutex;
use stdio::{KeyEventQueue, KeyEventQueueReader, KeyEventQueueWriter};
use task::KillReason;

pub struct Shell<T>
where
    T: crate::frontend::Frontend,
{
    pub(crate) frontend: T,
    pub(crate) input_buf: String,
    pub(crate) jobs: HashMap<usize, Job>,
    pub(crate) key_event_producer: KeyEventQueueWriter,
    pub(crate) key_event_consumer: Arc<Mutex<Option<KeyEventQueueReader>>>,
    pub(crate) foreground_job: Option<usize>,
    pub(crate) history: history::History,
    pub(crate) input: T::Input,
}

impl<T> Shell<T>
where
    T: Frontend,
{
    pub fn new(frontend: T, input: T::Input) -> Self {
        let key_event_queue = KeyEventQueue::new();
        let key_event_producer = key_event_queue.get_writer();
        let key_event_consumer = Arc::new(Mutex::new(Some(key_event_queue.get_reader())));

        Self {
            jobs: HashMap::new(),
            key_event_producer,
            key_event_consumer,
            foreground_job: None,
            history: history::History::new(),
            input_buf: String::new(),
            frontend,
            input,
        }
    }

    pub(crate) fn set_input(&mut self, string: String) {
        if !self.input_buf.is_empty() {
            self.clear_input();
        }
        self.frontend.push_str(&string);
        self.input_buf = string;
        self.frontend.cursor_mut().leftmost();
    }

    pub(crate) fn clear_input(&mut self) {
        self.frontend.cursor_mut().rightmost();
        for _ in 0..self.input_buf.len() {
            self.frontend.pop(false);
        }
        self.input_buf.clear();
    }

    pub(crate) fn push(&mut self, c: char) {
        let offset = self.frontend.cursor().offset();
        self.input_buf.insert(offset, c);
        self.frontend.push(c);
    }

    pub(crate) fn pop(&mut self, in_front: bool) {
        let mut offset = self.frontend.cursor().offset();
        if in_front {
            offset += 1;
        }
        self.input_buf.remove(offset);
        self.frontend.pop(in_front);
    }

    pub(crate) fn display_prompt(&mut self) {
        let env = task::get_my_current_task().unwrap().get_env();
        let prompt = format!("{}: ", env.lock().working_dir.lock().get_absolute_path());
        self.frontend.push_str(&prompt);
        self.frontend.push_str(&self.input_buf);
    }

    fn handle_key_event(&mut self, key_event: KeyboardEvent) -> Result<()> {
        match key_event {
            KeyboardEvent::CtrlC => {
                let foreground_job = if let Some(job) = self.foreground_job {
                    job
                } else {
                    self.clear_input();
                    self.frontend.push_str("^C\n");
                    self.history.reset();
                    self.display_prompt();
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
                                            "killed task but could not pop it from runqueue: {}",
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
                    self.frontend.push_str("^C\n");
                    Ok(())
                } else {
                    warn!("foreground job not found in job map");

                    self.clear_input();
                    self.frontend.push_str("^C\n");
                    self.display_prompt();

                    Ok(())
                }
            }
            KeyboardEvent::CtrlZ => {
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
                    error!("foreground job not found in job map");
                    Ok(())
                }
            }
            KeyboardEvent::CtrlD => {
                if let Some(foreground_job) = self.foreground_job {
                    if let Some(job) = self.jobs.get(&foreground_job) {
                        job.stdin.lock().set_eof();
                    }
                }
                Ok(())
            }
            KeyboardEvent::Begin => {
                self.frontend.to_begin();
                Ok(())
            }
            KeyboardEvent::End => {
                self.frontend.to_end();
                Ok(())
            }
            KeyboardEvent::PageUp => {
                self.frontend.page_up();
                Ok(())
            }
            KeyboardEvent::PageDown => {
                self.frontend.page_down();
                Ok(())
            }
            KeyboardEvent::LineUp => {
                self.frontend.line_up();
                Ok(())
            }
            KeyboardEvent::LineDown => {
                self.frontend.line_down();
                Ok(())
            }
            KeyboardEvent::Tab => self.auto_complete(),
            KeyboardEvent::Backspace => {
                self.pop(false);
                Ok(())
            }
            KeyboardEvent::Delete => {
                self.pop(true);
                Ok(())
            }
            KeyboardEvent::Enter => {
                self.handle_enter()
            }
            KeyboardEvent::Leftmost => {
                self.frontend.cursor_mut().leftmost();
                Ok(())
            }
            KeyboardEvent::Rightmost => {
                self.frontend.cursor_mut().rightmost();
                Ok(())
            }
            KeyboardEvent::Up => {
                if let Some(cmd) = self.history.previous(&self.input_buf) {
                    let cmd = cmd.to_owned();
                    self.set_input(cmd);
                }
                Ok(())
            }
            KeyboardEvent::Down => {
                if let Some(cmd) = self.history.next() {
                    let cmd = cmd.to_owned();
                    self.set_input(cmd);
                }
                Ok(())
            }
            KeyboardEvent::Left => {
                let offset = self.frontend.cursor().offset();
                if offset > 0 {
                    self.frontend.cursor_mut().left();
                }
                Ok(())
            }
            KeyboardEvent::Right => {
                let offset = self.frontend.cursor().offset();
                if offset < self.input_buf.len() {
                    self.frontend.cursor_mut().right();
                }
                Ok(())
            }
            KeyboardEvent::Other(c) => {
                self.push(c);
                Ok(())
            }
        }
    }

    fn handle_enter(&mut self) -> Result<()> {
        if let Some(foreground_job) = self.foreground_job {
            if let Some(job) = self.jobs.get(&foreground_job) {
                self.frontend.push('\n');
                let mut buf = String::new();
                core::mem::swap(&mut buf, &mut self.input_buf);
                buf.push('\n');
                job.stdin.lock().write_all(buf.as_bytes()).or(Err(Error::Io))
            } else {
                error!("foreground job not found in job map");
                Ok(())
            }
        } else {
            if self.input_buf.is_empty() {
                self.frontend.push('\n');
                self.display_prompt();
            } else {
                self.frontend.push('\n');
                // TODO: Push history after to avoid unescessary clone.
                self.history.push(self.input_buf.clone());

                if self.try_execute_internal() {
                    self.clear_input();
                } else {
                    let job_num = self.new_job();
                    if let Some("&") = self.input_buf.split_whitespace().last() {
                        self.frontend
                            .push_str(&format!("[{}] [running] {}\n", job_num, self.input_buf));
                        self.foreground_job = None;
                        self.clear_input();
                        self.display_prompt();
                    } else {
                        self.foreground_job = Some(job_num);
                    }
                }
            }
            Ok(())
        }
    }

    fn auto_complete(&mut self) -> Result<()> {
        todo!();
    }

    fn new_job(&mut self) -> usize {
        todo!();
    }

    pub fn start(mut self) -> Result<()> {
        self.display_prompt();
        self.frontend.refresh();

        loop {
            match self.input.event() {
                Some(event) => match event {
                    Event::Keyboard(event) => self.handle_key_event(event)?,
                    Event::Exit => break,
                },
                None => continue,
            }
            // FIXME: Do stuff.

            // if let Some(ref key_event_consumer) =
            // self.key_event_consumer.lock() {     loop {
            //         if let Some(key_event) = key_event_consumer.read_one() {
            //             self.handle_key_event(key_event);
            //         } else {
            //             break;
            //         }
            //     }
            // }

            // if need_refresh {
            //     self.frontend.refresh();
            // } else {
            //     scheduler::schedule();
            // }
        }

        Ok(())
    }
}
