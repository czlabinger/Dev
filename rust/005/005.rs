// match
fn main() {
    let stateCode = "MH";
    let state = match stateCode {
        "MH" => "MH",
        "KL" => "KL",
        "KA" => "KA",
        _ => "Unknown"
    };
    println!("{}",state);
}
