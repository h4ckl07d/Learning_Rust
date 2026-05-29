use std::io;
fn main() {
    println!("Welcome to Temperature converter");
    loop{
     println!("Enter your Temperature");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to enter Temp");

    println!("Enter (1) Celsius to Fahrenheit or (2) Fahrenheit to Celsius");
    }
  
}
