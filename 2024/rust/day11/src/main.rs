use std::u128;

fn main() {
    let file = include_str!("../input.txt");
    let numbers: Vec<u128> = file
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap())
        .collect();

    let mut start_vec = numbers.clone();
    for n in 1..=25 {
        start_vec = rules(&start_vec);
    }
    println!("You have {} stones", start_vec.len());
}

fn rules(l: &Vec<u128>) -> Vec<u128> {
    let mut ret_vec: Vec<u128> = vec![];
    for &n in l.iter() {
        let str_rep = n.to_string();
        let str_len = str_rep.len();
        if n == 0 {
            ret_vec.push(1);
        } else if str_len % 2 == 0 {
            let mut first = &str_rep[..str_len / 2].trim_start_matches('0');
            let mut second = &str_rep[str_len / 2..].trim_start_matches('0');
            // if empty, add a 0
            if *first == "" {
                first = &"0";
            }
            if *second == "" {
                second = &"0";
            }
            ret_vec.push(first.parse::<u128>().unwrap());
            ret_vec.push(second.parse::<u128>().unwrap());
        } else {
            ret_vec.push(n * 2024);
        }
    }
    return ret_vec;
}
