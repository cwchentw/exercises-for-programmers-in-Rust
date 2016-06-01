extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("Enter the principal: ");
    let mut _principal = String::new();
    let principal: f64 = input(&mut _principal)
        .parse().ok().expect("Not a number");

    prompt("Enter the rate of interest: ");
    let mut _rate = String::new();
    let rate: f64 = input(&mut _rate)
        .parse().ok().expect("Not a number");

    prompt("Enter the number of years: ");
    let mut _year = String::new();
    let year: f64 = input(&mut _year)
        .parse().ok().expect("Not a number");

    println!("After {} years at {}%, the investment will", year, rate);
    println!("be worth ${}.", principal * (1.0 + year * (rate / 100.0)));
}
