#[derive(Debug)]
struct Name<'a> {
    first_name: &'a str,
    last_name: &'a str,
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

    println!("{:?}", person_name);
}
