#![feature(iter_map_windows)]

use std::cmp::Ordering;
use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &str = include_str!("../inputs/day5.txt");

// part1: 6612
// part2: 4944
fn main() {
    let (rules, update) = INPUT.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .into_group_map();

    let part1 = update
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|line| {
            line.iter()
                .map_windows(|[x, y]| rules.get(x).map(|v| v.contains(y)).unwrap_or(false))
                .all_equal()
        })
        .map(|line| line[line.len() / 2])
        .sum::<i32>();

    println!("{:?}", part1);

    let part2 = update
        .lines()
        .par_bridge()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .filter(|line| {
            !line
                .iter()
                .map_windows(|[x, y]| rules.get(x).map(|v| v.contains(y)).unwrap_or(false))
                .all_equal()
        })
        .map(|mut line| {

            line.sort_by(|x, y|
                match rules.get(x) {
                    Some(v) if v.contains(y) => Ordering::Less,
                    _ => Ordering::Greater,
                }
            );

            line
        })
        .map(|line| line[line.len() / 2])
        .sum::<i32>();;

    println!("{}", part2);
}
