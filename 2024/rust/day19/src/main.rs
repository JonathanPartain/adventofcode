use std::{collections::HashMap, process::exit};

fn main() {
    let sections: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let str_frags = sections[0];
    let frags: Vec<&str> = str_frags.split(", ").collect();

    let patterns = sections[1];
    part_one(&frags, patterns);
}

// solution:
// if target == "" -> true
// else
//  for frag in frags:
//      let x = frag.len()
//      if target[..x] == frag:
//          return solution(target[x..], frags)
// return false?

fn can_form_word(
    target: &str,
    frags: &Vec<&str>,
    memo: &mut HashMap<String, (bool, usize)>,
) -> (bool, usize) {
    if target == "" {
        return (true, 1);
    }
    if let Some(&cached) = memo.get(target) {
        return cached; // Return cached result
    }
    let mut total_count = 0;
    let mut can_form = false;

    for frag in frags.iter() {
        let x = frag.len();
        if target.len() >= x && target[..x] == **frag {
            let (sub_can_form, sub_count) = can_form_word(&target[x..], frags, memo);
            if sub_can_form {
                can_form = true;
            }
            total_count += sub_count;
        }
    }
    memo.insert(target.to_string(), (can_form, total_count));
    return (can_form, total_count);
}
fn part_one(frags: &Vec<&str>, patterns: &str) {
    let mut count = 0;
    let mut memo = HashMap::new();
    let mut tot = 0;
    for p in patterns.lines() {
        let (can_be_formed, c) = can_form_word(p, frags, &mut memo);
        if can_be_formed {
            tot += c;
            count += 1;
        }
    }
    println!("Part 1: {}", count);
    println!("Part 2: {:?}", tot);
}
