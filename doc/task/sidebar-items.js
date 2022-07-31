window.SIDEBAR_ITEMS = {"enum":[["ExitValue","The list of ways that a Task can exit, including possible return values and conditions."],["KillReason","The list of possible reasons that a given `Task` was killed prematurely."],["RunState","The set of possible runstates that a task can be in, e.g., runnable, blocked, exited, etc. "]],"fn":[["bootstrap_task","Bootstrap a new task from the current thread of execution."],["get_my_current_task","Returns a reference to the current task."],["get_my_current_task_id","Returns the current task’s ID."],["get_task","returns a shared reference to the `Task` specified by the given `task_id`"],["set_kill_handler","Registers a kill handler function for the current `Task`."],["take_kill_handler","Takes ownership of the current `Task`’s [`KillHandler`] function."]],"struct":[["PanicInfoOwned","Just like `core::panic::PanicInfo`, but with owned String types instead of &str references."],["RestartInfo","A struct holding data items needed to restart a `Task`."],["TASKLIST","The list of all Tasks in the system."],["Task","A structure that contains contextual information for a thread of execution. "],["TaskInner","The parts of a `Task` that may be modified after its creation."],["TaskRef","A shareable, cloneable reference to a `Task` that exposes more methods for task management, and accesses the enclosed `Task` by locking it. "]],"type":[["FailureCleanupFunction","The signature of a Task’s failure cleanup function."],["KillHandler","The function signature of the callback that will be invoked when a given Task panics or otherwise fails, e.g., a machine exception occurs."]]};