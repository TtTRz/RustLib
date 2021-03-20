mod person;

use person::person::Person;

#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Person for Student<'a> {
    fn say_name(&self) {}
    fn say_age(&self) {}
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: i32) -> Self {
        Student { name, age }
    }
}

fn main() {
    let student = Student::new("rom", 22);
    println!("{:?}", student);
}
