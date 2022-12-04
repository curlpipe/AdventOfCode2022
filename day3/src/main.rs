use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input.txt").expect("File not found");
    let rucksacks: Vec<&str> = data.split('\n').filter(|rs| rs != &"").collect();
    let result: u32 = rucksacks
        .chunks(3)
        .map(|rss| (rss[0], rss[1], rss[2]))
        .map(|(one, two, three)| {
            one.chars()
                .filter(|c| two.contains(*c) && three.contains(*c))
                .nth(0)
                .unwrap()
        })
        .map(|cc| (cc as u32) - if cc.is_uppercase() { 38 } else { 96 })
        .sum();
    println!("{}", result);
}
