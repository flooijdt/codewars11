fn main() {
    println!("Hello, world!");
    // cooment
}

fn order_weight(s: &str) -> String {
    // your code
    let mut strr: Vec<&str> = s.split_whitespace().collect();
    println!("{:?}", strr);
    "String".to_string()
}
