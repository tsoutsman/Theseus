#![no_std]

extern crate alloc;

use log::{warn, error};
use alloc::{
    format,
    vec::Vec,
    string::String,
    sync::Arc,
};
use sync_block::Mutex;

pub fn main(_args: Vec<String>) -> isize {    
    let res = match _args.get(0).map(|s| &**s) {
        Some("-c") => test_contention(),
        _          => test_lockstep(),
    };
    match res {
        Ok(_) => 0,
        Err(e) => {
            error!("Error: {}", e); 
            -1
        }
    }
    
}

/// A simple test that spawns 3 tasks that all contend to increment a shared usize
fn test_contention() -> Result<(), &'static str> {
    let my_cpu = cpu::current_cpu();

    let shared_lock = Arc::new(Mutex::new(0usize));

    let t1 = spawn::new_task_builder(sync_block_task, shared_lock.clone())
        .name(String::from("sync_block_test_1"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;

    let t2 = spawn::new_task_builder(sync_block_task, shared_lock.clone())
        .name(String::from("sync_block_test_2"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;
    
    let t3 = spawn::new_task_builder(sync_block_task, shared_lock.clone())
        .name(String::from("sync_block_test_3"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;

    warn!("Finished spawning the 3 tasks");

    t3.unblock().unwrap();
    t2.unblock().unwrap();
    t1.unblock().unwrap();

    t1.join()?;
    t2.join()?;
    t3.join()?;
    warn!("Joined the 3 tasks. Final value of shared_lock: {:?}", shared_lock);
    
    Ok(())
}


fn sync_block_task(lock: Arc<Mutex<usize>>) -> Result<(), &'static str> {
    let curr_task = task::with_current_task(|t| format!("{:?}", t))
        .map_err(|_| "couldn't get current task")?;
    warn!("ENTERED TASK {}", curr_task);

    for _i in 0..1000 {
        task::schedule(); // give other tasks a chance to acquire the lock
        warn!("{} trying to acquire lock...", curr_task);
        let mut locked = lock.lock();
        warn!("{} acquired lock!", curr_task);
        *locked += 1;
        warn!("{} incremented shared_lock value to {}.  Releasing lock.", curr_task, &*locked);
    }
    warn!("{} \n     FINISHED LOOP.", curr_task);
    Ok(())
}



/// A test for running multiple tasks that are synchronized in lockstep
fn test_lockstep() -> Result<(), &'static str> {
    let my_cpu = cpu::current_cpu();

    let shared_lock = Arc::new(Mutex::new(0usize));

    let t1 = spawn::new_task_builder(lockstep_task, (shared_lock.clone(), 0))
        .name(String::from("lockstep_task_1"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;

    let t2 = spawn::new_task_builder(lockstep_task, (shared_lock.clone(), 1))
        .name(String::from("lockstep_task_2"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;
    
    let t3 = spawn::new_task_builder(lockstep_task, (shared_lock.clone(), 2))
        .name(String::from("lockstep_task_3"))
        .pin_on_cpu(my_cpu)
        .block()
        .spawn()?;

    warn!("Finished spawning the 3 tasks");

    t3.unblock().unwrap();
    t2.unblock().unwrap();
    t1.unblock().unwrap();

    t1.join()?;
    t2.join()?;
    t3.join()?;
    warn!("Joined the 3 tasks. Final value of shared_lock: {:?}", shared_lock);
    
    Ok(())
}


fn lockstep_task((lock, remainder): (Arc<Mutex<usize>>, usize)) -> Result<(), &'static str> {
    let curr_task = task::with_current_task(|t| format!("{:?}", t))
        .map_err(|_| "couldn't get current task")?;
    warn!("ENTERED TASK {}", curr_task);

    for _i in 0..20 {
        loop { 
            warn!("{} top of loop, remainder {}", curr_task, remainder);
            task::schedule(); // give other tasks a chance to acquire the lock
            let mut locked = lock.lock();
            task::schedule();
            if *locked % 3 == remainder {
                warn!("Task {} Time to shine, value is {}!", curr_task, *locked);
                *locked += 1;
                break;
            } else {
                task::schedule();
                warn!("Task {} going back to sleep, value {}, remainder {}!", curr_task, *locked, remainder);
            }
            task::schedule();
        }
    }
    warn!("{} finished loop.", curr_task);
    Ok(())
}
