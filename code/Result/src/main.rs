type UseResult = Result<i64, MathError>;

#[derive(Debug)]
enum MathError {
    AddError,
}

fn add(a: i64, b: i64) -> UseResult {
    if a > b {
        return Err(MathError::AddError);
    }
    Ok(a + b)
}

fn use_add(a: i64, b: i64) -> UseResult {
    let a = add(a, b)?;
    Ok(a)
}

fn main() {
    let a = 3i64;
    let b = 2i64;
    let r = use_add(a, b);
    match r {
        Ok(n) => {
            println!("{}", n)
        }
        Err(e) => panic!(match e {
            MathError::AddError => "add error",
            _ => "other error",
        }),
    }
    // println!("{}", r.unwrap());
}
