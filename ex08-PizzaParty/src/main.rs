extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("How many people? ");
    let mut _people = String::new();
    let people :i32 = input(&mut _people)
        .parse().ok().expect("Not a number");

    prompt("How many pizzas do you have? ");
    let mut _pizzas = String::new();
    let pizzas :i32 = input(&mut _pizzas)
        .parse().ok().expect("Not a number");

    println!("{} people with {} pizzas", people, pizzas);
    let slices = 8;
    println!("Each persons get {} pieces of pizza", pizzas * slices / people);
    println!("There are {} leftover pieces.", pizzas * slices % people);
}
