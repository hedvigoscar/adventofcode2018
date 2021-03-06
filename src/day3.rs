use fxhash::FxHashMap as HashMap;

pub struct Claim {
    id: String,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn parse_day3(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|s| {
            let parts = s.split_whitespace().collect::<Vec<&str>>();
            let edges = parts[2].split(',').collect::<Vec<&str>>();
            let sizes = parts[3].split('x').collect::<Vec<&str>>();
            Claim {
                id: parts[0].to_string(),
                left: edges[0].parse().unwrap(),
                top: edges[1].trim_matches(':').parse().unwrap(),
                width: sizes[0].parse().unwrap(),
                height: sizes[1].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Claim]) -> u32 {
    let mut square_inches = HashMap::<String, u32>::default();
    for claim in input {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let coordinates = format!("{},{}", x, y);
                let prev_count = square_inches.get(&coordinates).unwrap_or(&0);
                square_inches.insert(coordinates, prev_count + 1);
            }
        }
    }

    let mut overlaps = 0;
    for (_, count) in square_inches {
        if count > 1 {
            overlaps += 1;
        }
    }
    overlaps
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[Claim]) -> String {
    let mut square_inches = HashMap::<String, u32>::default();
    let mut candidates = Vec::<&Claim>::default();

    for claim in input {
        let mut is_overlapping = false;
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                let coordinates = format!("{},{}", x, y);
                let prev_count = square_inches.get(&coordinates).unwrap_or(&0);
                if *prev_count > 1 {
                    is_overlapping = true;
                }
                square_inches.insert(coordinates, prev_count + 1);
            }
        }
        if !is_overlapping {
            candidates.push(claim);
        }
    }
    for candidate in candidates {
        let mut is_valid = true;
        for x in candidate.left..candidate.left + candidate.width {
            for y in candidate.top..candidate.top + candidate.height {
                let coordinates = format!("{},{}", x, y);
                let coordinate_count = square_inches[&coordinates];
                if coordinate_count > 1 {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }

        if is_valid {
            return candidate.id.clone();
        }
    }
    unreachable!();
}
