use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
};

struct Shared {
    queue: Mutex<VecDeque<u32>>,
    condvar: Condvar,
}

#[allow(unused_variables)]
fn main() {

    let shared = Arc::new(
        Shared {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        });


    let mut threads = Vec::new();

    for i in 0..5 {
        let s = Arc::clone(&shared);
        let th = thread::spawn( move || {
            s.queue.lock().unwrap().push_front(i);
            println!("Thread - {:?}", i);
            return i;
        });
        threads.push(th);
    }


    for th in threads {
        let res = th.join();
        println!("Result : {:?} ", res);
    }
}


