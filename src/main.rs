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
    for v in vecko.iter_mut() {
        let mut ve: Vec<u32> = Vec::new();
        vecka.push(ve.clone());
        for c in v {
            ve.push(c.to_digit(10).unwrap());
        }
        if !ve.is_empty() {
            vecka.push(ve.clone());
        } else {
            continue;
        }
        // vecka.push(ve.clone());
        //     vecka.push(veckb.clone());
    }

    let mut counter = 0;
    let mut dif = 0;
    for v in vecka.clone().iter_mut() {
        if v.is_empty() {
            vecka.remove(counter - dif);
            dif += 1;
        }
        counter += 1;
    }
    println!("{:?}", &vecka);
    "String".to_string()
}
