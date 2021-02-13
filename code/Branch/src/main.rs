// branch
#[derive(Debug)]
enum Sex {
    Male,
    Female,
    Other,
}

struct Person {
    name: String,
    sex: Sex,
}

fn main() {
    // if
    let mut a = 1;
    if a == 1 {
        println!(" a == 1: {}", true);
    } else {
        println!(" a == 1: {}", false);
    }

    let mut b = false;
    b = if a == 1 { true } else { false };
    println!("b: {}", b);

    // match
    let male = Person {
        name: format!("Rom"),
        sex: Sex::Male,
    };
    let female = Person {
        name: format!("Yuki"),
        sex: Sex::Female,
    };

    let other = Person {
        name: "o".into(),
        sex: Sex::Other,
    };

    match male.sex {
        Sex::Male => println!("{} is male !", male.name),
        Sex::Female => println!("{} is female", male.name),
        _ => println!("{} dont know", female.name),
    }

    match female.sex {
        Sex::Male => println!("{} is male !", female.name),
        Sex::Female => println!("{} is female", female.name),
        _ => println!("{} dont know", female.name),
    }

    match male.sex {
        Sex::Male | Sex::Female => println!("{} know sex", female.name),
        _ => println!("{} dont know", female.name),
    }

    println!("{:?}", female.sex);

    let op: Option<String> = Some("rom".into());

    match op {
        Some(ref h) => {}
        None => {}
    }

    println!("{:?}", op);

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
    while a != 10 {
        a += 1;
        println!("while (a = {})", a);
    }

    println!("{}", a);
    // loop

    'outer: loop {
        a += 1;
        'inner: loop {
            a += 1;
            if a >= 20 {
                print!("{}", a);
                break 'outer;
            }
        }
    }
}
