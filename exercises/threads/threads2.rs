// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            if let Ok(mut status_shared) = status_shared.lock() {
                status_shared.jobs_completed += 1;
            }
        });
        handles.push(handle);
    }

    // As per the TODO this causes the output to happen 10x
    /*
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
    */

    // I believe one can `join()` any handle and get the correct result.
    //
    // Thinking... if any thread acquires thr lock when mainthread tries, it'll
    // get blocked behind all the threads
    handles.remove(0).join().unwrap();
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
