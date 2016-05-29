extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("Enter a noun: ");
    let mut _noun = String::new();
    let noun = input(&mut _noun);

    prompt("Enter a verb: ");
    let mut _verb = String::new();
    let verb = input(&mut _verb);

    prompt("Enter an adjective: ");
    let mut _adj = String::new();
    let adj = input(&mut _adj);

    prompt("Enter an adverb: ");
    let mut _adv = String::new();
    let adv = input(&mut _adv);

    println!("Do you {} your {} {} {}?  That's hilarious!",
             verb, adj, noun, adv);
}
