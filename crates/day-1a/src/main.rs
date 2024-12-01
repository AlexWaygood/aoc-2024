fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-1.txt"
    ));

    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right = Vec::<u32>::with_capacity(1000);

    for line in input.lines() {
        let mut split_whitespace = line.split_whitespace();
        let mut next_number = || split_whitespace.next().unwrap().parse().unwrap();
        left.push(next_number());
        right.push(next_number());
        assert_eq!(split_whitespace.next(), None);
    }

    left.sort_unstable();
    right.sort_unstable();

    let answer: u32 = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    println!("{answer}");
}
