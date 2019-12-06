use advent_2019::*;
use std::collections::HashMap;

fn main() -> StdResult<()> {
    let input = include_str!("../../data/day_six.txt").trim();
    let input = parse_input(input)?;

    let part_one = solve_part_one(&input);
    println!("part one: {:#?}", part_one);

    let part_two = solve_part_two(&input);
    println!("part two: {:#?}", part_two);

    Ok(())
}

fn parse_input(input: &str) -> StdResult<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();
    for (k, v) in input.lines().map(parse_line) {
        result.insert(k.to_string(), v.to_string());
    }

    Ok(result)
}

fn parse_line(line: &str) -> (&str, &str) {
    let split: Vec<_> = line.split(")").collect();
    (split[1], split[0])
}

fn solve_part_one(orbits: &HashMap<String, String>) -> usize {
    // TODO this is dumb
    orbits.keys().map(|k| orbit_chain(orbits, k).len()).sum()
}

fn orbit_chain<'a>(orbits: &'a HashMap<String, String>, k: &'a str) -> Vec<&'a str> {
    let mut chain = Vec::new();
    let mut k = k;
    while let Some(next) = orbits.get(k) {
        chain.push(k);
        k = next;
    }

    chain
}

fn solve_part_two(orbits: &HashMap<String, String>) -> usize {
    let mut our_chain = orbit_chain(orbits, "YOU");
    let mut san_chain = orbit_chain(orbits, "SAN");
    while our_chain.last() == san_chain.last() {
        our_chain.pop();
        san_chain.pop();
    }

    our_chain.len() + san_chain.len() - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let orbits = parse_input(example).expect("parse test input");
        let result = solve_part_one(&orbits);

        assert_eq!(result, 42);
    }

    #[test]
    fn test_part_two() {
        let example = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let orbits = parse_input(example).expect("parse test input");
        let result = solve_part_two(&orbits);

        assert_eq!(result, 4);
    }
}
