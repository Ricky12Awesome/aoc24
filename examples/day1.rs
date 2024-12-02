#![feature(iter_array_chunks)]

use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day1.txt");

// part 1: 2815556
// part 2: 23927637
fn main() {
    let (mut left, mut right) = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .array_chunks::<2>()
                .map(|[a, b]| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .next()
                .unwrap()
        })
        .collect::<(Vec<usize>, Vec<usize>)>();

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>();

    let left = left.iter().copied().counts();
    let right = right.iter().copied().counts();

    println!("{}", sum);

    let sum = left
        .iter()
        .filter(|(n, _)| right.contains_key(n))
        .map(|(n, count)| n * right[n] * count)
        .sum::<usize>();

    println!("{}", sum);
}
