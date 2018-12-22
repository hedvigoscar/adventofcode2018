use fxhash::FxHashMap as HashMap;

#[aoc_generator(day6)]
pub fn parse_day6(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|s| {
            let s = s.split(',').collect::<Vec<_>>();
            let (x, y) = (s[0], s[1].trim());
            (
                x.parse::<u32>().expect("Failed to parse x"),
                y.parse::<u32>().expect("Failed to parse y"),
            )
        })
        .collect()
}

fn manhattan_distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    let ax = a.0 as i32;
    let ay = a.1 as i32;
    let bx = b.0 as i32;
    let by = b.1 as i32;
    ((ax - bx).abs() + (ay - by).abs()) as u32
}

fn check_closeness(input: &[(u32, u32)], target: (u32, u32)) -> Vec<((u32, u32), u32)> {
    let (x, y) = target;
    let mut distances = Vec::<((u32, u32), u32)>::default();

    // eprintln!("Manhattan distances to ({}, {}):", x, y);
    for i in input {
        let current_manhattan = manhattan_distance(*i, (x, y));
        distances.push((*i, current_manhattan));
        // eprintln!("({}, {}): {}", i.0, i.1, current_manhattan);
    }
    distances.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
    distances
}

fn is_closest(input: &[(u32, u32)], target: (u32, u32), subject: (u32, u32)) -> bool {
    let distances = check_closeness(input, target);
    let (wx, wy) = distances[0].0;
    wx == subject.0 && wy == subject.1
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &[(u32, u32)]) -> u32 {
    let mut disqualified = Vec::<(u32, u32)>::default();
    let mut finder = input.to_owned();
    finder.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    let max_x = finder[finder.len() - 1].0;
    disqualified.push(finder[0]);
    disqualified.push(finder[finder.len() - 1]);
    finder.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    let max_y = finder[finder.len() - 1].1;
    disqualified.push(finder[0]);
    disqualified.push(finder[finder.len() - 1]);

    let mut records = HashMap::<(u32, u32), u32>::default();

    for x in 0..=max_x {
        for y in 0..=max_y {
            let distances = check_closeness(input, (x, y));

            if distances[0].1 == distances[1].1 {
                continue;
            }

            let count = records.get(&distances[0].0).unwrap_or(&0);
            records.insert(distances[0].0, count + 1);
        }
    }

    let mut records = records
        .iter()
        .filter(|(r, _)| !disqualified.contains(r))
        .filter(|(r, _)| {
            if is_closest(input, (r.0, 0), **r) {
                return false;
            }
            if is_closest(input, (r.0, max_y), **r) {
                return false;
            }
            if is_closest(input, (0, r.1), **r) {
                return false;
            }
            if is_closest(input, (max_x, r.1), **r) {
                return false;
            }

            true
        })
        .collect::<Vec<_>>();
    records.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

    *records[0].1
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &[(u32, u32)]) -> u32 {
    let mut finder = input.to_owned();
    finder.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    let max_x = finder[finder.len() - 1].0;
    finder.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
    let max_y = finder[finder.len() - 1].1;

    let mut region_size = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut total = 0;
            for i in input {
                total += manhattan_distance(*i, (x, y));
                if total > 10000 {
                    break;
                }
            }

            if total < 10000 {
                region_size += 1;
            }
        }
    }

    region_size
}
