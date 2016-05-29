extern crate time;

extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("What is your current age? ");
    let mut _age = String::new();
    let age :i32 = input(&mut _age)
                   .parse()
                   .ok()
                   .expect("Not a number");

    prompt("At what age would you like to retire? ");
    let mut _retiredAge = String::new();
    let retiredAge :i32 = input(&mut _retiredAge)
                          .parse()
                          .ok()
                          .expect("Not a number");

    let currentYear :i32 = time::now().tm_year + 1900;

    println!("You have {} years left until you can retire.", retiredAge - age);
    println!("It's {}, so you can retire in {}.", currentYear, currentYear + retiredAge - age);
}
