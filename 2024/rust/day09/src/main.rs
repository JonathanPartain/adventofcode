use std::{collections::VecDeque, process::exit, usize};

fn main() {
    let input = include_str!("../small.txt");
    let chars: Vec<char> = input.lines().map(|l| l.chars().collect()).nth(0).unwrap();
    let list: &str = input.lines().nth(0).unwrap();
    //part_one(&chars);
    part_two(&chars);
}

fn build_disk(char_list: &Vec<char>) -> Vec<i32> {
    let mut return_vec = vec![];

    for (index, val) in char_list.iter().enumerate() {
        let n = val.to_digit(10).unwrap();
        if index % 2 == 0 {
            let file_num = index / 2;
            // $value amount of indexes
            for _ in 0..n {
                return_vec.push(file_num as i32);
            }
        } else {
            // $ value amount of char '.'
            for _ in 0..n {
                return_vec.push(-1);
            }
        }
    }
    return return_vec;
}

fn calc_checksum(l: &Vec<u32>) -> u64 {
    let mut chksum: u64 = 0;
    for (i, v) in l.iter().enumerate() {
        let n = i as u64 * *v as u64;
        chksum += n;
    }
    return chksum;
}

fn part_one(char_list: &Vec<char>) {
    let disk = build_disk(char_list);

    let mut disk_q = VecDeque::from(disk.clone());
    let disk_len = disk_q.len();
    let mut ret_vec: Vec<u32> = Vec::with_capacity(disk_len);
    let mut _popped_dots = 0;

    for _ in 0..disk_len {
        // Access the front item
        if let Some(&front) = disk_q.front() {
            if front == -1 {
                // Remove the item from the front
                disk_q.pop_front();
                // Replace it with an item from the back if available
                while let Some(back) = disk_q.pop_back() {
                    match back {
                        -1 => _popped_dots += 1,
                        _ => {
                            ret_vec.push(back as u32);
                            break;
                        }
                    }
                }
            } else {
                ret_vec.push(disk_q.pop_front().unwrap() as u32);
            }
        }
    }

    let check_sum = calc_checksum(&ret_vec);
    println!("{:?}", check_sum);
}
// insert_item(&mut ret_vec, &size, &-1, &saved_item) {
fn insert_item(
    list: &Vec<i32>,
    size: &usize,
    item: &i32,
    check_for: &i32,
) -> ((usize, usize), bool) {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut inserting: bool = false;

    println!("Replace item {:?} for size {:?}", item, size);
    for (k, &v) in list.iter().enumerate() {
        if v != *check_for && inserting && end >= start {
            let diff: i32 = end as i32 - start as i32 + 1;
            println!(
                "diff {:?} end {:?} start {:?} size {:?}",
                diff, end, start, size
            );
            if diff >= *size as i32 {
                println!("start: {start}, end: {end}");
                // remove
                return ((start, end - 1), true);
            }
            inserting = false;
            start = 0;
            end = 0;
            continue;
        }
        if v == -1 && !inserting {
            inserting = true;
            start = k;
        }
        if v == -1 && inserting {
            end = k;
        }
    }

    return ((0, 0), false);
}

fn part_two(char_list: &Vec<char>) {
    let disk = build_disk(char_list);

    let mut disk_v = Vec::from(disk.clone());
    let disk_len = disk_v.len();
    let mut head = 0;
    let mut tail = disk_len - 1;

    let mut ret_vec: Vec<i32> = disk_v.clone();
    let mut _popped_dots = 0;

    // first or last item
    let mut saved_item = disk[tail];
    let mut size: usize = 0;

    for i in (0..disk_len).rev() {
        let tail_item = disk_v[i];
        if tail_item != saved_item {
            if saved_item == -1 {
                // skip
                size = 1;
                saved_item = tail_item;
                continue;
            }
            println!("Found {} of type {} at index {}", size, saved_item, i);
            println!(
                "{:?}",
                ret_vec
                    .iter()
                    .map(|&n| if n == -1 {
                        ".".to_string()
                    } else {
                        n.to_string()
                    })
                    .collect::<String>()
            );
            // insert
            let mut remove;
            let range;
            println!("size before insert: {}", size);
            (range, remove) = insert_item(&mut ret_vec, &size, &saved_item, &-1);
            println!("size after insert: {}", size);
            if remove {
                let add_s = range.0;
                let add_e = add_s + size - 1;

                if add_s < i {
                    println!("add {saved_item} at {add_s} to {add_e}");
                    if add_s == add_e {
                        ret_vec[add_s] = saved_item;
                    } else {
                        for x in add_s..=add_e {
                            println!("ret_vec[{x}] = {saved_item}");
                            ret_vec[x] = saved_item;
                        }
                    }
                    let rem_s = i + 1;
                    let rem_e = rem_s + &size - 1;

                    println!("Remove old items at {rem_s} to {rem_e}");
                    if rem_s == rem_e {
                        ret_vec[rem_s] = -1;
                    } else {
                        for x in 1..=size {
                            println!("ret_vec[{}] = -1", x + i);
                            ret_vec[x + i] = -1;
                        }
                    }
                }
            }
            size = 0;
            println!(
                "{:?}",
                ret_vec
                    .iter()
                    .map(|&n| if n == -1 {
                        ".".to_string()
                    } else {
                        n.to_string()
                    })
                    .collect::<String>()
            );

            saved_item = tail_item;
        }
        size += 1;
        head += 1;
        println!();
    }
}
