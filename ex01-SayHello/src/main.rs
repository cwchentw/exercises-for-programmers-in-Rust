extern crate commons;

fn main() {
    commons::prompt("What is your name? ");
    let mut _name = String::new();
    let name = commons::input(&mut _name);

    println!("Hello, {}, nice to meet you!", name);
}
