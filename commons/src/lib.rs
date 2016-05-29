use std::io;
use std::io::Write;
use std::io::stdout;

pub fn prompt(msg: &str) {
    print!("{}", msg);
    let _ = stdout().flush();
}

pub fn input<'a>(mut msg: &'a mut String) -> &str {
    io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read from stdin");

    return msg.trim();
}
