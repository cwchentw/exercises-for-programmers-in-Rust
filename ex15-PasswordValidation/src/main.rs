extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is the password? ");
    let mut _pass = String::new();
    let pass = input(&mut _pass);

    if pass == "abc$123" {
        println!("Welcome!");
    } else {
        println!("I don't know you.");
    }
}
