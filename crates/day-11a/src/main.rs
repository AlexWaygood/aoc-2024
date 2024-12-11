fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-11.txt"
    ));

    let mut stones: Vec<Stone> = input
        .split_ascii_whitespace()
        .map(|digits| Stone(digits.parse().unwrap()))
        .collect();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(Stone::blink)
            .flatten()
            .collect();
    }

    println!("{}", stones.len());
}

#[derive(Debug)]
struct Stone(u64);

impl Stone {
    fn blink(self) -> [Option<Self>; 2] {
        if self.0 == 0 {
            [Some(Stone(1)), None]
        } else {
            let num_digits = self.0.ilog10() + 1;
            if num_digits % 2 == 0 {
                let multiplier = 10u64.pow(num_digits / 2);
                let first_stone = Stone(self.0 / multiplier);
                let second_stone = Stone(self.0 - first_stone.0 * multiplier);
                [Some(first_stone), Some(second_stone)]
            } else {
                [Some(Stone(self.0 * 2024)), None]
            }
        }
    }
}
