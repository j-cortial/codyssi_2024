use std::collections::{HashMap, HashSet};

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

type Loc = &'static str;

type Data = Vec<[Loc; 2]>;

fn parse_input(input: &'static str) -> Data {
    input
        .lines()
        .map(|line| {
            let pair = line.split_once(" <-> ").unwrap();
            [pair.0, pair.1]
        })
        .collect()
}

fn solve_part1(data: &Data) -> usize {
    let unique_locs: HashSet<_> = data
        .iter()
        .flat_map(|entry| entry.iter().copied())
        .collect();
    unique_locs.len()
}

type Graph = HashMap<Loc, Vec<Loc>>;

fn solve_part2(data: &Data) -> usize {
    let graph = make_graph(data);
    let mut front = vec!["STT"];
    let mut visited: HashSet<_> = front.iter().copied().collect();

    for _ in 0..3 {
        let mut new_front = vec![];
        for &f in front.iter() {
            for &candidate in graph.get(f).unwrap() {
                if visited.insert(candidate) {
                    new_front.push(candidate);
                }
            }
        }
        front = new_front;
    }

    visited.len()
}

fn solve_part3(data: &Data) -> usize {
    let graph = make_graph(data);
    let mut front = vec![vec!["STT"]];
    let mut visited: HashSet<_> = front.last().unwrap().iter().copied().collect();

    while !front.last().unwrap().is_empty() {
        let mut new_front = vec![];
        for &f in front.last().unwrap().iter() {
            for &candidate in graph.get(f).unwrap() {
                if visited.insert(candidate) {
                    new_front.push(candidate);
                }
            }
        }
        front.push(new_front);
    }

    front
        .into_iter()
        .enumerate()
        .fold(0, |acc, (l, v)| acc + l * v.len())
}

fn make_graph(data: &Data) -> Graph {
    data.iter().fold(Graph::default(), |mut acc, x| {
        for &[a, b] in [x, &[x[1], x[0]]].into_iter() {
            acc.entry(a).or_default().push(b);
        }
        acc
    })
}
