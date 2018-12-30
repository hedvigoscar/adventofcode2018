#![allow(clippy::trivially_copy_pass_by_ref)]

use fxhash::FxHashMap as HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Coordinate(i32, i32);

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)?;
        Ok(())
    }
}

#[aoc_generator(day11)]
pub fn parse_day11(input: &str) -> i32 {
    input.parse().expect("Failed to parse input")
}

fn find_cell_value(c: &Coordinate, serial_number: i32) -> i32 {
    let rack_id: i32 = c.0 + 10;
    let mut power_level = rack_id * c.1;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(input: &i32) -> Coordinate {
    let mut cell_values = HashMap::<Coordinate, i32>::default();

    for x in 1..=300 {
        for y in 1..=300 {
            let cell = Coordinate(x, y);
            let cell_value = find_cell_value(&cell, *input);
            cell_values.insert(cell, cell_value);
        }
    }

    let mut threes_values = HashMap::<Coordinate, i32>::default();

    for x in 1..=298 {
        for y in 1..=298 {
            let mut sum = 0;
            for tx in x..=x + 2 {
                for ty in y..=y + 2 {
                    let val = cell_values
                        .get(&Coordinate(tx, ty))
                        .expect("Uncalculated coordinate");
                    sum += val;
                }
            }
            let coordinate = Coordinate(x, y);
            threes_values.insert(coordinate, sum);
        }
    }

    let mut threes_values = threes_values.iter().collect::<Vec<_>>();
    threes_values.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

    threes_values[0].0.clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_solves_example_1() {
        let expected = Coordinate(33, 45);
        let result = solve_day11_part1(&18);

        assert_eq!(expected, result);
    }

    #[test]
    fn part1_solves_example_2() {
        let expected = Coordinate(21, 61);
        let result = solve_day11_part1(&42);

        assert_eq!(expected, result);
    }
}
