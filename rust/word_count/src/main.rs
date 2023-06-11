use std::collections::HashMap;

fn main() {
    let mut words: HashMap<String, u16> = HashMap::new();

    'run: {
        let mut line = String::new();
        let input: usize = std::io::stdin().read_line(&mut line).unwrap();

        for entry in words.values() {
            println!("{}", entry);
            break;
        }
    }
}
