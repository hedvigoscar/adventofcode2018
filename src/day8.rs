use fxhash::FxHashMap as HashMap;

#[aoc_generator(day8)]
pub fn parse_day8(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|c| c.parse::<u8>().expect("Could not parse item"))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[u8]) -> u32 {
    evaluate_node_metadata_sum(&mut input.iter())
}

fn evaluate_node_metadata_sum<'a, I: Iterator<Item = &'a u8>>(i: &mut I) -> u32 {
    let n_children = i.next().expect("Missing header field children");
    let n_metadata = i.next().expect("Missing header field metadata");

    let mut accumulator = 0;

    for _ in 0..*n_children {
        accumulator += evaluate_node_metadata_sum(&mut *i);
    }

    for _ in 0..*n_metadata {
        accumulator += u32::from(*i.next().expect("Invalid metadata node"));
    }

    accumulator
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &[u8]) -> u32 {
    evaluate_node_metadata_indexed_sum(&mut input.iter())
}

fn evaluate_node_metadata_indexed_sum<'a, I: Iterator<Item = &'a u8>>(i: &mut I) -> u32 {
    let n_children = i.next().expect("Missing header field children");
    let n_metadata = i.next().expect("Missing header field metadata");

    if *n_children == 0 {
        let mut accumulator = 0;

        for _ in 0..*n_metadata {
            accumulator += u32::from(*i.next().expect("Invalid metadata node"));
        }

        return accumulator;
    }

    let mut child_nodes = HashMap::<u8, u32>::default();

    for n in 1..=*n_children {
        child_nodes.insert(n, evaluate_node_metadata_indexed_sum(&mut *i));
    }

    let mut accumulator = 0;
    for _ in 1..=*n_metadata {
        if let Some(n) = child_nodes.get(i.next().expect("Invalid metadata node")) {
            accumulator += n;
        }
    }

    accumulator
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_input() -> Vec<u8> {
        vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
    }

    #[test]
    fn day8_part1_solves_example() {
        let input = make_input();
        let result = solve_day8_part1(&input);
        let expected = 138;

        assert_eq!(result, expected);
    }

    #[test]
    fn day8_part2_solves_example() {
        let input = make_input();
        let result = solve_day8_part2(&input);
        let expected = 66;

        assert_eq!(result, expected);
    }
}
