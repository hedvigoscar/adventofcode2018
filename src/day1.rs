use fxhash::FxHashSet as HashSet;

#[aoc_generator(day1)]
pub fn parse_day1(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i32]) -> i32 {
    let mut results = HashSet::<i32>::default();
    let mut state = 0;
    loop {
        for i in input {
            state += i;
            if !results.insert(state) {
                return state;
            }
        };
    }
}
