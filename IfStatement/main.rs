use std::io::{self, Write};

fn main(){

    print!("Insert a number: ");
    io::stdout().flush().expect("Flush error");

    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Invalid Input");
    let num: i32 = num.trim().parse().expect("Not a valid number");

    if num%2 == 0{

        println!("The inserted number is even");

    } else {

        println!("The inserted number is odd");

    }



}
