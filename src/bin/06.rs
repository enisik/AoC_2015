advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Instruction {
    command: Command,
    start: Coordinate,
    end: Coordinate,
}

impl Instruction {
    fn new(instruction: &str) -> Instruction {
        let command: Command;

        let mut chars = instruction.split_whitespace();
        match chars.next() {
            Some("turn") => match chars.next() {
                Some("on") => command = Command::TurnOn,
                Some("off") => command = Command::TurnOff,
                _ => panic!("No 'on' or 'off'"),
            },
            Some("toggle") => {
                command = Command::Toggle;
            }
            _ => panic!("Not 'toogle' or 'turn'"),
        }
        let coord: Vec<u16> = chars
            .next()
            .unwrap()
            .splitn(2, ',')
            .map(|c| c.parse::<u16>().unwrap())
            .collect();
        let start: Coordinate = Coordinate(coord[0], coord[1]);
        if chars.next() != Some("through") {
            panic!("No 'through'")
        }
        let coord: Vec<u16> = chars
            .next()
            .unwrap()
            .splitn(2, ',')
            .map(|c| c.parse::<u16>().unwrap())
            .collect();
        let end: Coordinate = Coordinate(coord[0], coord[1]);
        Instruction {
            command,
            start,
            end,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Coordinate(u16, u16);

fn change_lights_bool(lights: &mut [[bool; 1000]; 1000], instruction: Instruction) {
    for x in instruction.start.0..instruction.end.0 + 1 {
        for y in instruction.start.1..instruction.end.1 + 1 {
            match instruction.command {
                Command::TurnOn => lights[x as usize][y as usize] = true,
                Command::TurnOff => lights[x as usize][y as usize] = false,
                Command::Toggle => lights[x as usize][y as usize] = !lights[x as usize][y as usize],
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = [[false; 1000]; 1000];
    for line in input.lines() {
        let instruction = Instruction::new(line);
        change_lights_bool(&mut lights, instruction);
    }
    Some(lights.iter().flatten().filter(|x| **x).count())
}

fn change_lights_num(lights: &mut [[u8; 1000]; 1000], instruction: Instruction) {
    for x in instruction.start.0..instruction.end.0 + 1 {
        for y in instruction.start.1..instruction.end.1 + 1 {
            match instruction.command {
                Command::TurnOn => lights[x as usize][y as usize] += 1,
                Command::TurnOff => {
                    if lights[x as usize][y as usize] > 0 {
                        lights[x as usize][y as usize] -= 1
                    }
                }
                Command::Toggle => lights[x as usize][y as usize] += 2,
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lights = [[0; 1000]; 1000];
    for line in input.lines() {
        let instruction = Instruction::new(line);
        change_lights_num(&mut lights, instruction);
    }
    Some(lights.iter().flatten().map(|&x| x as usize).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_instructions() {
        let string_1 = "turn on 0,0 through 999,999";
        let string_2 = "toggle 0,0 through 999,0";
        let string_3 = "turn off 499,499 through 500,500";

        let instruction_1 = Instruction {
            command: Command::TurnOn,
            start: Coordinate(0, 0),
            end: Coordinate(999, 999),
        };
        let instruction_2 = Instruction {
            command: Command::Toggle,
            start: Coordinate(0, 0),
            end: Coordinate(999, 0),
        };
        let instruction_3 = Instruction {
            command: Command::TurnOff,
            start: Coordinate(499, 499),
            end: Coordinate(500, 500),
        };

        assert_eq!(Instruction::new(string_1), instruction_1);
        assert_eq!(Instruction::new(string_2), instruction_2);
        assert_eq!(Instruction::new(string_3), instruction_3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(999000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1002000));
    }
}
