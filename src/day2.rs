use fxhash::FxHashMap as HashMap;

#[aoc_generator(day2)]
pub fn parse_day2(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[String]) -> u32 {
    let mut n_twos = 0;
    let mut n_threes = 0;
    for i in input {
        let mut is_two = false;
        let mut is_three = false;

        let mut occurrences: HashMap<char, u8> = Default::default();

        for c in i.chars() {
            let prev_occurrences = occurrences.get(&c).unwrap_or(&0);
            occurrences.insert(c, prev_occurrences + 1);
        }

        for (_, count) in occurrences {
            if count == 2 {
                is_two = true;
            }
            if count == 3 {
                is_three = true;
            }
        }

        if is_two {
            n_twos += 1;
        }

        if is_three {
            n_threes += 1;
        }
    }
    n_twos * n_threes
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[String]) -> String {
    for tested in input {
        for i in input {
            if tested == i {
                continue;
            }
            let mut diffs = Vec::<usize>::default();
            for (index, (c1, c2)) in tested.chars().zip(i.chars()).enumerate() {
                if c1 != c2 {
                    diffs.push(index);
                }
                if diffs.len() > 1 {
                    break;
                }
            }
            if diffs.len() != 1 {
                continue;
            }
            let mut res = tested.clone();
            res.remove(diffs[0]);
            return res;
        }
    }
    panic!("Did not find the right answer");
}
