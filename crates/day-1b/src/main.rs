use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-1.txt"
    ));

    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right: FxHashMap<u32, u32> = FxHashMap::default();

    for line in input.lines() {
        let mut split_whitespace = line.split_whitespace();
        let mut next_number = || split_whitespace.next().unwrap().parse().unwrap();
        left.push(next_number());
        right
            .entry(next_number())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        assert_eq!(split_whitespace.next(), None);
    }

    let answer: u32 = left
        .into_iter()
        .map(|left| left * right.get(&left).copied().unwrap_or_default())
        .sum();

    println!("{answer}");
}
