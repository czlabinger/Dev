use std::io;

fn main() {
    
    println!("What do you want to convert to?");
    println!("Options are: \n Euro to convert US to Euro \n US to convert Euro to US");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_str = input.trim();

    println!("How much do you want to convert?");

    let mut input_value: String = String::new();
    io::stdin().read_line(&mut input_value).unwrap();
    let value: f64 = input_value.trim().parse().unwrap();



    let out: String;

    match input_str {
        "Euro" => {
            out = value.to_string() + " US$ are " + (value * 0.92 as f64).to_string().as_str() + " Euro";
        }
        
        "US" => {
            out = value.to_string() + " Euro are " + (value * 1.08 as f64).to_string().as_str() + " US$";
        }

        _ => {
            println!("Not a valid input!");
            return;
        } 
    }
    println!("{}", out);
}
