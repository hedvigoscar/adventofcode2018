use fxhash::FxHashMap as HashMap;

fn react_polymer(polymer: &str) -> String {
    let mut buffer = polymer.to_string();
    loop {
        let mut marked_for_deletion = Vec::<usize>::default();
        let mut chars = buffer.chars().collect::<Vec<_>>();
        for i in (1..chars.len()).rev() {
            let a = chars[i - 1];
            let b = chars[i];
            if marked_for_deletion.contains(&i) || marked_for_deletion.contains(&(i - 1)) {
                continue;
            }

            if (a.to_lowercase().to_string() == b.to_string()
                || a.to_uppercase().to_string() == b.to_string())
                && a != b
            {
                marked_for_deletion.push(i);
                marked_for_deletion.push(i - 1);
            }
        }

        if marked_for_deletion.is_empty() {
            break;
        }

        for i in marked_for_deletion {
            chars.remove(i);
        }
        buffer = chars.iter().collect();
    }

    buffer
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &str) -> usize {
    react_polymer(input).len()
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &str) -> usize {
    let mut unit_types = input.to_lowercase().chars().collect::<Vec<_>>();
    unit_types.sort_unstable_by(|a, b| a.cmp(b));
    unit_types.dedup();

    let mut units_and_counts = HashMap::<char, usize>::default();

    for u in unit_types {
        let filtered_input = input
            .chars()
            .filter(|c| c.to_lowercase().to_string() != u.to_string())
            .collect::<String>();
        units_and_counts.insert(u, react_polymer(&filtered_input).len());
    }

    let mut units_and_counts = units_and_counts.iter().collect::<Vec<_>>();
    units_and_counts.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    let (_, shortest_polymer_length) = units_and_counts[0];

    *shortest_polymer_length
}
