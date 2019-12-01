use advent_2019::*;
use itertools::iterate;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_one.txt");

    let parsed: Result<Vec<i32>, _> = input.lines().map(|line| line.parse()).collect();
    let parsed = parsed?;

    let part_one = solve_part_one(&parsed);
    println!("part one: {}", part_one);

    let part_two = solve_part_two(&parsed);
    println!("part two: {}", part_two);

    Ok(())
}

fn solve_part_one(input: &[i32]) -> i32 {
    input.iter().map(fuel_calc).sum()
}

fn solve_part_two(input: &[i32]) -> i32 {
    input.iter().map(total_fuel).sum()
}

fn fuel_calc(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel(mass: &i32) -> i32 {
    let mass = fuel_calc(mass);
    iterate(mass, fuel_calc).take_while(|&m| m > 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_fuel() {
        let mass = 100756;
        let result = total_fuel(&mass);

        assert_eq!(result, 50346);
    }
}
