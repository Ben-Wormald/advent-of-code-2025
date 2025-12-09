use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord(isize, isize, isize);

impl Coord {
    fn new(coord: (isize, isize, isize)) -> Self {
        Coord(coord.0, coord.1, coord.2)
    }

    fn distance_from(&self, other: &Coord) -> f64 {
        (
            (self.0 - other.0).pow(2) as f64
            + (self.1 - other.1).pow(2) as f64
            + (self.2 - other.2).pow(2) as f64
        ).sqrt()
    }
}

pub fn solve_part_one(input: &str) -> usize {
    solve_part_one_n(input, 1000)
}

fn solve_part_one_n(input: &str, edge_count: usize) -> usize {
    let edges = get_edges(get_coords(input));

    let mut circuits: Vec<HashSet<Coord>> = Vec::new();

    for edge in edges.take(edge_count) {
        let has_circuit_with_both = circuits.iter_mut()
            .any(|circuit| circuit.contains(&edge.0) && circuit.contains(&edge.1));

        if has_circuit_with_both {
            continue;
        }

        let existing_circuit_count = circuits
            .iter()
            .filter(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1))
            .count();

        if existing_circuit_count == 2 {
            let mut existing_circuits = circuits
                .iter_mut()
                .filter(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1));

            let circuit_a = existing_circuits.next().unwrap();
            let circuit_b = existing_circuits.next().unwrap();

            circuit_a.drain().for_each(|coord| {
                circuit_b.insert(coord);
            });
        } else if existing_circuit_count == 1 {
            let existing_circuit = circuits
                .iter_mut()
                .find(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1))
                .unwrap();

            existing_circuit.insert(edge.0);
            existing_circuit.insert(edge.1);
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(edge.0);
            circuit.insert(edge.1);
            circuits.push(circuit);
        }
    }

    circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .filter(|len| *len > 0)
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn solve_part_two(input: &str) -> usize {
    let coords = get_coords(input);
    let edges = get_edges(coords);

    let mut circuits: Vec<HashSet<Coord>> = Vec::new();
    let mut last_x_product = 0;

    for edge in edges {
        let has_circuit_with_both = circuits.iter_mut()
            .any(|circuit| circuit.contains(&edge.0) && circuit.contains(&edge.1));

        if has_circuit_with_both {
            continue;
        }

        let existing_circuit_count = circuits
            .iter()
            .filter(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1))
            .count();

        if existing_circuit_count == 2 {
            let mut existing_circuits = circuits
                .iter_mut()
                .filter(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1));

            let circuit_a = existing_circuits.next().unwrap();
            let circuit_b = existing_circuits.next().unwrap();

            circuit_a.drain().for_each(|coord| {
                circuit_b.insert(coord);
            });

            last_x_product = edge.0.0 * edge.1.0;
        } else if existing_circuit_count == 1 {
            let existing_circuit = circuits
                .iter_mut()
                .find(|circuit| circuit.contains(&edge.0) || circuit.contains(&edge.1))
                .unwrap();

            last_x_product = edge.0.0 * edge.1.0;

            existing_circuit.insert(edge.0);
            existing_circuit.insert(edge.1);

        } else {
            last_x_product = edge.0.0 * edge.1.0;

            let mut circuit = HashSet::new();
            circuit.insert(edge.0);
            circuit.insert(edge.1);
            circuits.push(circuit);
        }
    }

    last_x_product as usize
}

fn get_edges(coords: impl Iterator<Item = Coord>) -> impl Iterator<Item = (Coord, Coord)> {
    coords
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let a = pair.first().unwrap().clone();
            let b = pair.last().unwrap().clone();

            let distance = a.distance_from(&b);
            
            (a, b, distance)
        })
        .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .map(|pair| (pair.0, pair.1))
}

fn get_coords(input: &str) -> impl Iterator<Item = Coord> {
    input
        .lines()
        .map(|coord| coord
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .map(Coord::new)
            .unwrap()
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        162,817,812\n\
        57,618,57\n\
        906,360,560\n\
        592,479,940\n\
        352,342,300\n\
        466,668,158\n\
        542,29,236\n\
        431,825,988\n\
        739,650,466\n\
        52,470,668\n\
        216,146,977\n\
        819,987,18\n\
        117,168,530\n\
        805,96,715\n\
        346,949,466\n\
        970,615,88\n\
        941,993,340\n\
        862,61,35\n\
        984,92,344\n\
        425,690,689\n\
    ";

    #[test]
    fn part_one() {
        let expected = 40;

        assert_eq!(solve_part_one_n(INPUT, 10), expected);
    }

    #[test]
    fn part_two() {
        let expected = 25272;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
