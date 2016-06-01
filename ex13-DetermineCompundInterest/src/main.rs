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

    let mut msg = String::new();
    msg.push_str("What is the number of times the interest\n");
    msg.push_str("is compounded per year? ");
    let s: &str = &msg;
    prompt(s);
    let mut _per_year = String::new();
    let per_year: f64 = input(&mut _per_year)
        .parse().ok().expect("Not a number");
    assert!(per_year >= 0.0);

    println!("${} invested at {}% for {} years", principal, rate, year);
    println!("compounded {} times per year is ${:.*}.",
             per_year, 2,
             principal * (1.0 + rate / 100.0 / per_year)
             .powf(per_year * year));
}
