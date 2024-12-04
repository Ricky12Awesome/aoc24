use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day4.txt");

// part1: 2493
// part2: 1890
fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let part1 = part1(&grid);

    println!("{}", part1);

    let part2 = part2(&grid);

    println!("{}", part2);
}

fn part1(grid: &[Vec<char>]) -> usize {
    let search = b"XMAS".map(char::from).map(Some);

    let offsets = [
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(3, 0), (2, 0), (1, 0), (0, 0)],
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 3), (0, 2), (0, 1), (0, 0)],
        [(0, 0), (1, 1), (2, 2), (3, 3)],
        [(0, 0), (1, -1), (2, -2), (3, -3)],
        [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        [(3, 3), (2, 2), (1, 1), (0, 0)],
        [(3, -3), (2, -2), (1, -1), (0, 0)],
        [(-3, 3), (-2, 2), (-1, 1), (0, 0)],
        [(-3, -3), (-2, -2), (-1, -1), (0, 0)],
    ];

    count(grid, search, &offsets)
}

fn part2(grid: &[Vec<char>]) -> usize {
    let search = b"MSASM".map(char::from).map(Some);

    let offsets = [
        [(-1, 1), (1, 1), (0, 0), (1, -1), (-1, -1)],
        [(1, 1), (-1, 1), (0, 0), (-1, -1), (1, -1)],
        [(-1, 1), (-1, -1), (0, 0), (1, -1), (1, 1)],
        [(-1, -1), (-1, 1), (0, 0), (1, 1), (1, -1)],
    ];

    count(grid, search, &offsets)
}

fn count<const N: usize>(
    grid: &[Vec<char>],
    search: [Option<char>; N],
    offsets: &[[(isize, isize); N]],
) -> usize {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    offsets
        .iter()
        .flat_map(|offset| (0..height).map(move |y| (offset, y)))
        .flat_map(|(offset, y)| (0..width).map(move |x| (offset, x, y)))
        .map(|(offset, x, y)| get(grid, x, y, offset))
        .filter(|(_, result)| result.eq(&search))
        .collect::<HashSet<_>>()
        .len()
}

fn get_char(
    grid: &[Vec<char>],
    x: isize,
    y: isize,
    (offset_x, offset_y): (isize, isize),
) -> Option<((usize, usize), char)> {
    let y = y.checked_add(offset_y)?;
    let x = x.checked_add(offset_x)?;
    let y = usize::try_from(y).ok()?;
    let x = usize::try_from(x).ok()?;

    grid.get(y).and_then(|row| Some(((x, y), *row.get(x)?)))
}

fn get<const N: usize>(
    grid: &[Vec<char>],
    x: isize,
    y: isize,
    offsets: &[(isize, isize); N],
) -> ([Option<(usize, usize)>; N], [Option<char>; N]) {
    let value = offsets.map(|offset| get_char(grid, x, y, offset));

    (
        value.map(|it| it.map(|it| it.0)),
        value.map(|it| it.map(|it| it.1)),
    )
}
