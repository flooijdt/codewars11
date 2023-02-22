fn main() {
    order_weight("  101 35 67 2 09");
}

fn order_weight(s: &str) -> String {
    // your code
    let mut vecko: Vec<Vec<u32>> = Vec::new();
    let mut strr: Vec<&str> = s.split_whitespace().collect();
    for i in strr.iter_mut() {
        let mut vecky: Vec<u32> = i.chars().to_digit(10).collect();
        vecko.push(vecky);
    }
    println!("{:?}", vecko);
    "String".to_string()
}
