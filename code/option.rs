fn main() {
    // Option Some None
    let first_name: Option<String> = Some(format!("Rom"));
    let last_name = Some(format!("Chung"));
    let last_name_none: Option<String> = None;
    // println!("{}", first_name); // Option<String>
    println!("{}", first_name.unwrap());
    println!("{}", last_name_none.expect("last_name is missing")); // panic with error
}
