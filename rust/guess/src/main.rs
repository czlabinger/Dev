use rand::Rng;

fn main() {
    println!("Guess a number between 0 and 100");
    let mut rng = rand::thread_rng();
    let val: u16 = rng.gen_range(0..100);
    let mut tries: u16 = 0;

    while tries < 10 {
        let mut line = String::new();
        let input: u16 = std::io::stdin().read_line(&mut line).unwrap() as u16;

        if input > val {
            println!("Number was too big!");
        } else if input < val {
            println!("Number was too small!");
        } else {
            println!("You got it!");
            break;
        }
        tries += 1;
    }
}
