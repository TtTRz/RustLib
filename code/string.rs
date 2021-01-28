// string
fn main() {
    let a_string: String = String::from("a_string");
    let b_string: String = format!("b_string");
    println!("a_string = {}, b_string = {}", a_string, b_string);

    let c_str: &str = "c_str";

    let c_string = a_string;
    // println!("{}", &a_string); // 报错，a_string 所有权已经被转移到 c_string
    println!("c_string = {}", c_string); // a_string

    let d_string = &c_string; // 借用 c_string
    println!("c_string = {}", c_string); // a_string
    println!("d_string = {}", d_string); // a_string

    let mut e_string = format!("e_string");
    e_string = e_string + c_str;
    println!("e_string = {}", e_string);

    let bool_str: &str = "true";
    let bool_string: String = format!("true");
    println!("bool_str == bool_string: {}", bool_str == bool_string);

    let bool_str = bool_string.as_str();
    println!("{}", bool_str);
    println!("{}", bool_string);

    for i in bool_string.bytes() {
        // [116,114,117,101] true ASCII
        println!("{}", i);
    }

    for i in bool_string.chars() {
        // [t, r, u ,e]
        println!("{}", i);
    }

    let bool_string_split: Vec<_> = bool_string.split("").collect();
    println!("bool_string_split: {:?}", bool_string_split); // ["", "t", "r", "u", "e", ""]

    let bool_string_map = bool_string.chars().map(|x| x.to_string() + "end");
    println!("{:?}", bool_string_map);
    for i in bool_string_map { 
        println!("{}", i);
    }

    
}
