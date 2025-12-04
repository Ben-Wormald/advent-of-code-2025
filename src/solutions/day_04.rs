use std::cmp::{max, min};

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        let x_min = max(0, x as isize - 1) as usize;
        let x_max = min(self.grid.len() - 1, x + 1);

        let y_min = max(0, y as isize - 1) as usize;
        let y_max = min(self.grid.first().unwrap().len() - 1, y + 1);

        for nx in x_min..=x_max {
            for ny in y_min..=y_max {
                if !(nx == x && ny == y) {
                    let is_roll = self.grid
                        .get(ny).unwrap()
                        .get(nx).unwrap();
                    
                    if *is_roll {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let grid = get_grid(input);
    
    let mut count = 0;

    for (y, row) in grid.grid.iter().enumerate() {
        for (x, is_roll) in row.iter().enumerate() {
            if *is_roll {
                let neighbours = grid.count_neighbours(x, y);

                if neighbours < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn solve_part_two(input: &str) -> usize {
    let mut grid = get_grid(input);
    let mut removed = 0;

    loop {
        let mut any_changed = false;

        for y in 0..grid.grid.len() {
            for x in 0..grid.grid.first().unwrap().len() {
                let neighbours = grid.count_neighbours(x, y);

                if neighbours < 4 && grid.grid[y][x] {
                    grid.grid[y][x] = false;
                    any_changed = true;
                    removed += 1;
                }
            }
        }

        if !any_changed {
            break;
        }
    }

    removed
}

fn get_grid(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|row| {
            row.chars().map(|cell| cell == '@').collect()
        })
        .collect();

    Grid { grid }    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\n\
    ";

    #[test]
    fn part_one() {
        let expected = 13;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 43;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
