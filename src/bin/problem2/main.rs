use std::iter::from_fn;
use std::mem::replace;

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

fn parse_input(input: &str) -> Vec<bool> {
    input
        .lines()
        .map(|line| line.to_lowercase().parse().unwrap())
        .collect()
}

fn solve_part1(data: &[bool]) -> i64 {
    data.iter()
        .enumerate()
        .filter_map(
            |(rank, &value)| {
                if value { Some(rank as i64 + 1) } else { None }
            },
        )
        .sum()
}

fn solve_part2(data: &[bool]) -> i64 {
    process_layer(data).into_iter().filter(|&x| x).count() as i64
}

fn solve_part3(data: &[bool]) -> i64 {
    let mut data = data.to_vec();
    from_fn(move || {
        if data.is_empty() {
            return None;
        }

        let next_data = process_layer(&data);
        Some(replace(&mut data, next_data))
    })
    .map(|data| data.into_iter().filter(|&x| x).count() as i64)
    .sum()
}

fn process_layer(data: &[bool]) -> Vec<bool> {
    let ops = [|a, b| a && b, |a, b| a || b];
    data.chunks_exact(2)
        .zip(ops.into_iter().cycle())
        .map(|(pair, op)| op(pair[0], pair[1]))
        .collect()
}
