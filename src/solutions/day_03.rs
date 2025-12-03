pub fn solve_part_one(input: &str) -> usize {
    get_banks(input)
        .map(|bank| get_largest(&bank, 2))
        .sum()
}

pub fn solve_part_two(input: &str) -> usize {
    get_banks(input)
        .map(|bank| get_largest(&bank, 12))
        .sum()
}

fn get_banks(input: &str) -> impl Iterator<Item = Vec<u8>> {
    input
        .lines()
        .map(|bank| bank
            .chars()
            .map(|battery| battery.to_digit(10).unwrap() as u8)
            .collect()
        )
}

fn get_largest(bank: &[u8], digits: usize) -> usize {
    let mut range_start = 0;
    let mut sum = 0;

    for digit in (0..digits).rev() {
        let range_end = bank.len() - digit;

        let (digit_n_index, digit_n) = bank[..range_end]
            .iter()
            .enumerate()
            .skip(range_start)
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();

        range_start = digit_n_index + 1;
        sum += *digit_n as u64 * 10_u64.pow(digit as u32);
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\n\
    ";

    #[test]
    fn part_one() {
        let expected = 357;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 3121910778619;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
