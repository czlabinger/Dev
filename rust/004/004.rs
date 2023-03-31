//String and String obj
fn main() {
    //String variable
    let name:&str="Stoffi05";
    println!("My name is: {}",name);

    //String obj
    let mut nameObj = String::new(); //Or let nameObj = String::from("Stoffi05");
    nameObj.push_str("Stoffi05");
    println!("My name is: {}",nameObj);
}
