use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input.txt").expect("File not found!");

    // Split each elf up
    let total: u32 = data
        .split("\n\n")
        .map(|elf| {
            // Split up the numbers, parse them (skipping invalid numbers) and sum them
            elf.split('\n')
                .filter_map(|calories| calories.parse::<u32>().ok())
                .sum::<u32>()
        })
        // Sum the top 3 calorie counts
        .sorted()
        .rev()
        .take(3)
        .sum();

    // Done!
    println!("{total}");
}
