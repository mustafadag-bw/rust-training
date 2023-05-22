use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;

// Mutexes = mutual exclusion. Two rules of mutexes are :
// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
// Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

// Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.

// Send is one of the traits that ensures the types we use with threads are meant for use in concurrent situations.
// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. A: atomic. Atomics work like primitive types but are safe to share across threads
// Atomicity comes with a performance price

// Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty. (Not marked as Send nor Sync)

// Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
// In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely
// to another thread. Similar to Send, primitive types are Sync, and types composed ENTIRELY of types that are Sync are also Sync.
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // This is used to finish threads simultaneously. This code blocks main thread until spawned thread finishes
    // Note that join method blocks main thread if written above the loop, the loop will not be triggered until thread finishes.
    let v = vec![1, 2, 3];

    // move keyword tells the closure will take the ownership of the value instead of expecting Rust to infer that it should be borrowed
    // Note: We cannot use move for the parameters that is dropped on main thread!
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // Drop can run before the thread even started hence Rust cannot know whether thread can get the ownership of the parameter

    // Here main thread waits for handle thread to finish before closing itself. This way we guarantee thread to over before main thread
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    // rx.recv breaks the main thread and waits untill the value is get. This is not very safe since it will block execution of the thread!!
    // rx.try_recv do not breaks main thread. It immediately runs and if the value is not there yet it throws an error
    // let received = rx.recv().unwrap();

    // Multiple transmitter and one receiver (main thread is receiver other threads are transmitter)
    let tx1 = tx.clone();
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
            // Here we sent the val to main thread. Its ownership is now on main thread!
            // println!("val is {}", val);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Here loop will wait for each parameter to be sent to print. It will take 4 secs since the second thread has sleep in it
    for received in rx {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5);

    {
        // MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope
        // Here we converted inner value of m to 6 and dropped lock immediately after
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m); // Will print 6

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
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
