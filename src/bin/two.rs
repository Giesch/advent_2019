use advent_2019::*;
use std::num::ParseIntError;
use std::ops::Add;
use std::ops::Mul;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_two.txt");
    let input = parse_input(input)?;

    let part_one = solve_part_one(&input);
    println!("part one: {}", part_one);

    let part_two = solve_part_two(&input, 19690720);
    println!("part two: {}", part_two);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.trim().split(",").map(str::parse).collect()
}

fn solve_part_one(program: &[usize]) -> usize {
    run_program(program, 12, 2)
}

fn solve_part_two(program: &[usize], target: usize) -> usize {
    let mut left = 0;
    let mut right = 0;
    let mut result = 0;

    while result != target {
        result = run_program(program, left, right);

        if result < target {
            left += 1;
        }

        if result > target {
            left -= 1;
            right += 1;
        }
    }

    100 * left + right
}

fn run_program(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;

    let mut offset = 0;
    let mut done = false;
    while !done {
        done = run_line(&mut program, offset);
        offset += 4;
    }

    program[0]
}

fn run_line(program: &mut [usize], offset: usize) -> bool {
    match program[offset] {
        1 => execute(program, offset, Add::add),
        2 => execute(program, offset, Mul::mul),
        99 => true,
        _ => panic!("invalid opcode"),
    }
}

fn execute<F>(program: &mut [usize], offset: usize, instruction: F) -> bool
where
    F: Fn(usize, usize) -> usize,
{
    let left = program[offset + 1];
    let right = program[offset + 2];
    let dest = program[offset + 3];
    program[dest] = instruction(program[left], program[right]);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run_program(&mut program, 9, 10);
        println!("{:#?}", program);
        assert_eq!(result, 3500);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../../data/day_two.txt");
        let input = parse_input(input).expect("parse input");
        let result = solve_part_two(&input, 19690720);
        assert_eq!(result, 7195);
    }
}
