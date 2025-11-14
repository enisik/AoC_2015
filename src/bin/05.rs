advent_of_code::solution!(5);

use itertools::Itertools;
use std::collections::HashMap;

fn is_nice_string(input: &str) -> bool {
    let mut vowels = 0;
    let mut doubles = false;
    let mut forbidden_strs = false;
    let mut iterator = input.chars().peekable();
    while let Some(curr) = iterator.next() {
        if matches!(curr, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowels += 1;
        }
        if let Some(next_char) = iterator.peek() {
            if curr == *next_char {
                doubles = true;
            }
            if matches!(
                (curr, next_char),
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
            ) {
                forbidden_strs = true;
            }
        };
    }

    vowels > 2 && doubles && !forbidden_strs
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nice_strings = 0;
    for line in input.lines() {
        if is_nice_string(line) {
            nice_strings += 1;
        }
    }
    Some(nice_strings)
}

fn is_nice_string_2(input: &str) -> bool {
    let mut two_letter_twice = false;
    let mut repeat_with_gap = false;
    let mut two_letters: HashMap<(char, char), u32> = HashMap::new();
    let mut chars = input.chars().multipeek();
    while let Some(curr) = chars.next() {
        let Some(&next) = chars.peek() else {
            break;
        };
        let opt_next_2 = chars.peek();
        if let Some(&next_2) = opt_next_2 {
            if curr == next_2 {
                repeat_with_gap = true;
            }
        }
        if let Some(count) = two_letters.get_mut(&(curr, next)) {
            *count += 1;
        } else {
            two_letters.insert((curr, next), 1);
            let Some(&next_2) = opt_next_2 else {
                break;
            };
            if curr == next_2 && curr == next {
                if let Some(count) = two_letters.get_mut(&(curr, next_2)) {
                    *count -= 1;
                }
            }
        }
    }
    for count in two_letters.values() {
        if *count > 1 {
            two_letter_twice = true;
            break;
        }
    }
    two_letter_twice && repeat_with_gap
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nice_strings = 0;
    for line in input.lines() {
        if is_nice_string_2(line) {
            nice_strings += 1;
        }
    }
    Some(nice_strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
