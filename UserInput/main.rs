use std::io::{self, Write};

fn main(){


    print!("Insert a number: ");
    io::stdout().flush().expect("Flush Error");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Invalid input");
    let num1: i32 = num1.trim().parse().expect("Not a valid number");

    print!("Insert another number: ");
    io::stdout().flush().expect("Flush Error");

    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Invalid input");
    let num2: i32 = num2.trim().parse().expect("Not a valid number");

    let result = num1 + num2;

    println!("The result is: {}", result);

}
