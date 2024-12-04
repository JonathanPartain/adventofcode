use regex::Regex;
fn part_one(input: &str) {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let res: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    let sum = calc(&res);
    println!("{sum}");
}
fn calc(list: &Vec<&str>) -> u32 {
    let num_match = Regex::new(r"\d{1,3}\,\d{1,3}").unwrap();
    let mut total_sum = 0;
    for item in list.iter() {
        let strs: Vec<_> = num_match.find_iter(item).map(|n| n.as_str()).collect();
        let item = strs[0];
        let numlist: Vec<&str> = item.split(",").collect();

        let mut res: u32 = 1;
        for num_to_mul in numlist {
            let num: u32 = num_to_mul.parse::<u32>().unwrap();
            res = num * res;
        }
        total_sum += res;
    }
    total_sum
}

fn part_two(input: &str) {
    let ro = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let res: Vec<_> = ro.find_iter(input).map(|m| m.as_str()).collect();
    let mut new_list = vec![];
    let mut add = true;
    for item in res {
        if item == "do()" {
            add = true;
            continue;
        }
        if item == "don't()" {
            add = false;
            continue;
        }
        if add {
            new_list.push(item);
        }
    }
    let sum = calc(&new_list);
    println!("{sum}");
}

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}
