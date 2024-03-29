fn main() {
    println!("Hello, world!");
    fizz_buz();
}

fn fizz_buz() {
    let mut fizz_buzz_count = 0;
    for n in 1..=301 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }
    println!("Number of fizz buzz: {}", fizz_buzz_count)
}
