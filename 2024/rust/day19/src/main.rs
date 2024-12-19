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

fn can_form_word(target: &str, frags: &Vec<&str>, memo: &mut HashMap<String, bool>) -> bool {
    if target == "" {
        return true;
    }
    if let Some(&cached) = memo.get(target) {
        return cached; // Return cached result
    }
    for frag in frags.iter() {
        let x = frag.len();
        if target.len() >= x && target[..x] == **frag {
            if can_form_word(&target[x..], frags, memo) {
                memo.insert(target.to_string(), true);
                return true;
            }
        }
    }
    memo.insert(target.to_string(), false);
    return false;
}
fn part_one(frags: &Vec<&str>, patterns: &str) {
    let mut count = 0;
    let mut memo = HashMap::new();
    for p in patterns.lines() {
        if can_form_word(p, frags, &mut memo) {
            println!("{} can be formed", p);
            count += 1;
        }
    }
    println!("number of word formed: {}", count);
}
