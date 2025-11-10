use std::collections::HashSet;

advent_of_code::solution!(3);

fn mov(c: &char, (x, y): (i64, i64)) -> Option<(i64, i64)> {
    match c {
        '<' => Some((x - 1, y)),
        '>' => Some((x + 1, y)),
        'v' => Some((x, y - 1)),
        '^' => Some((x, y + 1)),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut houses = HashSet::new();
    let mut current_pos = (0, 0);
    houses.insert(current_pos);
    for c in input.chars() {
        current_pos = mov(&c, current_pos)?;
        houses.insert(current_pos);
    }
    Some(houses.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut houses = HashSet::new();
    let mut current_pos_santa = (0, 0);
    let mut current_pos_robo = (0, 0);
    houses.insert(current_pos_santa);
    for c in input.chars().step_by(2) {
        current_pos_santa = mov(&c, current_pos_santa)?;
        houses.insert(current_pos_santa);
    }
    for c in input.chars().skip(1).step_by(2) {
        current_pos_robo = mov(&c, current_pos_robo)?;
        houses.insert(current_pos_robo);
    }
    Some(houses.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
