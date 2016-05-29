extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is the first number? ");
    let mut _first = String::new();
    let first :u32 = input(&mut _first)
                     .parse()
                     .ok()
                     .expect("Not a number");

    prompt("What is the second number? ");
    let mut _second = String::new();
    let second :u32 = input(&mut _second)
                      .parse()
                      .ok()
                      .expect("Not a number");

    println!("{} + {} = {}", first, second, first + second);
    println!("{} - {} = {}", first, second, first - second);
    println!("{} * {} = {}", first, second, first * second);
    println!("{} / {} = {}", first, second, first / second);
}
