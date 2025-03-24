use std::iter::from_fn;

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!(
        "The answer to part 1 is {answer1}",
        answer1 = solve_part1(&input)
    );
    println!(
        "The answer to part 2 is {answer2}",
        answer2 = solve_part2(&input)
    );
    println!(
        "The answer to part 3 is {answer3}",
        answer3 = solve_part3(&input)
    );
}

struct Entry {
    reading: &'static str,
    base: u32,
}

fn parse_input(input: &'static str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (reading, base) = line.split_once(' ').unwrap();
            Entry {
                reading,
                base: base.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_part1(data: &[Entry]) -> i64 {
    data.iter().map(|Entry { base, .. }| *base as i64).sum()
}

fn solve_part2(data: &[Entry]) -> i64 {
    data.iter()
        .map(|&Entry { reading, base }| i64::from_str_radix(reading, base).unwrap())
        .sum()
}

fn solve_part3(data: &[Entry]) -> String {
    to_custom_base(solve_part2(data))
}

fn to_custom_base(mut x: i64) -> String {
    let res: Vec<_> = from_fn(|| {
        if x == 0 {
            return None;
        }

        let res = (x % 65) as u8;
        x /= 65;
        return Some(res);
    })
    .map(to_custom_digit)
    .collect();

    res.into_iter().rev().map(|d| d as char).collect()
}

fn to_custom_digit(x: u8) -> u8 {
    match x {
        x if x < 10 => x + b'0',
        x if x < 36 => (x - 10) + b'A',
        x if x < 62 => (x - 36) + b'a',
        62 => b'!',
        63 => b'@',
        64 => b'#',
        _ => unreachable!(),
    }
}
