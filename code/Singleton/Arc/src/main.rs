use std::sync::{Arc, Mutex};
use std::thread;

struct Singleton {
    count: Option<Arc<Mutex<i32>>>,
}

impl Singleton {
    pub fn new() -> Self {
        Singleton { count: None }
    }

    pub fn get_instance() -> Arc<Mutex<Singleton>> {
        static mut SINGLETON: Option<Arc<Mutex<Singleton>>> = None;
        unsafe {
            SINGLETON
                .get_or_insert_with(|| Arc::new(Mutex::new(Singleton::new())))
                .clone()
        }
    }
}

fn main() {
    let singleton = Singleton::get_instance();
    let mut handles = vec![];
    for _ in 0..10 {
        let singleton = Arc::clone(&singleton);
        let handle = thread::spawn(move || {
            if singleton.lock().unwrap().count.is_none() {
                singleton.lock().unwrap().count = Some(Arc::new(Mutex::new(10)));
            } else {
                *singleton
                    .lock()
                    .unwrap()
                    .count
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap() += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "{}",
        *singleton
            .lock()
            .unwrap()
            .count
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
    );
}
