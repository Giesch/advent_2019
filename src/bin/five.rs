use advent_2019::*;
use std::num::ParseIntError;
use std::ops::Add;
use std::ops::Mul;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_five.txt").trim();
    let input = parse_input(input)?;

    let part_one = solve_part_one(&input);
    println!("part one: {:#?}", part_one);

    let part_two = solve_part_two(&input);
    println!("part one: {:#?}", part_two);

    Ok(())
}

fn solve_part_two(program: &[i32]) -> i32 {
    let mut program = program.to_vec();
    let outputs = run_program(&mut program, 5);
    *outputs.last().unwrap()
}

fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.trim().split(",").map(str::parse).collect()
}

fn solve_part_one(program: &[i32]) -> i32 {
    let mut program = program.to_vec();
    let outputs = run_program(&mut program, 1);
    // println!("outputs: {:?}", outputs);
    *outputs.last().unwrap()
}

fn run_program(mut program: &mut [i32], input: i32) -> Vec<i32> {
    let mut offset = 0;
    let mut done = false;

    let mut inputs = vec![input];
    let mut outputs = Vec::new();

    while !done {
        let (done_now, new_offset) = run_line(&mut program, offset, &mut inputs, &mut outputs);
        done = done_now;
        offset = new_offset;
    }

    outputs
}

fn run_line(
    program: &mut [i32],
    offset: i32,
    inputs: &mut Vec<i32>,
    outputs: &mut Vec<i32>,
) -> (bool, i32) {
    let instr = program[offset as usize];
    let op = instr % 100;
    let first_mode = (instr / 100) % 2;
    let second_mode = (instr / 1000) % 2;

    match op {
        99 => (true, 0),
        1 => execute(program, offset, first_mode, second_mode, Add::add),

        2 => execute(program, offset, first_mode, second_mode, Mul::mul),

        3 => {
            let i = inputs.pop().unwrap();
            let dest = program[offset as usize + 1];
            program[dest as usize] = i;
            (false, offset + 2)
        }

        4 => {
            let v = get_value(program, offset + 1, first_mode);
            outputs.push(v);
            (false, offset + 2)
        }

        5 => {
            let jump = get_value(program, offset + 1, first_mode);
            if jump == 0 {
                (false, offset + 3)
            } else {
                let target = get_value(program, offset + 2, second_mode);
                (false, target)
            }
        }

        6 => {
            let jump = get_value(program, offset + 1, first_mode);
            if jump != 0 {
                (false, offset + 3)
            } else {
                let target = get_value(program, offset + 2, second_mode);
                (false, target)
            }
        }

        7 => execute(program, offset, first_mode, second_mode, |first, second| {
            if first < second {
                1
            } else {
                0
            }
        }),

        8 => execute(program, offset, first_mode, second_mode, |first, second| {
            if first == second {
                1
            } else {
                0
            }
        }),

        _ => panic!("invalid opcode"),
    }
}

fn execute<F>(
    program: &mut [i32],
    offset: i32,
    left_mode: i32,
    right_mode: i32,
    op: F,
) -> (bool, i32)
where
    F: Fn(i32, i32) -> i32,
{
    let left = get_value(program, offset + 1, left_mode);
    let right = get_value(program, offset + 2, right_mode);
    let dest = program[offset as usize + 3];
    program[dest as usize] = op(left, right);
    (false, offset + 4)
}

fn get_value(program: &[i32], arg_address: i32, mode: i32) -> i32 {
    if mode == 0 {
        program[program[arg_address as usize] as usize]
    } else {
        program[arg_address as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut program = vec![1002, 4, 3, 4, 33];
        let _result = run_program(&mut program, 1);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn negative_allowed() {
        let mut program = vec![1101, 100, -1, 4, 0];
        let _result = run_program(&mut program, 1);
        assert_eq!(program, vec![1101, 100, -1, 4, 99]);
    }
}
