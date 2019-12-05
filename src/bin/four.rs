use advent_2019::*;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_four.txt").trim();
    let input = parse_input(input)?;

    let part_one = solve_part_one(input);
    println!("part one: {:#?}", part_one);

    let part_two = solve_part_two(input);
    println!("part one: {:#?}", part_two);

    Ok(())
}

fn parse_input(input: &str) -> StdResult<(u32, u32)> {
    let split: Vec<&str> = input.split("-").collect();
    Ok((split[0].parse()?, split[1].parse()?))
}

fn solve_part_one((min, max): (u32, u32)) -> usize {
    let possibilities: HashSet<_> = (min..max).filter(keep).collect();
    possibilities.len()
}

fn solve_part_two((min, max): (u32, u32)) -> usize {
    let possibilities: HashSet<_> = (min..max).filter(keep_two).collect();
    possibilities.len()
}

fn keep(num: &u32) -> bool {
    let digits = as_digits(*num);
    monotonic(&digits) && has_run_of_two(&digits)
}

fn monotonic(digits: &[u32]) -> bool {
    digits.iter().tuple_windows().all(|(x, y)| x <= y)
}

fn has_run_of_two(digits: &[u32]) -> bool {
    digits
        .into_iter()
        .group_by(|x| *x)
        .into_iter()
        .map(|(_key, group)| group.count())
        .any(|x| x >= 2)
}

fn as_digits(n: u32) -> Vec<u32> {
    let mut remainder = n;
    let mut digits = Vec::new();

    while remainder > 0 {
        let digit = remainder % 10;
        digits.push(digit);
        remainder /= 10;
    }

    digits.reverse();
    digits
}

fn keep_two(num: &u32) -> bool {
    let digits = as_digits(*num);
    monotonic(&digits) && has_run_of_exactly_two(&digits)
}

fn has_run_of_exactly_two(digits: &[u32]) -> bool {
    digits
        .into_iter()
        .group_by(|x| *x)
        .into_iter()
        .map(|(_key, group)| group.count())
        .any(|x| x == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_digits() {
        let result = as_digits(123);
        let expected = vec![1, 2, 3];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_keep() {
        assert!(keep(&111111));
        assert!(!keep(&223450));
        assert!(!keep(&123789));
    }
}
