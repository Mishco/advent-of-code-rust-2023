advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .filter_map(|lll| {
            let first = lll.chars().find(char::is_ascii_digit);
            let second = lll.chars().rfind(char::is_ascii_digit);
            Some(first?.to_digit(10)?*10 + second?.to_digit(10)?)
        }).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
