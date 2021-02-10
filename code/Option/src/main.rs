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

    let mut person_name: Option<String> = Some("rom".into());

    {
        let b = person_name.as_mut().unwrap();
        *b = "hello".into();
    }

    println!("{:?}", person_name);

    let mut rom_name: Option<String> = Some("rom".into());
    let m = rom_name.take();
    println!("{:?}", rom_name); // None
    let mut unwrap_name: Option<String> = Some("rom".into());
    // let un = unwrap_name.unwrap();
    // println!("{:?}, {:?}", unwrap_name, un); // unwrap_name moved due to this method call

    let unm = unwrap_name.as_mut().unwrap();
    *unm = "lll".into();
    let un = unwrap_name.as_ref().unwrap();
    println!("{:?}, {:?}", unwrap_name, un);
}

// Option::unwrap
// Option::take
// Option::as_ref
// Option::as_mut
// Option::is_none
// Option::is_some
// Option::contains
