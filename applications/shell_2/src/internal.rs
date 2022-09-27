use crate::{frontend::Frontend, job::Status, Shell};
use alloc::{format, string::String};
use log::error;

/// Internal commands.
impl<T> Shell<T>
where
    T: Frontend,
{
    /// Tries to interpret and execute the input buffer as an internal shell
    /// command.
    ///
    /// Returns `true` if the input buffer was succesfuly interpreted.
    pub(crate) fn try_execute_internal(&mut self) -> bool {
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
        if self.execute_internal_g().is_err() {
            self.frontend.print("Usage: fg %job_num\n");
        }
    }

    fn execute_internal_bg(&mut self) {
        match self.execute_internal_g() {
            Ok(true) => {
                self.clear_input();
                self.display_prompt();
            }
            Ok(false) => {}
            Err(()) => self.frontend.print("Usage: bg %job_num\n"),
        }
    }

    fn execute_internal_g(&mut self) -> Result<bool, ()> {
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
                            return Ok(true);
                        } else {
                            self.frontend.print(&format!("No job with number {num} found!\n"));
                            return Ok(false);
                        }
                    }
                }
            }
        }
        Err(())
    }

    fn execute_internal_clear(&mut self) {
        self.frontend.clear();
        self.display_prompt();
    }
}
