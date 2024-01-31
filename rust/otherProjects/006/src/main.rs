use rand::{self, Rng};

fn main() {

    let num = rand::thread_rng().gen_range(0..=2);

    match num {
        0 => {
            fn_one();
        }
        1 => {
            fn_two();
        }
        2 => {
            fn_three();
        }
        _ => println!("This shouldnt happen!"),
    }

}

fn fn_one() {
    println!("This is fn_one!");
}

fn fn_two() {
    println!("This is fn_two!");
}

fn fn_three() {
    println!("This is fn_three");
}
