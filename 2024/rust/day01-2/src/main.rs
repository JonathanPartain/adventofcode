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

    let mut similarity_score = 0;

    for (_, value) in left.iter().enumerate() {
        let count: usize = right.iter().filter(|x| *x == value).count();
        let score: u32 = value * count as u32;
        similarity_score += score;
    }
    println!("{similarity_score}");
}
