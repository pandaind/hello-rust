fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);

    let mut vec = vec![2, 4, 8, 10];
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);

    let mut s = "Hello".to_string();
    s = concat_string(s);
    println!("{}", s);

    control_flow(31);
}

fn concat_string(mut s: String) -> String {
    s.push_str(" World");
    return s;
}

fn control_flow(a: i8) {
    if a == 1 {
        println!("The value is one")
    } else if a > 50 {
        println!("The value is greater than 50")
    } else if a < 25 {
        println!("The value is less than 25")
    } else {
        println!("The value is greater than 25 but less than 50")
    }
}
