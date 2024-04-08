// Do not communicate by sharing memory, share memory by communicating
use std::thread;
use std::time::Duration;

fn main() {
    spawn_thread();
    pass_values_between_threads();
    channels();
    multiple_values_waiting_receiver();
    shared_state_concurrency();
}

// Closures are often used with move keyword to take ownership of values from environment and pass between threads

fn spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // if we put join here, it will wait for the thread to finish before moving on. prevents alternating threads.
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // thread::spawn returns a JoinHandle<T>. it's owned and when we call join on it, will wait for its thread to finish. this prevents premature thread exit.
}

fn pass_values_between_threads() {
    let v = vec![1, 2, 3];
    // without `move`, values within this thread could outlive the main thread, resulting in access and double-drop errors
    let handle = thread::spawn(move || {
        println!("Heres a vector {:?}", v);
    });
    // drop(v); // additionally, if we tried to drop here after moving, we'd get an error for a similar reason
    handle.join().unwrap();
}

use std::sync::mpsc; // multiple producer, single consumer
                     // channels allow one-to-many streams of data. Transmitters and Receiver. channel is closed if either half is dropped.
fn channels() {
    let (tx, rx) = mpsc::channel(); // tx and rx are traditional syntax for transmitter and receiver
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is val: {val}"); // not allowed, we already sent the value down the channel
    });
    let received = rx.recv().unwrap();
    println!("Got it! {received}");
}

fn multiple_values_waiting_receiver() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // create multiple producers by cloning the transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

// `mutex` = mutual exclusion, only one thread can access data at any given time
// this is accomplished by acquiring a lock on the data and the mutex guards the data via a locking system
// 1. acquire the lock before using data 2. unlock the data that mutex guards when done so others can acquire lock
// use std::rc::Rc; // using Rc for multiple ownership does not work, it is designed for single threaded applications
use std::sync::{Arc, Mutex}; // Atomic-rc or Arc is safe for multi-threaded use``
                             // Why all primitive types aren't atomic and std libs don't default Arc, it's b/c guaranteeing atomicity has performance penalty
                             // We don't want to incur that cost unless we have to, and in single thread, there's no need.
                             // Mutex is an interior mutable, thread safe, version of RefCell
use std::sync::atomic::{AtomicU16, Ordering}; // atomics can be used for primitive types
fn shared_state_concurrency() {
    let m = AtomicU16::new(5); // not
    {
        m.store(6, Ordering::Relaxed)
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0)); // set the atomic ref count on mutex
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone it for access, without taking ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
// Caveat; mutexes can cause deadlocks. e.g. need to acquire two locks to op but two threads have each acquired one lock each.
// they would be waiting on each other forever for the other lock

// Extensible concurrency with `std::marker::Send` and `std::marker::Sync` traits
// Almost all rust prim types are Send types except raw pointers
// `Sync` maker trait indicates the type is thread safe, can be ref'd from many threads
// Any type that implements `Send` and `Sync` are also `Send` and `Sync`. Nothing to implement, just a marker for concurrent types.
// Manually implementing these is possible but UNSAFE. Not recommended.

// Rust doesn't have a lot of concurrency baked into lang, so most use crates
// Search online for state-of-the-art crates to use in multithreaded situations
