fn main() {
    // functions
    print_phrase("hello");
    println!("{}", gcd(20, 4));
}

fn print_phrase(phrase: &str) {
    println!("{}", phrase);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}