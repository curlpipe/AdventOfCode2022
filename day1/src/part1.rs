use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input.txt").expect("File not found!");

    // Split each elf up
    let (elf_no, calorie_count) = data
        .split("\n\n")
        .map(|elf| {
            // Split up the numbers, parse them (skipping invalid numbers) and sum them
            elf.split('\n')
                .filter_map(|calories| calories.parse::<u32>().ok())
                .sum::<u32>()
        })
        // Number the elves and select the one with the most calories
        .enumerate()
        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
        .expect("More than one elf has the maximum calorie count");

    // Done!
    println!("Elf {elf_no} has the most at {calorie_count}");
}
