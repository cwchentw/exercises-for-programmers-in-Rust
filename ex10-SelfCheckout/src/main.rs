use std::io::Write;
use std::io::stdout;

extern crate commons;

struct Item {
    price: f64,
    quantity: u32,
}

fn main() {
    let mut items: Vec<Item> = Vec::new();

    let mut count = 1;
    loop {
        print!("Enter the price of item {}: ", count);
        let _ = stdout().flush();

        let mut _input1 = String::new();
        let _price = commons::input(&mut _input1);
        let _result1 = _price.parse();
        if _result1.is_err() {
            break;
        }
        let price: f64 = _result1.unwrap();

        print!("Enter the quantity if item {}: ", count);
        let _ = stdout().flush();

        let mut _input2 = String::new();
        let _quantity = commons::input(&mut _input2);
        let _result2 = _quantity.parse();
        if _result2.is_err() {
            break;
        }
        let quantity: u32 = _result2.unwrap();

        let item = Item { price: price, quantity: quantity};
        items.push(item);

        count += 1;
    }

    let mut sum: f64 = 0.0;
    for item in items {
        let _q = item.quantity as f64;
        sum += item.price * _q;
    }

    let tax = 0.055;
    println!("Subtotal: ${}", sum);
    println!("Tax: ${}", sum * tax);
    println!("Total: ${}", sum * (1.0 + tax))
}
