extern crate commons;

fn main() {
    commons::prompt("What is the input string? ");

    let mut _input = String::new();
    let input = commons::input(&mut _input);

    println!("{} has {} characters.", input, input.chars().count());
}
