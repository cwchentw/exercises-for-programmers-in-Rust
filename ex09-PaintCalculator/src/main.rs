extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("Input the length in feet: ");
    let mut _length = String::new();
    let length :f64 = input(&mut _length)
        .parse().ok().expect("Not a number");

    prompt("Input the width in feet: ");
    let mut _width = String::new();
    let width :f64 = input(&mut _width)
        .parse().ok().expect("Not a number");

    println!("You will need to purchase {} gallons of",
             (length * width / 350.0).ceil());
    println!("paint to cover {} square feet.", length * width)
}
