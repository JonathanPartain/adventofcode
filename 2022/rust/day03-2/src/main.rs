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
    let input_list: Vec<&str> = input.trim().split("\n").collect();
    let mut counter = 0;
    let mut score = 0;
    while counter < input_list.len() {
        let c1 = input_list[counter];
        let c2 = input_list[counter + 1];
        let c3 = input_list[counter + 2];

        for c in c1.chars() {
            if c2.contains(c) && c3.contains(c) {
                score += get_priority(c);
                break;
            }
        }

        counter += 3;
    }

    println!("{}", score);
}
