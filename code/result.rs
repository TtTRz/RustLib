fn add(a: i64, b: i64) -> Result<i64, &'static str> {
    if a > b {
        return Err("error");
    }
    Ok(a + b)
}

fn main() {
    let a = 3i64;
    let b = 2i64;
    let r = add(a, b);
    println!("{}", r.unwrap());
}
