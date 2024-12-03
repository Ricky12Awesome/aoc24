const INPUT: &str = include_str!("../inputs/day3.txt");

// part 1: 187833789
// part 2: 94455185
fn main() {
    let input = format!("do(){INPUT}don't()");
    let options = aoc24::options(5);

    let part1 = parse(&input);
    println!("{}", part1);

    let part2 = parse2(&input);
    println!("{}", part2);
    
    aoc24::bench(&options, "Part 1", || parse(&input));
    aoc24::bench(&options, "Part 2", || parse2(&input));
}

fn parse2(src: &str) -> i64 {
    src.split_inclusive("do()")
        .flat_map(|s| {
            s.find("don't()")
                .or_else(|| s.find("do()"))
                .map(|i| &s[..i])
        })
        .map(parse)
        .sum::<i64>()
}

fn parse(src: &str) -> i64 {
    src.split("mul(")
        .flat_map(|s| s.find(")").map(|i| &s[..i]))
        .flat_map(|s| s.split_once(','))
        .flat_map(|(x, y)| Some((x.parse::<i64>().ok()?, y.parse::<i64>().ok()?)))
        .map(|(x, y)| x * y)
        .sum::<i64>()
}
