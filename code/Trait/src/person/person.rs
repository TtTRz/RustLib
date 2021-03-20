pub trait Person {
    fn say_name(&self);
    fn say_age(&self);
}

pub trait Assistant {
    fn say_teacher_name(&self) {
        println!("My teacher name: None");
    }
}
