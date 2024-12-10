use std::collections::VecDeque;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-9.txt"
    ));

    let mut disk: VecDeque<Option<usize>> = VecDeque::with_capacity(20000);
    for (i, c) in input.char_indices() {
        if i % 2 == 0 {
            let file_id = i / 2;
            let file_length = c.to_digit(10).unwrap();
            for _ in 0..file_length {
                disk.push_back(Some(file_id));
            }
        } else {
            let free_space = c.to_digit(10).unwrap();
            for _ in 0..free_space {
                disk.push_back(None);
            }
        }
    }

    let mut new_disk: Vec<usize> = Vec::with_capacity(10000);

    while let Some(item) = disk.pop_front() {
        match item {
            Some(item) => new_disk.push(item),
            None => loop {
                match disk.pop_back() {
                    Some(None) => continue,
                    Some(Some(item)) => {
                        new_disk.push(item);
                        break;
                    }
                    None => break,
                }
            },
        }
    }

    let answer: usize = new_disk
        .iter()
        .enumerate()
        .map(|(index, file_id)| index * file_id)
        .sum();

    println!("{answer}");
}
