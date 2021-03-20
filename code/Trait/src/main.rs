mod person;

use person::person::{Assistant, Person};

#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Person for Student<'a> {
    fn say_name(&self) {
        println!("My name: {}", self.name);
    }
    fn say_age(&self) {
        println!("My age: {}", self.age);
    }
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: i32) -> Self {
        Student { name, age }
    }
}

impl<'a> Assistant for Student<'a> {}

fn main() {
    let student = Student::new("rom", 22);
    student.say_age();
    student.say_name();
    student.say_teacher_name();
    println!("{:?}", student);
}
