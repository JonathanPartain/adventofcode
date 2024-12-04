use std::char;

fn get_priority(c: char) -> u32 {
    let scorecard = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let s = scorecard.chars().position(|t| t == c).unwrap();
    let add_one = s as u32 + 1;
    add_one
}
fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../small.txt");
    let input_list = input.split("\n");

    let mut score = 0;
    for item in input_list {
        let l = item.len();
        let c1 = &item[0..&l / 2];
        let c2 = &item[&l / 2..];
        for c in c1.chars() {
            if c2.contains(c) {
                score += get_priority(c);
                break;
            }
        }
    }
    println!("{}", score);
}
