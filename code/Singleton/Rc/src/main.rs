use std::cell::RefCell;
use std::rc::Rc;

struct Singleton {
    count: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { count: 0 }
    }

    pub fn get_instance() -> Rc<RefCell<Singleton>> {
        static mut SINGLETON: Option<Rc<RefCell<Singleton>>> = None;
        unsafe {
            SINGLETON
                .get_or_insert_with(|| Rc::new(RefCell::new(Singleton::new())))
                .clone()
        }
    }
}

fn add_count(x: i32) {
    let singleton = Singleton::get_instance();
    singleton.borrow_mut().count += x;
    println!("{}", singleton.borrow().count);
}

fn main() {
    let singleton = Singleton::get_instance();
    let count = singleton.borrow().count;
    println!("{}", count);
    add_count(2);
    add_count(3);
    add_count(-1);
}
