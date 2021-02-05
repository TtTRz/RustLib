// struct
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: format!("{}", name),
            age,
        }
    }
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

#[derive(Debug, Clone)]
struct Student {
  person: Person,
  teacher: Person
}

impl Student {
  fn new(name: &str, age: u8, teacher: Person) -> Self {
    Self {
      person: Person {
        name: format!("{}", name),
        age,
      },
      teacher
    }
  }
}

fn main() {
    let person = Person {
        name: format!("Rom"),
        age: 22,
    };
    println!("{:?}", person);

    let mut person = Person::new("RomC", 22);

    println!("{:?}", person);
    person.set_age(1);
    println!("{:?}", person);
    person.age += 1;
    println!("{:?}", person);

    let student = Student::new("RomC", 22, person.clone());

    println!("{:?}, {:?}", student, person);
}
