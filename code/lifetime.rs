#[derive(Debug)]
struct Name<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

#[derive(Debug)]
struct Person<'a, 'b> {
    name: &'a str,
    address: &'b str,
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let name = String::from("Rom");
    {
        let block_name = String::from("Chung");
        println!("{}", block_name);
    }
    // println!("{}", block_name) // error
    println!("{}", name);
    let f_n_str = "Rom";
    let l_n_str = "Chung";

    let person_name = Name {
        first_name: f_n_str,
        last_name: l_n_str,
    };

    {
        let address = "guangzhou";
        let person = Person {
            name: &name,
            address,
        };
        println!("{:?}", person);
    }

    println!("{:?}", person_name);

    println!("{}", longer(f_n_str, l_n_str));
}
