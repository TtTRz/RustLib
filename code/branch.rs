// branch

fn main() {
    // if
    let mut a = 1;
    if (a == 1) {
        println!(" a == 1: {}", true);
    } else {
        println!(" a == 1: {}", false);
    }

    let mut b = false;
    b = if (a == 1) { true } else { false };
    println!("a == 1: {}", b);

    // match
    match a {
        1 => println!(" a == 1: {}", true),
        _ => println!(" a == 1: {}", false),
    }
    b = match a {
        1 => {
            println!(" a == 1: {}", true);
            true
        }
        _ => {
            println!(" a == 1: {}", false);
            false
        }
    };
    println!("a == 1: {}", b);

    // for
    let mut arr = [1, 2, 3, 4, 5];

    for i in &arr {
        println!("item: {}", i);
    }

    for (index, item) in arr.iter().enumerate() {
        println!("index: {}, item: {}", index, item);
    }

    for (index, item) in arr.iter_mut().enumerate() {
        *item += index;
        println!("index: {}, item + index: {}", index, item);
    }

    // while
    while (a != 10) {
        a += 1;
    }
    println!("{}", a);
    // loop

    loop {
        a += 1;
        if (a == 20) {
            print!("{}", a);
            break;
        }
    }
}
