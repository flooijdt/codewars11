use std::collections::HashMap;

fn main() {
    order_weight("  101 35 67 2 09");
}

fn order_weight(s: &str) -> String {
    let mut vecko: Vec<Vec<char>> = Vec::new();
    let mut vecka: Vec<Vec<u32>> = Vec::new();
    let mut strr: Vec<&str> = s.split_whitespace().collect();

    for i in strr.iter_mut() {
        let vecky: Vec<char> = i.chars().collect();
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

    let mut biggest: u32 = 0;
    let mut vecka_ordered: Vec<Vec<u32>> = Vec::new();
    let mut vecka_sum: Vec<u32> = Vec::new();

    for v in vecka.iter() {
        let sum: u32 = v.iter().sum();
        vecka_sum.push(sum);
        // if sum >= biggest {
        //     vecka_ordered.push(v.to_vec());
        //     biggest = v.clone().iter().sum();
        // }
    }

    let mut hashy: HashMap<Vec<u32>, u32> = HashMap::new();

    let mut counter = 0;

    for v in vecka_sum.iter() {
        for i in vecka.iter() {
            hashy.insert(i.clone(), vecka_sum[counter]);
        }
        counter += 1;
    }

    // let mut bloom: Vec<u32> = *vecka.clone().iter().collect_into();

    let mut zipped = vecka.clone().sort();
    // .iter()
    // .enumerate()
    // .zip(vecka_sum.iter())
    // .collect::<Vec<&Vec<u32>>>();

    // let coll = zipped.sort();

    // println!("{:?}", &hashy);
    println!("{:?}", &zipped);
    // println!("{:?}", &zipped);
    "String".to_string()
}
