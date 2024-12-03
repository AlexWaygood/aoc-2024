use std::str::FromStr;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-3.txt"
    ));

    let dont_re = regex::Regex::new(r"don't\(\)").unwrap();
    let do_re = regex::Regex::new(r"do\(\)").unwrap();

    let mut donts = dont_re.find_iter(input);
    let mut dos = do_re.find_iter(input);

    let donts_by_ref = donts.by_ref();
    let dos_by_ref = dos.by_ref();

    let mut cursor_position = 0;
    let mut program = String::new();
    let mut state = State::LookingForDonts;

    loop {
        match state {
            State::LookingForDonts => {
                let Some(next_dont) =
                    donts_by_ref.find(|re_match| re_match.start() > cursor_position)
                else {
                    program.push_str(&input[cursor_position..]);
                    break;
                };
                program.push_str(&input[cursor_position..next_dont.start()]);
                cursor_position = next_dont.end();
                state = State::LookingForDos;
            }
            State::LookingForDos => {
                let Some(next_do) = dos_by_ref.find(|re_match| re_match.start() > cursor_position)
                else {
                    break;
                };
                cursor_position = next_do.end();
                state = State::LookingForDonts;
            }
        }
    }

    let mul_re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result: u32 = 0;
    for caps in mul_re.captures_iter(&program) {
        let (_, [first, second]) = caps.extract();
        result += u32::from_str(first).unwrap() * u32::from_str(second).unwrap();
    }

    println!("{result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    LookingForDonts,
    LookingForDos,
}
