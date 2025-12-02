use std::collections::HashSet;

pub fn solve_part_one(input: &str) -> usize {
    let ranges = get_ranges(input);
    let mut sum = 0;

    for range in ranges {
        let mut search_start = range.0.to_string();
        let mut search_start_digits = search_start.len();

        if search_start_digits % 2 != 0 {
            search_start = (10_u64.pow(search_start_digits as u32)).to_string();
            search_start_digits = search_start.len();
        }

        let (search, _) = search_start.split_at(search_start_digits / 2);

        let mut search = search.parse::<u64>().unwrap();

        loop {
            let id = search + search * 10_u64.pow((search).ilog10() + 1);

            if id > range.1 {
                break;
            }

            if id >= range.0 {
                sum += id;
            }

            search += 1;
        }
    }

    sum as usize
}

pub fn solve_part_two(input: &str) -> usize {
    let ranges = get_ranges(input);
    let mut ids = HashSet::new();

    for range in ranges {
        let mut repeats = 2;

        loop {
            let mut pattern = 1;

            loop {
                let id = pattern.to_string().repeat(repeats).parse::<u64>().unwrap();

                if id > range.1 {
                    break;
                }
    
                if id >= range.0 {
                    ids.insert(id);
                }
    
                pattern += 1;
            }

            repeats += 1;

            if repeats as u32 > range.1.ilog10() + 1 {
                break;
            }
        }
    }

    ids.into_iter().sum::<u64>() as usize
}

fn get_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input
        .split(',')
        .map(|range| {
            let (min, max) = range.split_once('-').unwrap();
            (min.trim().parse().unwrap(), max.trim().parse().unwrap())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

    #[test]
    fn part_one() {
        let expected = 1227775554;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 4174379265;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
