use std::fmt::write;

fn main() {
    let input = include_str!("../small.txt");
    let mut input_list: Vec<&str> = input.split("\n").collect();
    // remove last index, as it is just an empty str
    input_list.remove(input_list.len() - 1);

    let num_list: Vec<Vec<u32>> = input_list
        .iter()
        .map(|item| {
            item.split(" ")
                .map(|s| s.replace(":", "").parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    for i in num_list.iter() {
        do_math(&i.to_vec());
    }
}

fn add(x: &u32, y: &u32) -> u32 {
    x + y
}
fn mul(x: &u32, y: &u32) -> u32 {
    x * y
}
/*
fn do_op(x, y, op) {
    ret
}
*/
/* Recursive function.
 * Take 2 numbers, do all possible math.
 * For each result, take next number, do all possible math
 * end up with a list/struct whatever of different results. if any are equal
 * to first num, success!
 */

fn do_math(numbers: &Vec<u32>) {
    let res = numbers[0];

    let mut total = numbers[1];
    let operations = vec![add, mul];

    for (pos_n, n) in numbers.iter().enumerate() {
        let mut tmp = 0;
        for (pos_m, m) in numbers.iter().enumerate() {
            if pos_n == 0 {
                continue;
            }
            // more right only
            if pos_m <= pos_n {
                continue;
            }

            for op in &operations {
                let last_op = op(n, m) + tmp;
                if last_op == res {
                    println!("Done!, {}", last_op);
                }
            }
        }
    }
}
