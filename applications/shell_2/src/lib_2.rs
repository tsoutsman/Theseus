extern crate alloc;

use crate::{
    frontend::{Cursor, Frontend},
    history,
    job::{Job, Status},
};

use crate::error::{Error, Result};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use core2::io::Write;
use hashbrown::HashMap;
use keycodes_ascii::{KeyAction, KeyEvent, Keycode};
use log::{error, warn};
use mutex_sleep::MutexSleep as Mutex;
use stdio::{KeyEventQueue, KeyEventQueueReader, KeyEventQueueWriter};
use task::KillReason;

pub struct Shell<T>
where
    T: crate::frontend::Frontend,
{
    jobs: HashMap<usize, Job>,
    key_event_producer: KeyEventQueueWriter,
    key_event_consumer: Arc<Mutex<Option<KeyEventQueueReader>>>,
    foreground_job: Option<usize>,
    history: history::History,
    input_buf: String,
    frontend: T,
}

impl<T> Shell<T>
where
    T: Frontend,
{
    pub fn new(frontend: T) -> Self {
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
        }
    }

    fn set_input(&mut self, string: String) {
        if !self.input_buf.is_empty() {
            self.clear_input();
        }
        self.frontend.print(&string);
        self.input_buf = string;
        self.frontend.cursor_mut().set_offset(0);
    }

    fn clear_input(&mut self) {
        for _ in 0..self.input_buf.len() {
            self.frontend.remove_char(1);
        }
        self.input_buf.clear();
        self.frontend.cursor_mut().set_offset(0);
    }

    fn push_to_input(&mut self, c: char) {
        self.input_buf.push(c);
        self.frontend.insert_char(c, 0);
    }

    fn pop_from_input(&mut self) -> Option<char> {
        todo!();
    }

    fn display_prompt(&mut self) {
        let env = task::get_my_current_task().unwrap().get_env();
        let prompt = format!("{}: ", env.lock().working_dir.lock().get_absolute_path());
        self.frontend.print(&prompt);
        todo!("print cmdline?");
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
                    self.clear_input();
                    self.frontend.print("^C\n");
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
                    self.frontend.print("^C\n");
                    Ok(())
                } else {
                    warn!("foreground job not found in job map");

                    self.clear_input();
                    self.frontend.print("^C\n");
                    self.display_prompt();

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
                    error!("foreground job not found in job map");
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
                self.frontend.to_begin();
                Ok(())
            }
            Keycode::End if control => {
                self.frontend.to_end();
                Ok(())
            }
            Keycode::PageUp if shift => {
                self.frontend.page_up();
                Ok(())
            }
            Keycode::PageDown if shift => {
                self.frontend.page_down();
                Ok(())
            }
            Keycode::Up if control && shift => {
                self.frontend.line_up();
                Ok(())
            }
            Keycode::Down if control && shift => {
                self.frontend.line_down();
                Ok(())
            }
            Keycode::Tab => self.auto_complete(),
            Keycode::Backspace => {
                let offset = self.frontend.cursor().offset() + 1;
                if offset > self.input_buf.len() {
                    Ok(())
                } else {
                    self.frontend.remove_char(offset);
                    Ok(())
                }
            }
            Keycode::Delete => {
                let offset = self.frontend.cursor().offset();
                if offset > 0 {
                    self.frontend.remove_char(offset);
                    self.frontend.cursor_mut().set_offset(offset - 1);
                }
                Ok(())
            }
            Keycode::Enter => {
                // TODO: What does this if statement mean?
                if key_event.keycode.to_ascii(key_event.modifiers).is_some() {
                    todo!();
                } else {
                    todo!();
                }
            }
            Keycode::Home => {
                self.frontend.cursor_mut().set_offset(self.input_buf.len());
                Ok(())
            }
            Keycode::End => {
                self.frontend.cursor_mut().set_offset(0);
                Ok(())
            }
            Keycode::Up => {
                if let Some(cmd) = self.history.previous(&self.input_buf) {
                    let cmd = cmd.to_owned();
                    self.set_input(cmd);
                }
                Ok(())
            }
            Keycode::Down => {
                if let Some(cmd) = self.history.next() {
                    let cmd = cmd.to_owned();
                    self.set_input(cmd);
                }
                Ok(())
            }
            Keycode::Left => {
                let offset = self.frontend.cursor().offset();
                if offset < self.input_buf.len() {
                    self.frontend.cursor_mut().set_offset(offset + 1);
                }
                Ok(())
            }
            Keycode::Right => {
                let offset = self.frontend.cursor().offset();
                if offset > 0 {
                    self.frontend.cursor_mut().set_offset(offset - 1);
                }
                self.frontend.cursor_mut().enable();
                Ok(())
            }
            _ => {
                if let Some(c) = key_event.keycode.to_ascii(key_event.modifiers) {
                    self.push_to_input(c);
                }
                Ok(())
            }
        }
    }

    fn handle_enter(&mut self) -> Result<()> {
        if let Some(foreground_job) = self.foreground_job {
            if let Some(job) = self.jobs.get(&foreground_job) {
                self.frontend.print("\n");
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
                self.frontend.print("\n");
                self.display_prompt();
            } else {
                self.frontend.print("\n");
                // TODO: Push history after to avoid unescessary clone.
                self.history.push(self.input_buf.clone());

                if self.try_execute_internal() {
                    self.clear_input();
                } else {
                    let job_num = self.new_job();
                    if let Some("&") = self.input_buf.split_whitespace().last() {
                        self.frontend
                            .print(&format!("[{}] [running] {}\n", job_num, self.input_buf));
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

    fn start(mut self) -> Result<()> {
        self.display_prompt();
        self.frontend.refresh();

        loop {
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
    }
}

/// Internal commands.
impl<T> Shell<T>
where
    T: Frontend,
{
    fn try_execute_internal(&mut self) -> bool {
        if let Some(cmd) = self.input_buf.split_whitespace().next() {
            match cmd {
                "jobs" => self.execute_internal_jobs(),
                "fg" => self.execute_internal_fg(),
                "bg" => self.execute_internal_bg(),
                "clear" => self.execute_internal_clear(),
                _ => return false,
            };
            true
        } else {
            error!("called try_execute_internal without a command");
            false
        }
    }

    fn execute_internal_jobs(&mut self) {
        if self.jobs.is_empty() {
            self.frontend.print("No running or stopped jobs.\n");
        } else {
            for (num, rref) in self.jobs.iter() {
                let status = match rref.status {
                    Status::Running => "running",
                    Status::Stopped => "stopped",
                };

                self.frontend.print(&format!("[{}] [{}] {}\n", num, status, rref.cmd));
            }
        }

        self.clear_input();
        self.display_prompt();
    }

    fn execute_internal_fg(&mut self) {
        let mut iter = self.input_buf.split_whitespace();
        iter.next();

        if let Some(arg) = iter.next() {
            if iter.next().is_none() {
                let mut chars = arg.chars();
                if let Some('%') = chars.next() {
                    let num_str = chars.collect::<String>();
                    if let Ok(num) = num_str.parse::<usize>() {
                        if let Some(job) = self.jobs.get_mut(&num) {
                            self.foreground_job = Some(num);
                            for task in job.tasks.iter() {
                                if !task.rref.has_exited() {
                                    task.rref.unblock();
                                }
                                job.status = Status::Running;
                            }
                        } else {
                            self.frontend.print(&format!("No job with number {num} found!\n"));
                        }
                        return;
                    }
                }
            }
        }

        self.frontend.print("Usage: fg %job_num\n");
    }

    fn execute_internal_bg(&mut self) {
        let mut iter = self.input_buf.split_whitespace();
        iter.next();

        if let Some(arg) = iter.next() {
            if iter.next().is_none() {
                let mut chars = arg.chars();
                if let Some('%') = chars.next() {
                    let num_str = chars.collect::<String>();
                    if let Ok(num) = num_str.parse::<usize>() {
                        if let Some(job) = self.jobs.get_mut(&num) {
                            self.foreground_job = Some(num);
                            for task in job.tasks.iter() {
                                if !task.rref.has_exited() {
                                    task.rref.unblock();
                                }
                                job.status = Status::Running;
                            }
                            self.clear_input();
                            self.display_prompt();
                        } else {
                            self.frontend.print(&format!("No job with number {num} found!\n"));
                        }
                        return;
                    }
                }
            }
        }

        self.frontend.print("Usage: bg %job_num\n");
    }

    fn execute_internal_clear(&mut self) {
        self.frontend.clear();
        self.display_prompt();
    }
}
