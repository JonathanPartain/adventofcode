use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let mut input_list: Vec<&str> = input.split("\n").collect();
    // remove last index, as it is just an empty str
    input_list.remove(input_list.len() - 1);

    let num_list: Vec<Vec<u64>> = input_list
        .iter()
        .map(|item| {
            item.split(" ")
                .map(|s| s.replace(":", "").parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for i in num_list.iter() {
        let r = do_math(&i.to_vec());
        p1_sum += r.0;
        p2_sum += r.1;
    }

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
    println!("Exection time: {:.3?}", start.elapsed());
}

fn add(x: &u64, y: &u64) -> u64 {
    x + y
}
fn mul(x: &u64, y: &u64) -> u64 {
    x * y
}

fn cat(x: &u64, y: &u64) -> u64 {
    let sx = x.to_string();
    let sy = y.to_string();
    (sx + &sy).parse::<u64>().unwrap()
}

fn do_ops(x: &u64, y: &u64) -> Vec<u64> {
    let ops = vec![add, mul];
    let mut ret_vec: Vec<u64> = vec![];
    for op in ops {
        let res = op(&x, &y);
        ret_vec.push(res);
    }
    return ret_vec;
}
fn do_ops_cat(x: &u64, y: &u64) -> Vec<u64> {
    let ops = vec![add, mul, cat];
    let mut ret_vec: Vec<u64> = vec![];
    for op in ops {
        let res = op(&x, &y);
        ret_vec.push(res);
    }
    return ret_vec;
}

fn vec_multi(prev_res: &Vec<u64>, num: &u64, func: fn(&u64, &u64) -> Vec<u64>) -> Vec<u64> {
    let mut ret: Vec<u64> = vec![];
    for n in prev_res.iter() {
        ret.extend(func(&n, &num));
    }
    return ret;
}

fn do_math(numbers: &Vec<u64>) -> (u64, u64) {
    let mut p1sum = 0;
    let mut p2sum = 0;
    let should_equal = numbers[0];
    let first = numbers[1];

    let mut p1: Vec<u64> = vec![];
    p1.push(first);

    let mut p2: Vec<u64> = vec![];
    p2.push(first);

    for (pos_n, n) in numbers.iter().enumerate() {
        if pos_n > 1 {
            let p1_replacer = vec_multi(&p1, &n, do_ops);
            p1 = p1_replacer;
            let p2_replacer = vec_multi(&p2, &n, do_ops_cat);
            p2 = p2_replacer;
        }
    }
    if p1.contains(&should_equal) {
        p1sum += should_equal;
    }
    if p2.contains(&should_equal) {
        p2sum += should_equal;
    }
    (p1sum, p2sum)
}
