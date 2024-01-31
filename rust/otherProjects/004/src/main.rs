use std::io;

fn main() {
    let mut input_value: String = String::new();
    io::stdin().read_line(&mut input_value).unwrap();
    let value: u32 = input_value.trim().parse().unwrap();

    for i in 0..value {
        for j in 0..=i {
            print!("*");
        }
        println!("");
    }
}
