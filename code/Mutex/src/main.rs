use std::sync::{Arc, Mutex};
use std::thread;

// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap(); // counter is moved
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("{}", *counter.lock().unwrap());
// }

// ARC
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // counter is moved
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}

//实施了std::marker下的send的类型叫做send, 实施了sync的叫做sync
//send可以在线程间转移所有权，sync引用可以被线程共享，T是sync, 则&T is send
//大部分类型是send and sync
//Rc<T>, RefCell<T>, Cell<T>不是send也不是sync, 
//Mutex是sync, 所以引用可以被线程共享