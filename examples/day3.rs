const INPUT: &str = include_str!("../inputs/day3.txt");

// part 1: 187833789
// part 2: 94455185
fn main() {
    let part1 = parse(INPUT).sum::<i64>();

    println!("{}", part1);

    let part2 = format!("do(){INPUT}don't()")
        .split_inclusive("do()")
        .flat_map(|s| {
            s.find("don't()")
                .or_else(|| s.find("do()"))
                .map(|i| &s[..i])
        })
        .flat_map(parse)
        .sum::<i64>();

    println!("{}", part2);
}

fn parse(src: &str) -> impl Iterator<Item = i64> + '_ {
    src.split("mul(")
        .flat_map(|s| s.find(")").map(|i| &s[..i]))
        .flat_map(|s| s.split_once(','))
        .flat_map(|(x, y)| Some((x.parse::<i64>().ok()?, y.parse::<i64>().ok()?)))
        .map(|(x, y)| x * y)
}
