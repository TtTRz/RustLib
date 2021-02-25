use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(10);
    let b = Arc::clone(&a);
    let muta = Arc::new(Mutex::new(100));
    let mutb = Arc::clone(&muta);
    let mutc = Arc::clone(&muta);
    let handle = thread::spawn(move || {
        let mut lock = mutb.lock().unwrap();
        *lock = 300;
    });

    let handl = thread::spawn(move || {
        *mutc.lock().unwrap() = 1000;
    });

    handl.join().unwrap();

    handle.join().unwrap();
    println!("{}", *muta.lock().unwrap());
}
