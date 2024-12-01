use std::str::FromStr;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-1.txt"
    ));

    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    for line in input.lines() {
        let mut split_whitespace = line.split_whitespace();
        left.push(u32::from_str(split_whitespace.next().unwrap()).unwrap());
        right.push(u32::from_str(split_whitespace.next().unwrap()).unwrap());
        assert_eq!(split_whitespace.next(), None);
    }

    left.sort_unstable();
    right.sort_unstable();

    let answer: u32 = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| {
            if left > right {
                left - right
            } else {
                right - left
            }
        })
        .sum();

    println!("{answer}");
}
