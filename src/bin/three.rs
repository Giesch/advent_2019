use advent_2019::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_three.txt").trim();
    let input = parse_input(input)?;

    let part_one = solve_part_one(&input)?;
    println!("part one: {:#?}", part_one);

    let part_two = solve_part_two(&input)?;
    println!("part two: {:#?}", part_two);

    Ok(())
}

fn parse_input(input: &str) -> StdResult<Vec<Vec<Command>>> {
    input.lines().map(parse_wire).collect()
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Command {
    dir: Direction,
    dist: i32,
}

fn parse_wire(line: &str) -> StdResult<Vec<Command>> {
    line.split(",").map(parse_command).collect()
}

fn parse_command(s: &str) -> StdResult<Command> {
    let mut chars = s.chars();
    let first_char: char = chars.next().ok_or("no first character")?;
    let dir: Direction = match first_char {
        'D' => Direction::Down,
        'U' => Direction::Up,
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => Err("invalid direction")?,
    };
    let dist: i32 = chars.as_str().parse()?;

    Ok(Command { dir, dist })
}

fn solve_part_one(wires: &[Vec<Command>]) -> StdResult<i32> {
    let first_wire_path = wire_path(&wires[0]);
    let second_wire_path = wire_path(&wires[1]);
    let intersections = first_wire_path.intersection(&second_wire_path);

    intersections
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .ok_or("no intersections".into())
}

fn wire_path(wire: &[Command]) -> HashSet<(i32, i32)> {
    let mut path = HashSet::new();
    let mut position = (0, 0);

    for command in wire {
        for _ in 0..command.dist {
            match command.dir {
                Direction::Up => position.1 += 1,
                Direction::Down => position.1 -= 1,
                Direction::Right => position.0 += 1,
                Direction::Left => position.0 -= 1,
            }

            path.insert(position);
        }
    }

    path
}

fn solve_part_two(wires: &[Vec<Command>]) -> StdResult<i32> {
    let first_wire_path = ordered_wire_path(&wires[0]);
    let second_wire_path = ordered_wire_path(&wires[1]);

    let first: HashSet<_> = first_wire_path.keys().collect();
    let second: HashSet<_> = second_wire_path.keys().collect();
    let intersections = first.intersection(&second);

    let mut step_counts = Vec::new();
    for pos in intersections {
        let left = first_wire_path.get(&pos).ok_or("i just put this here")?;
        let right = second_wire_path.get(&pos).ok_or("this one too")?;
        step_counts.push(left + right);
    }

    step_counts
        .into_iter()
        .min()
        .ok_or("no intersections".into())
}

fn ordered_wire_path(wire: &[Command]) -> HashMap<(i32, i32), i32> {
    let mut path = HashMap::new();
    let mut position = (0, 0);
    let mut step = 0;

    for command in wire {
        for _ in 0..command.dist {
            match command.dir {
                Direction::Up => position.1 += 1,
                Direction::Down => position.1 -= 1,
                Direction::Right => position.0 += 1,
                Direction::Left => position.0 -= 1,
            }
            step += 1;
            path.insert(position, step);
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let input = parse_input(input).expect("parsed");
        let result = solve_part_one(&input).expect("solve test");

        assert_eq!(result, 159);
    }
}
