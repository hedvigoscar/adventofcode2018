#[aoc_generator(day8)]
pub fn parse_day8(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|c| c.parse::<u8>().expect("Could not parse item"))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[u8]) -> u32 {
    parse_node(&mut input.iter())
}

fn parse_node<'a, I: Iterator<Item = &'a u8>>(i: &mut I) -> u32 {
    let n_children = i.next().expect("Missing header field children");
    let n_metadata = i.next().expect("Missing header field metadata");

    let mut accumulator = 0;

    for _ in 0..*n_children {
        accumulator += parse_node(&mut *i);
    }

    for _ in 0..*n_metadata {
        accumulator += u32::from(*i.next().expect("Invalid metadata node"));
    }

    accumulator
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day8_part1_solves_example() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let result = solve_day8_part1(&input);
        let expected = 138;

        assert_eq!(result, expected);
    }
}
