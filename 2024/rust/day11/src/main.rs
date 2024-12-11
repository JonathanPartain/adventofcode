use std::{collections::HashMap, time::Instant, u128};

fn main() {
    let start = Instant::now();
    let file = include_str!("../input.txt");
    let numbers: Vec<u128> = file
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap())
        .collect();

    let start_vec = numbers.clone();
    // done in part two
    //part_one(&start_vec);
    part_two(&start_vec);
    println!("time elapsed: {:.3?}", start.elapsed());
}

fn _part_one(numbers: &Vec<u128>) {
    let mut start_vec = numbers.clone();
    for _ in 1..=25 {
        start_vec = _rules_old(&start_vec);
    }

    println!("Part one: {}", start_vec.len());
}
fn part_two(numbers: &Vec<u128>) {
    let mut start_map: HashMap<u128, u128> = numbers
        .into_iter()
        .map(|val| (*val, 1))
        .collect::<HashMap<_, _>>();

    for i in 1..=75 {
        start_map = rules(&start_map);
        if i == 25 {
            println!("Part one: {}", start_map.values().sum::<u128>());
        }
    }

    println!("Part two: {}", start_map.values().sum::<u128>());
}

fn rules(l: &HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut ret_map = HashMap::default();

    for (&k, &v) in l.iter() {
        let str_rep = k.to_string();
        let str_len = str_rep.len();
        // the key was inserting v if none, else v+count. The line below
        // did it better than I could, dont understand why my version did
        // not work as intended tho.
        if k == 0 {
            *ret_map.entry(1).or_default() += v;
        } else if str_len % 2 == 0 {
            let first = &str_rep[..str_len / 2].trim_start_matches('0');
            let second = &str_rep[str_len / 2..].trim_start_matches('0');
            let f;
            let s;
            // if empty, add a 0
            if *first == "" {
                f = 0;
            } else {
                f = first.parse::<u128>().unwrap();
            }
            if *second == "" {
                s = 0;
            } else {
                s = second.parse::<u128>().unwrap();
            }
            *ret_map.entry(f).or_default() += v;
            *ret_map.entry(s).or_default() += v;
        } else {
            let multid = k * 2024;
            *ret_map.entry(multid).or_default() += v;
        }
    }
    return ret_map;
}
fn _rules_old(l: &Vec<u128>) -> Vec<u128> {
    let mut ret_vec: Vec<u128> = Vec::with_capacity(l.len() * 2);
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
