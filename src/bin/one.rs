use advent_2019::*;
use itertools::Itertools;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_one.txt");

    println!("Solving day one...");
    let solution = solve(input)?;
    println!("{}", solution);

    Ok(())
}

fn solve(input: &str) -> StdResult<String> {
    let solution = format!("got input: {}", input);
    let solution = solution
        .chars()
        .interleave(vec!['a', 'b', 'c', 'd'])
        .collect();

    Ok(solution)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
