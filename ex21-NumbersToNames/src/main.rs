extern crate commons;
use commons::prompt;
use commons::input;

fn main() {
    prompt("Please enter the number of the month: ");
    let mut _month = String::new();
    let month = input(&mut _month)
        .parse().ok().expect("Not a number");

    match num2month(month) {
        Ok(m) => {
            println!("The name of the month is {}.", m);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn num2month<'a>(num: i32) -> Result<&'a str, &'a str> {
    match num {
        1 => { Ok("January") }
        2 => { Ok("February") }
        3 => { Ok("March") }
        4 => { Ok("April") }
        5 => { Ok("May") }
        6 => { Ok("June") }
        7 => { Ok("July") }
        8 => { Ok("Augest") }
        9 => { Ok("September") }
        10 => { Ok("October") }
        11 => { Ok("November") }
        12 => { Ok("December") }
        _ => { Err("Unknown month") }
    }
}
