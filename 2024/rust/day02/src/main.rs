fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    //println!("{}", part1(&input));
}

fn part1(input: &str) -> u32 {
    let mut no_safes: u32 = 0;
    for line in input.lines() {
        let num_line: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        if follows_rules_1(&num_line) {
            no_safes += 1;
        }
    }
    no_safes
}

fn part2(input: &str) -> u32 {
    let mut count: u32 = 0;
    for line in input.lines() {
        let num_line: Vec<i32> = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();
        if follows_rules_2(&num_line) {
            count += 1;
        }
    }
    count
}

fn follows_rules_1(l: &Vec<i32>) -> bool {
    // make sure its sorted, no equals
    if l.is_sorted_by(|a, b| a < b) || l.is_sorted_by(|a, b| a > b) {
        let mut prev = l[0];
        for &num in &l[1..] {
            let diff = (prev - num).abs();
            let unacceptable_diff = diff < 1 || diff > 3;

            if unacceptable_diff {
                return false;
            }
            prev = num;
        }
        return true;
    }
    false
}

// getting subset stolen from here, big props
// https://github.com/nated0g/aoc/blob/master/src/solutions/y2024/day02.rs
fn follows_rules_2(l: &Vec<i32>) -> bool {
    // if it passes rule 1, done
    if follows_rules_1(l) {
        return true;
    }
    // brute force
    for skip_index in 0..l.len() {
        let subset = l
            .iter() // iterate
            .enumerate() //enumerate, (0, &num), (1, &num) etc
            // keep all values where index != 1 by
            .filter(|(i, _)| *i != skip_index) // dereferencing that index,
            .map(|(_, &n)| n) //transform the tuples into just the values again
            .collect();
        // if any of the subsets fit, it is acceptable
        if follows_rules_1(&subset) {
            return true;
        }
    }
    // no solution found, return false
    false
}
