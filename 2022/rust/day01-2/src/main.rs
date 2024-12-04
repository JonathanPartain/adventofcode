// lots of help from here, basically copied
// https://github.com/timvisee/advent-of-code-2022/blob/master/day01a/src/main.rs
fn main() {
    // split by 2 newlines
    // add all items in each list
    // sort list
    // pick index 0
    let elf_list = include_str!("../input.txt");
    let mut list = elf_list
        .split("\n\n") // two lines separate each elf
        // run function on each elf
        .map(|elf| {
            elf.lines() // get each line from elf
                .map(|ei| ei.parse::<u32>().unwrap()) // parse each line, get value with unwrap()
                .sum::<u32>() // sum all items
        }) // we now have a list of items, each item representing the total count
        .collect::<Vec<u32>>();
    list.sort();
    let top_3 = list.into_iter().rev().take(3).sum::<u32>();

    println!("{}", top_3);
}
