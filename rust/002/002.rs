//changeable and not changeable variables
fn main() {
    let unchangable = 1; // not changeable
    let mut changeable = 2; //changeable

    println!("{}",changeable);
    changeable = 3;
    println!("{}",changeable);
}
