advent_of_code::solution!(4);

use crypto::{digest::Digest, md5::Md5};

pub fn part_one(input: &str) -> Option<u64> {
    let mut hasher = Md5::new();
    let mut output = [0; 16];

    for i in 0..u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut output);
        let five_zeros = output[0] as u32 + output[1] as u32 + (output[2] >> 4) as u32;
        if five_zeros == 0 {
            println!("HexHash: {}", hasher.result_str());
            return Some(i);
        }
        hasher.reset();
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut hasher = Md5::new();
    let mut output = [0; 16];

    for i in 0..u64::MAX {
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut output);
        let five_zeros = output[0] as u32 + output[1] as u32 + (output[2]) as u32;
        if five_zeros == 0 {
            println!("HexHash: {}", hasher.result_str());
            return Some(i);
        }
        hasher.reset();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
