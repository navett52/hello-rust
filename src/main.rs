use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let foo = 5;
    let bar = 10;

    let foobar = foo + bar;
    let new_num = add(foo, bar);
    println!("{}", foobar);
    println!("{}", new_num);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
