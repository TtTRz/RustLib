// vector
fn main() {
    let a_vec: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", a_vec);

    let mut a_vec: Vec<i32> = (1..=3).collect();
    println!("{:?}", a_vec);

    a_vec.push(4);
    println!("{:?}", a_vec);

    let len = a_vec.len();
    println!("len: {}", len);

    let a_vec_0 = a_vec[0];
    println!("get by index 0: {}", a_vec_0);

    let a_vec_pop = a_vec.pop().unwrap();
    println!("pop: {}", a_vec_pop);

    println!("vec.iter()");
    for x in a_vec.iter() {
        println!("> {}", x);
    }

    println!("vec.iter().enumerate()");
    for (i, x) in a_vec.iter().enumerate() {
        println!("index: {}, content: {}", i, x);
    }

    for x in a_vec.iter_mut() {
        *x += 1;
    }
    println!("{:?}", a_vec);

    for x in (0..10).rev() {
        println!("{}", x);
    }
    let sum = a_vec.iter().sum::<i32>();
    println!("{}", sum);
}
