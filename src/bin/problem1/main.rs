fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("The answer to part 1 is {answer1}", answer1 = solve_part1(&input));
    println!("The answer to part 2 is {answer2}", answer2 = solve_part2(&input));
    println!("The answer to part 3 is {answer3}", answer3 = solve_part3(&input));
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_part1(data: &[i64]) -> i64 {
    data.iter().sum()
}

fn solve_part2(data: &[i64]) -> i64 {
    let mut data = data.to_vec();
    data.sort();
    data.iter().rev().skip(6).sum()
}

fn solve_part3(data: &[i64]) -> i64 {
    data.iter().fold((0, 1), |(acc, sign), x| {
        (acc + sign * x, -sign)
    }).0
}
