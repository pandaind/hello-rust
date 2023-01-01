fn main() {
    // 8 bit
    let x: i8 = 10;
    println!("x is {}", x);

    // unsigned 8 bit
    let y: u8 = 10;
    println!("y is {}", y);

    // Decimal
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;
    println!("decimal {}", decimal);
    println!("hex {}", hex);
    println!("octal {}", octal);
    println!("binary {}", binary);

    // Byte
    let byte = b'A';
    println!("byte {}", byte);

    // floating point
    let f = 2.1343242253246769; // f64 default
    let f32: f32 = 1.55435329;
    println!("f is {}", f);
    println!("f32 is {}", f32);

    // boolean
    let t = true;
    let f: bool = false;
    println!("t is {}", t);
    println!("f is {}", f);

    // character
    let c = 'c';
    println!("c is {}", c);


    // + - * / %
    let a = 10;
    let b = 4;

    let rem = a % b;
    println!("rem is {}", rem);

    // Tuples
    let tup = (10, "Rust", true);
    println!("Tuple ({},{},{})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, z);

    // Arrays
    let array = [1, 2, 3];
    println!("index 0 is {}", array[0]);
    let mut array2: [i32; 3] = [1, 2, 3];
    println!("index 0 is {}", array2[0]);
    array2[0] = 10;
    println!("index 0 is {}", array2[0]);

    // Vector
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("nums {:?}", nums);
    nums.pop();
    println!("nums {:?}", nums);

    let mut vec = Vec::new();
    vec.push("hello");
    vec.push("world");
    println!("vec {:?}", vec);
    vec.reverse();
    println!("reverse vec {:?}", vec);

    let mut num_vec = Vec::<i32>::with_capacity(2);
    num_vec.push(1);
    println!("{}", num_vec.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    // Slice
    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);

    // String
    let hello_world = String::from("Hello World!");
    let course = "Rust".to_string();

    let hello_rust = hello_world.replace("World!", "Rust!");

    println!("{}", hello_world);
    println!("{}", course);
    println!("{}", hello_rust);

    // &str = "string slice" or "stir"
    let world = &hello_world.replace("Hello ", "");
    println!("{}", world);
    println!("{}", hello_world);

    // compare string == / !=
    println!("{}", "ONE".to_lowercase() == "one");

    // string literals - not utf8
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}