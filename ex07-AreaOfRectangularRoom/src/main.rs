extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is the length of the room in feet? ");
    let mut _length = String::new();
    let length :f64 = input(&mut _length)
        .parse().ok().expect("Not a number");

    prompt("What is the width of the room in feet? ");
    let mut _width = String::new();
    let width :f64 = input(&mut _width)
        .parse().ok().expect("Not a number");

    println!("You entered dimentions of {} feet by {} feet.", length, width);
    println!("The area is");
    println!("{} square feet", length * width);

    let factor = 0.09290304;
    println!("{} square meters", length * width * factor);
}
