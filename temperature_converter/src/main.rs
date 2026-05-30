use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - 32.0) * 5.0/9.0
}
fn main() {
    println!("Welcome to Temperature converter");

    loop{

        //get temperature
    println!("\nEnter your Temperature");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let temperature:f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number, please try again");
            continue;
        }
    };

        //get Conversion choice
    println!("Enter (1) Celsius to Fahrenheit or (2) Fahrenheit to Celsius");

    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");

    let choice = choice.trim();

    match choice{
        "1" => {
            let result = celsius_to_fahrenheit(temperature);
            println!("{}°C = {}°F", temperature, result);
        }
        "2" => {
            let result = fahrenheit_to_celsius(temperature);
            println!("{}°F = {}°C", temperature, result);
        }
        _ => {
            println!("Invalid choice, please enter 1 or 2");
            continue;
        }
    }
    // Convert Again
    println!("\n Convert Again? Enter Yes or No");

    let mut again = String::new();

    io::stdin()
    .read_line(&mut again)
    .expect("failed to read line");

    match again.trim(){
        "No" => {
            println!("Thank you");
            break;
        }
        "Yes" => {
            continue;
        }
        _ => {
            println!("Invalid input....Enter Yes or No");
            break;
        }
    }
    
    }
  
}
