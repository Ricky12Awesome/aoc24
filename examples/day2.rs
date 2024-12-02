#![feature(iter_map_windows)]

use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day2.txt");

// part 1: 359
// part 2: 418
fn main() {
    let example = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 5 10 15 20
1 3 6 7 9"#;
    // let input = example
    let input = INPUT
        .lines()
        .map(|line| line.split(' ').flat_map(str::parse::<isize>).collect_vec())
        .collect_vec();

    let part1 = input
        .iter()
        .filter(|line| is_safe(line))
        .count();

    println!("{}", part1);

    let part2 = input
        .iter()
        .map(|line| {
            if is_safe(line) {
                return true;
            }

            (0..line.len())
                .map(|i| {
                    let mut line = line.clone();
                    line.remove(i);
                    line
                })
                .any(|line| is_safe(&line))
        })
        .filter(|b| *b)
        .count();

    println!("{}", part2);
}

fn is_safe(slice: &[isize]) -> bool {
    let mut iter = slice
        .iter()
        .copied()
        .map_windows(|[a, b]| (a.cmp(b), a.abs_diff(*b)))
        .map(|(order, n)| (order, (1..=3).contains(&n)))
        .peekable();

    let value = iter.peek().unwrap();

    // edge case if all are equal and are within 1 to 3
    if value.0.is_eq() || !value.1 {
        return false;
    }

    iter.all_equal()
}