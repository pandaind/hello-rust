fn main() {
    //OWNERSHIP
    let var = 1; // on the stack
    println!("{}", var);
    let mut s = "hello".to_string(); // created on heap
    s.push_str(",world"); // allocated on heap as we are able to grow

    //MOVE
    let x = vec!["hello".to_string()];
    let y = x;
    let z = y;
    // println!("{:?}", x); -> error because of x's ownership moved to y
    // println!("{:?}", y); -> error because of y's ownership moved to z
    println!("{:?}", z);

    //CLONE
    let x = vec!["hello".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    //COPY
    let x = 1;
    let y = x; // some types implements copy trait so its working
    println!("x = {}, y = {}", x, y);

    //MORE MOVE
    let mm = String::from("takes");
    takes_ownership(mm); // give ownership to the function
    // println!("{}", mm) -> ownership is taken by the function
    let val = 1;
    make_copy(val);
    println!("{}", val); // -> some types implements copy trait so its working and save on stack

    let str = give_ownership();
    println!("{}", str);

    let back = takes_and_give(str);
    println!("{}", back);

    if true {
        let _str1 = back;
    }

    // println!("{}", back); -> also in control flow ownership concept is valid

    let mut _str2: String;
    /* loop {
         str2 = back; -> cant change ownership also inside loop
     }*/

    //REFERENCES AND BORROWING
    //1. shared references
    //2. mutable references
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);
}

// var is dropped here, s is dropped here


fn change_string(s1: &mut String) {
    s1.push_str(", world");
}

fn takes_ownership(s: String) {
    let str = s;
    println!("{}", str);
}

fn make_copy(one: i32) {
    let copy = one;
    println!("{}", copy);
}

fn give_ownership() -> String {
    "Given".to_string()
}

fn takes_and_give(s: String) -> String {
    s
}