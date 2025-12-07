use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
enum Cell {
    Empty,
    Splitter,
}

pub fn solve_part_one(input: &str) -> usize {
    let (start_column, grid) = get_grid(input);

    let mut split_count = 0;
    let mut beam_columns = HashSet::new();
    beam_columns.insert(start_column);

    for row in grid {
        row
            .enumerate()
            .filter(|(_column, cell)| matches!(cell, Cell::Splitter))
            .for_each(|(column, _splitter)| {
                if beam_columns.contains(&column) {
                    beam_columns.insert(column - 1);
                    beam_columns.insert(column + 1);
                    beam_columns.remove(&column);
                    split_count += 1;
                }
            });
    }

    split_count
}

pub fn solve_part_two(input: &str) -> usize {
    let (start_column, grid) = get_grid(input);

    let mut cache = HashMap::new();

    get_timelines(start_column, grid, 0, &mut cache)
}

fn get_timelines(
    beam: usize,
    mut rows: impl Iterator<Item = impl Iterator<Item = Cell>> + Clone,
    depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(timelines) = cache.get(&(beam, depth)) {
        *timelines
    } else {
        let timelines = if let Some(mut row) = rows.next() {
            if let Some(Cell::Splitter) = row.nth(beam) {
                let timelines_left = get_timelines(beam - 1, rows.clone(), depth + 1, cache);
                let timelines_right = get_timelines(beam + 1, rows, depth + 1, cache);
                timelines_left + timelines_right
            } else {
                get_timelines(beam, rows, depth + 1, cache)
            }
        } else {
            1
        };
    
        cache.insert((beam, depth), timelines);
        timelines
    }
}

fn get_grid(input: &str) -> (usize, impl Iterator<Item = impl Iterator<Item = Cell>> + Clone) {
    let start_column = input.find("S").unwrap();

    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|cell| match cell {
                    '^' => Cell::Splitter,
                    _ => Cell::Empty,
                })
        });

    (start_column, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\n\
    ";

    #[test]
    fn part_one() {
        let expected = 21;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 40;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
