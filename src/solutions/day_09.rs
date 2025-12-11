use itertools::Itertools;

trait Tile {
    fn area(&self, other: &(isize, isize)) -> usize;
}

impl Tile for (isize, isize) {
    fn area(&self, other: &(isize, isize)) -> usize {
        let x = (self.0 - other.0).abs() + 1;
        let y = (self.1 - other.1).abs() + 1;

        (x * y).try_into().unwrap()
    }
}

pub fn solve_part_one(input: &str) -> usize {
    get_tiles(input)
        .combinations(2)
        .map(|pair| {
            let a = pair.first().unwrap();
            let b = pair.last().unwrap();
            a.area(b)
        })
        .max()
        .unwrap()
}

pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

fn get_tiles(input: &str) -> impl Iterator<Item = (isize, isize)> {
    input
        .lines()
        .map(|tile| tile
            .split(',').
            map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap()
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3\n\
    ";

    #[test]
    fn part_one() {
        let expected = 50;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 24;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
