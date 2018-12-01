use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn solve_day1(input: &str) -> i32 {
    input.lines().map(|s| s.parse::<i32>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn solve_day2(input: &str) -> i32 {
    let mut results: HashSet<i32> = HashSet::new();
    let input: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut state = 0;
    loop {
        for i in &input {
            state += i;
            if !results.insert(state) {
                return state;
            }
        };
    }
}
