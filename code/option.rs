fn main() {
    // Option Some None
    let first_name: Option<String> = Some(format!("Rom"));
    let last_name = Some(format!("Chung"));
    let last_name_none: Option<String> = None;
    // println!("{}", first_name); // Option<String>

    let first_name_string = match first_name {
        Some(ref name) => name.clone(),
        None => format!("none"),
    };

    let last_name_none_string = match last_name_none {
        Some(ref name) => name.clone(),
        None => format!("none"),
    };

    println!("{}", first_name_string);
    println!("{}", last_name_none_string);
    println!("{}", first_name.unwrap());

    println!("{}", last_name_none.unwrap_or(format!("default_name")));

    // println!("{}", last_name_none.expect("last_name is missing")); // panic with error
}
