fn main() {
    let mut i = 10;
    loop {
        i = i + 1;
        if i % 15 == 0 {
            println!("fizz")
        } else if i % 3 == 0 {
            println!("buzz")
        } else if i % 5 == 0 {
            println!("fizzbuzz")
        } else {
            println!("{}", i);
        }
        if i > 100 {
            break;
        }
    }
    println!("Hello, world!");
}
