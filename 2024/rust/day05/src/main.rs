use std::{u32, usize};

fn main() {
    let input = include_str!("../input.txt");

    let input_list: Vec<&str> = input.split("\n\n").collect();
    let rules_list: Vec<&str> = input_list[0].split("\n").collect();
    let mut lists_to_verify: Vec<&str> = input_list[1].split("\n").collect();

    // remove empty strings if there are any (like the last \n might create)
    let index_to_remove = lists_to_verify.iter().position(|x| *x == "").unwrap();
    lists_to_verify.remove(index_to_remove);

    // put page and rules into data structures
    let page_lists = build_lists(lists_to_verify);
    let rules = build_rules(rules_list);

    // sums for part 1 and 2
    let mut sum_middle = 0;
    let mut sum_wrong = 0;

    // do this for each list of pages
    for l in page_lists {
        // cloning cuz I dont know what im doing with borrow checking
        let l_c = l.clone();
        let sorted = sort_list(&l, &rules);
        let s_c = sorted.clone();

        // if the sorted and original vecs are equal, get middle of either
        // for part 1
        let (res_bool, len) = vecs_are_equal(l, sorted);
        if res_bool {
            let middle = len / 2;
            let num_to_add: u32 = *l_c.clone().iter().nth(middle as usize).unwrap();
            sum_middle += num_to_add;
        // use the list with the sorted items, get middle of them for part 2
        } else {
            let middle = len / 2;
            let num_to_add: u32 = *s_c.clone().iter().nth(middle as usize).unwrap();
            sum_wrong += num_to_add;
        }
    }
    println!("Part 1: {}", sum_middle);
    println!("Part 2: {}", sum_wrong);
}

fn vecs_are_equal(v1: Vec<u32>, v2: Vec<u32>) -> (bool, u32) {
    let matching = v1.iter().zip(&v2).filter(|&(a, b)| a == b).count();
    if matching == v1.len() && matching == v2.len() {
        return (true, v1.len() as u32);
    }
    return (false, v1.len() as u32);
}

fn get_index_of(list: Vec<u32>, c: u32) -> u32 {
    let i = list.iter().position(|x| *x == c).unwrap();
    return i as u32;
}

fn sort_list(list: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut sorted_list: Vec<u32> = vec![];
    for &item in list {
        if sorted_list.len() == 0 {
            sorted_list.push(item);
        } else {
            // Aight lets go
            // TODO: Make this into one or more funcs
            let mut insert_index: i32 = 0;
            //existing sorted items
            for x in sorted_list.clone() {
                // check all rules for every item
                for r in rules {
                    // rules match, x should be before item
                    if r.0 == x && r.1 == item {
                        let x_index = get_index_of(sorted_list.clone(), x);
                        // keep largest index
                        if x_index as i32 > insert_index {
                            insert_index = x_index as i32;
                        }
                        // Since we want to insert after X, += 1
                        insert_index += 1;
                    }
                    // insert item at index of x, shifting everything after
                    // to the right
                    else if r.0 == item && r.1 == x {
                        let x_index = get_index_of(sorted_list.clone(), x);
                        // keep smallest index
                        if (x_index as i32) < insert_index {
                            insert_index = x_index as i32;
                        }
                    }
                }
            }
            sorted_list.insert(insert_index as usize, item);
        }
    }
    return sorted_list;
}

fn build_lists(input_list: Vec<&str>) -> Vec<Vec<u32>> {
    let mut ret: Vec<Vec<u32>> = vec![];
    for str in input_list.iter() {
        let page_list: Vec<u32> = str.split(",").map(|c| c.parse::<u32>().unwrap()).collect();
        ret.push(page_list);
    }
    return ret;
}

fn build_rules(str_list: Vec<&str>) -> Vec<(u32, u32)> {
    let mut ret: Vec<(u32, u32)> = vec![];
    for str_tup in str_list.iter() {
        let nums_list: Vec<_> = str_tup
            .split("|")
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        ret.push((nums_list[0], nums_list[1]));
    }
    return ret;
}
