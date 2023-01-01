fn main() {
    // if statements
    let one = 1;

    if one > 2 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("False");
    }

    // loop
    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decreas = 5;
        loop {
            println!("Decreasing {}", decreas);
            if decreas == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decreas -= 1;
        }
        num += 1;
    }

    // while
    let mut num = 0;
    while num < 5 {
        println!("Num: {}", num);
        num += 1;
    }

    // loop vectors
    let vec: Vec<i8> = (0..10).collect();
    for element in vec  {
        println!("{}", element);
    }

    for element in (0..5).rev()  {
        println!("{}", element);
    }
}
