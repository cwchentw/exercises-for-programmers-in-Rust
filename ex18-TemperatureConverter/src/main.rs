extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    println!("Press C to convert from Fahrenheit to Celsius.");
    println!("Press F to convert from Celsius to Fahrenheit.");
    prompt("Your choice: ");
    let mut _choice = String::new();
    let choice = input(&mut _choice);
    assert!(choice == "C" || choice == "F");

    if choice == "C" {
        prompt("Please enter the temperature in Fahrenheit: ");
        let mut _temp = String::new();
        let temp: f64 = input(&mut _temp)
            .parse().ok().expect("Not a number");
        println!("The temperature in Celsius is {:.*}.", 2, f2c(temp));
    } else {
        prompt("Please enter the temperature in Celsius: ");
        let mut _temp = String::new();
        let temp: f64 = input(&mut _temp)
            .parse().ok().expect("Not a number");
        println!("The temperature in Fahrenheit is {:.*}.", 2, c2f(temp));
    }
}

fn c2f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn f2c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
