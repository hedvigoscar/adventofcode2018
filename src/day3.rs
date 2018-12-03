use fxhash::FxHashMap as HashMap;

pub struct Claim {
    _id: String,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn parse_day3(input: &str) -> Vec<Claim> {
    input.lines().map(|s| {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let edges = parts[2].split(',').collect::<Vec<&str>>();
        let sizes = parts[3].split('x').collect::<Vec<&str>>();
        Claim {
            _id: parts[0].to_string(),
            left: edges[0].parse().unwrap(),
            top: edges[1].trim_matches(':').parse().unwrap(),
            width: sizes[0].parse().unwrap(),
            height: sizes[1].parse().unwrap()
        }
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Claim]) -> u32 {
    let mut square_inches = HashMap::<String, u32>::default();
    for claim in input {
        for x in claim.left..claim.left+claim.width {
            for y in claim.top..claim.top+claim.height {
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
    unimplemented!()
}