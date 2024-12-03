use std::str::FromStr;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-3.txt"
    ));

    let mul_regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result: u32 = 0;
    for caps in mul_regex.captures_iter(input) {
        let (_, [first, second]) = caps.extract();
        result += u32::from_str(first).unwrap() * u32::from_str(second).unwrap();
    }

    println!("{result}");
}
