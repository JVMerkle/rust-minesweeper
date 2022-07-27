use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;

struct ThreadManager {
    handles: Vec<JoinHandle<i32>>,
    thread_ids: Arc<AtomicU32>,
}

impl ThreadManager {
    fn new() -> Self {
        ThreadManager {
            handles: Vec::new(),
            thread_ids: Arc::new(AtomicU32::new(0)),
        }
    }

    fn create_new(&mut self) {
        let mut thread_ids = self.thread_ids.clone();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1));
            let thread_id = thread_ids.fetch_add(1, Ordering::Acquire);
            println!("T{}: Hello!", thread_id);

            // let mut filename = String::from("Hello_");
            // filename.push_str(thread_id.to_string().as_str());
            // filename.push_str(".txt");
            //
            // let mut file = File::create(filename).unwrap();
            //
            // file.write("Hello World!".as_bytes()).unwrap();

            for i in 1..10 {
                println!("T{}: hi number {} from the spawned thread!", thread_id, i);
                thread::sleep(Duration::from_millis(1));
            }
            0
        });
        self.handles.push(handle);
    }
}

impl Drop for ThreadManager {
    fn drop(&mut self) {
        println!("Dropping ThreadManager");
    }
}

fn foo() {
    let mut tm = ThreadManager::new();

    for _ in 0..100 {
        tm.create_new();
    }

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Main done");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa() {
        foo();
    }
}
