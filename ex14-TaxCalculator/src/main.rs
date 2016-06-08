extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is the ordr amount? ");
    let mut _amount = String::new();
    let amount: f64 = input(&mut _amount)
        .parse().ok().expect("Not a number");
    assert!(amount >= 0.0);

    prompt("What is the state? ");
    let mut _state = String::new();
    let state = input(&mut _state);

    const tax: f64 = 0.055;
    if state == "WI" {
        println!("The subtotal is ${}.", amount);
        println!("The tax is ${}.", amount * tax);
        println!("The total is ${:.*}.", 2, amount * (1.0 + tax));
    } else {
        println!("The total is ${:.*}.", 2, amount);
    }
}
