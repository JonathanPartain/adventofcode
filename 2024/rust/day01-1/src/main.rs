fn main() {
    let input = include_str!("../input.txt");
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input.lines() {
        let items: Vec<&str> = line.split_whitespace().collect();

        // We know the size will always be two
        left.push(items[0].parse::<u32>().unwrap());
        right.push(items[1].parse::<u32>().unwrap());
    }
    // sort the vecs
    left.sort();
    right.sort();

    let mut dist = 0;
    for (index, value) in left.iter().enumerate() {
        let tmp_r = right[index];
        // value and index are at same position
        let tmp_dist = value.abs_diff(tmp_r);
        dist += tmp_dist;
    }
    println!("{dist}");
}
