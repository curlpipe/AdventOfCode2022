use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input.txt").expect("File not found");
    let result: u32 = data
        .split('\n')
        .filter(|rs| rs != &"")
        .map(|rs| rs.split_at(rs.len() / 2))
        .map(|(one, two)| one.chars().filter(|c| two.contains(*c)).nth(0).unwrap())
        .map(|cc| (cc as u32) - if cc.is_uppercase() { 38 } else { 96 })
        .sum();
    println!("{:?}", result);
}
