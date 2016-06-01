extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is the principal amount? ");
    let mut _principal = String::new();
    let principal: f64 = input(&mut _principal)
        .parse().ok().expect("Not a number");
    assert!(principal >= 0.0);

    prompt("What is the rate? ");
    let mut _rate = String::new();
    let rate: f64 = input(&mut _rate)
        .parse().ok().expect("Not a number");
    assert!(rate >= 0.0);

    prompt("What is the number of years? ");
    let mut _year = String::new();
    let year: f64 = input(&mut _year)
        .parse().ok().expect("Not a number");
    assert!(year >= 0.0);

    let msg = "What is the number of times the interest
is compounded per year? ";
    prompt(msg);
    let mut _perYear = String::new();
    let perYear: f64 = input(&mut _perYear)
        .parse().ok().expect("Not a number");
    assert!(perYear >= 0.0);

    println!("${} invested at {}% for {} years", principal, rate, year);
    println!("compounded {} times per year is ${:.*}.",
             perYear, 2,
             principal * (1.0 + rate / 100.0 / perYear).powf(perYear * year));
}
