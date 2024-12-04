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
        // -
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(3, 0), (2, 0), (1, 0), (0, 0)],
        // |
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 3), (0, 2), (0, 1), (0, 0)],
        // X
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

    let mut values = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            for offset in offsets {
                let value = get(grid, x, y, offset);
                let result = value.map(|it| it.map(|it| it.1));

                if result == search {
                    values.insert(value);
                }
            }
        }
    }

    values.len()
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
) -> [Option<((usize, usize), char)>; N] {
    offsets.map(|offset| get_char(grid, x, y, offset))
}
