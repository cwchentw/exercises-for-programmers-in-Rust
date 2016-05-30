extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("How many euros are you exchanging? ");
    let mut _euros = String::new();
    let euros: f64 = input(&mut _euros)
        .parse().ok().expect("Not a number");

    prompt("What is the exchange rate? ");
    let mut _rate = String::new();
    let rate: f64 = input(&mut _rate)
        .parse().ok().expect("Not a number");

    println!("{} euros at an exchange rate of {} is", euros, rate);
    println!("{} U.S. dollars", euros * rate / 100.0);
}
