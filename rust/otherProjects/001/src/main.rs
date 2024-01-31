use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_str: &str = input.trim();
    let input_u32: u32 = input_str.parse::<u32>().unwrap();

    let mut out: u32 = 1;
    for i in 1..=input_u32 {
        out = out * i; 
    }
    println!("{}! = {}", input_u32, out);
}
