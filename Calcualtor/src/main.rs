
use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("Could not flush stdout");
    stdin().read_line(input)
        .expect("Could not read input");
}

fn main() {
   println!("welcome to the calulator");
    println!("----------------------");

    let mut num1 = String::new();
    let mut num2 = String::new();

    let mut operator = String::new();

    print!("Please input your first number: ");
    read(&mut num1);
    print!("Please input your second number: ");
    read(&mut num2);
    print!("Please input your operator [+,-,*,/]: ");
    read(&mut operator);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();




    if !["+", "-", "*", "/"].contains(&operator.trim()) {
        println!("Unknown operator");
        return;
    }

    let result = match operator.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Unknown operator"),
    };


    println!("The result of {} {} {} = {}", num1, operator, num2, result);
}
