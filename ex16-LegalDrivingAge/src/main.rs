extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is your age? ");
    let mut _age = String::new();
    let age: u32 = input(&mut _age)
        .parse().ok().expect("Not a number");

    if age < 16 {
        println!("You are not old enough to legally drive.");
    } else {
        println!("You are old enough to legally drive.");
    }
}
