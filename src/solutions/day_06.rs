#[derive(Debug)]
enum Item {
    Number(usize),
    Op(Op),
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

struct Problems<I>(Vec<I>);

impl<I> Iterator for Problems<I> where I: Iterator {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

pub fn solve_part_one(input: &str) -> usize {
    get_problems(input)
        .map(|mut problem| {
            let (op, init) = if let Item::Op(op) = problem.pop().unwrap() {
                let init = match op {
                    Op::Add => 0,
                    Op::Multiply => 1,
                };
                (op, init)
            } else {
                panic!();
            };

            problem
                .into_iter()
                .fold(init, |acc, item| {
                    if let Item::Number(number) = item {
                        match op {
                            Op::Add => acc + number,
                            Op::Multiply => acc * number,
                        }
                    } else {
                        panic!();
                    }
                })
        })
        .sum()
}

pub fn solve_part_two(input: &str) -> usize {
    let mut sum = 0;

    let mut problem_result = 0;
    let mut op = Op::Add;

    for column in 0..=input.lines().next().unwrap().len() {
        if input.lines().all(|line| line.chars().nth(column).unwrap_or(' ') == ' ') {
            sum += problem_result;
            problem_result = 0;
            continue;
        }

        op = match input.lines().last().unwrap().chars().nth(column).unwrap_or(' ') {
            '+' => {
                problem_result = 0;
                Op::Add
            },
            '*' => {
                problem_result = 1;
                Op::Multiply
            },
            _ => op,
        };

        let number = input.lines()
            .map(|line| line.chars().nth(column).unwrap_or(' '))
            .filter(|char| char.is_numeric())
            .collect::<String>()
            .parse::<usize>().unwrap();

        match op {
            Op::Add => problem_result += number,
            Op::Multiply => problem_result *= number,
        };
    }

    if problem_result != 0 {
        sum += problem_result;
    }

    sum
}

fn get_problems(input: &str) -> Problems<impl Iterator<Item = Item>> {
    Problems(input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|item| {
                if item == "+" {
                    Item::Op(Op::Add)
                } else if item == "*" {
                    Item::Op(Op::Multiply)
                } else {
                    Item::Number(item.parse().unwrap())
                }
            }))
            .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn part_one() {
        let expected = 4277556;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 3263827;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
