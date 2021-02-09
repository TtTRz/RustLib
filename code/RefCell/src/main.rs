use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Student {
    name: String,
    teacher: Option<Rc<RefCell<Teacher>>>,
}

impl Student {
    fn new(name: String) -> Self {
        Student {
            name,
            teacher: None,
        }
    }
}

#[derive(Debug)]
struct Teacher {
    name: String,
    student: Vec<Weak<RefCell<Student>>>,
}

impl Teacher {
    fn new(name: String) -> Self {
        Teacher {
            name,
            student: vec![],
        }
    }
}

fn main() {
    let s_a = Rc::new(RefCell::new(Student::new("Rom".into())));
    let t_a = Rc::new(RefCell::new(Teacher::new("T".into())));
    s_a.borrow_mut().teacher = Some(Rc::clone(&t_a));
    t_a.borrow_mut().student.push(Rc::downgrade(&s_a));
    s_a.borrow_mut().teacher.as_ref().unwrap().borrow_mut().name = "trick".into();
    println!("{:?}", s_a);
}
