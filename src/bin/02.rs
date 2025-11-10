advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sq_feet = 0;
    for line in input.lines() {
        if let [l, w, h] = line
            .split('x')
            .map(|c| {
                let num: u64 = c.parse().expect("Expect a number");
                num
            })
            .collect::<Vec<_>>()[..]
        {
            let areas = [l * w, w * h, h * l];
            sq_feet += 2 * areas.iter().sum::<u64>();
            sq_feet += areas.iter().min().unwrap(); // areas isn't empty
        }
    }
    Some(sq_feet)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut feet_of_ribbon = 0;
    for line in input.lines() {
        if let [l, w, h] = line
            .split('x')
            .map(|c| {
                let num: u64 = c.parse().expect("Expect a number");
                num
            })
            .collect::<Vec<_>>()[..]
        {
            let mut perimeters = [l, w, h];
            perimeters.sort();
            feet_of_ribbon += 2 * (perimeters[0] + perimeters[1]) + l * w * h;
        }
    }
    Some(feet_of_ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34 + 14));
    }
}
