use crate::error::{Error, Result};
use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use app_io::IoStreams;
use path::Path;
use stdio::{Stdio, StdioReader, StdioWriter};
use task::JoinableTaskRef;

/// A succesfuly evaluated command.
pub(crate) struct Job {
    tasks: Vec<Task>,
    status: Status,
    cmd: String,
    stdin: StdioWriter,
    stdout: StdioReader,
}

pub(crate) struct Task {
    rref: JoinableTaskRef,
    id: usize,
}

/// The status of a job.
#[derive(PartialEq)]
enum Status {
    /// Normal state. All the tasks in this job are either running or exited.
    Running,
    /// The job is suspended (but not killed), e.g. upon ctrl-Z.
    /// All the tasks in this job are either blocked or exited.
    Stopped,
}

pub(crate) fn eval_cmd(cmd: String) -> Result<Job> {
    let mut tasks = Vec::new();

    let first_stdio = Stdio::new();
    let mut last_reader = first_stdio.get_reader();

    for task_cmd in cmd.split('|') {
        let mut args: Vec<&str> = task_cmd.split_whitespace().collect();
        let command = args.remove(0);

        if let Some(&"&") = args.last() {
            args.pop();
        }

        let task =
            create_task(command, args.into_iter().map(|a| a.to_owned()).collect::<Vec<_>>())?;

        let stdio_link = Stdio::new();
        let streams = IoStreams {
            stdin: last_reader,
            stdout: stdio_link.get_writer(),
            stderr: todo!(),
            key_event_reader: todo!(),
            terminal: todo!(),
        };
        app_io::insert_child_streams(task.id, streams);
        let last_reader = stdio_link.get_reader();

        // TODO: Set task env

        tasks.push(Task { id: task.id, rref: task });
    }

    for task in tasks.iter() {
        task.rref.unblock();
    }

    // TODO: Allocate job numbers?

    Ok(Job {
        tasks,
        status: Status::Running,
        cmd,
        stdin: first_stdio.get_writer(),
        stdout: last_reader,
    })
}

fn create_task(cmd: &str, args: Vec<String>) -> Result<JoinableTaskRef> {
    let namespace_dir = task::get_my_current_task()
        .map(|t| t.get_namespace().dir().clone())
        .ok_or(Error::NamespaceNotFound)?;
    let cmd_crate_name = format!("{}-", cmd);
    let mut matching_files = namespace_dir.get_files_starting_with(&cmd_crate_name).into_iter();
    let app_path = matching_files
        .next()
        .map(|f| Path::new(f.lock().get_absolute_path()))
        .ok_or_else(|| Error::AppNotFound(cmd.to_owned()))?;

    if matching_files.next().is_some() {
        return Err(Error::MultipleAppsFound);
    }

    let task = spawn::new_application_task_builder(app_path, None)
        .map_err(Error::SpawnFailed)?
        .argument(args)
        .block()
        .spawn()
        .map_err(Error::SpawnFailed)?;

    Ok(task)
}
