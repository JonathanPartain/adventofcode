use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    let chars: Vec<char> = input.lines().map(|l| l.chars().collect()).nth(0).unwrap();
    //println!("{:?}", chars);
    part_one(&chars);
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
