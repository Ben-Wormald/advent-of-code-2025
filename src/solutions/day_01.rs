enum Turn {
    Left(isize),
    Right(isize),
}

impl Turn {
    fn new(direction: char, distance: String) -> Turn {
        let distance = distance.parse().unwrap();
        match direction {
            'L' => Turn::Left(distance),
            'R' => Turn::Right(distance),
            _ => panic!(),
        }
    }

    fn delta(&self) -> isize {
        match self {
            Turn::Left(distance) => -distance,
            Turn::Right(distance) => *distance,
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let turns = get_turns(input);
    let mut zero_count = 0;
    let mut position = 50;

    for turn in turns.into_iter() {
        position = (position + turn.delta()) % 100;

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn solve_part_two(input: &str) -> usize {
    let turns = get_turns(input);
    let mut zero_count = 0;
    let mut position = 50;

    for turn in turns.into_iter() {
        let delta = turn.delta();

        let full_turns = (delta / 100).abs();
        zero_count += full_turns;

        let new_position = position + delta % 100;

        if (position != 0 && new_position < 0) || new_position > 100 {
            zero_count += 1;
        }

        position = new_position.rem_euclid(100);

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count as usize
}

fn get_turns(input: &str) -> Vec<Turn> {
    input.lines().map(|turn| {
        let mut chars = turn.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect();

        Turn::new(direction, distance)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\n\
    ";

    #[test]
    fn part_one() {
        let expected = 3;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 6;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
