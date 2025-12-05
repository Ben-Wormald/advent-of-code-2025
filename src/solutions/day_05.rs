use std::ops::RangeInclusive;

pub fn solve_part_one(input: &str) -> usize {
    let (ranges, ids) = get_ids(input);

    ids.filter(|id| ranges.iter().any(|range| range.contains(id))).count()
}

pub fn solve_part_two(input: &str) -> usize {
    let (ranges, _ids) = get_ids(input);
    let mut sum = 0;

    for range in ranges.iter() {
        // range.co
    }

    sum
}

fn get_ids(input: &str) -> (Vec<RangeInclusive<usize>>, impl Iterator<Item = usize>) {
    let (ranges, ids) = input.split_once("\n\n").expect("failed to split_once input");

    let ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = ids
        .lines()
        .map(|id| id.parse().unwrap());

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\n\
    ";

    #[test]
    fn part_one() {
        let expected = 3;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 14;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
