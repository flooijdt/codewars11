fn main() {
    order_weight("  101 35 67 2 09");
}

fn order_weight(s: &str) -> String {
    // your code
    let mut vecko: Vec<Vec<char>> = Vec::new();
    let mut vecka: Vec<Vec<u32>> = Vec::new();
    let mut veckb: Vec<u32> = Vec::new();
    let mut strr: Vec<&str> = s.split_whitespace().collect();
    for i in strr.iter_mut() {
        let mut vecky: Vec<char> = i.chars().collect();
        vecko.push(vecky);
    }
    // for v in vecko.iter_mut() {
    //     for c in v {
    //         veckb.push(c.to_digit(10).unwrap());
    //     }
    //     vecka.push(veckb.clone());
    // }
    vecka = vecko.into();
    println!("{:?}", vecka);
    "String".to_string()
}
