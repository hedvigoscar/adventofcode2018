use fxhash::FxHashSet as HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    before: char,
    after: char,
}

#[aoc_generator(day7)]
pub fn parse_day7(input: &str) -> Vec<Constraint> {
    input
        .lines()
        .map(|s| {
            let s = s.split_whitespace().collect::<Vec<&str>>();
            Constraint {
                before: s[1].chars().next().expect("Before character was empty"),
                after: s[7].chars().next().expect("After character was empty"),
            }
        })
        .collect()
}

fn find_initial(input: &[Constraint]) -> Vec<char> {
    let mut befores = HashSet::<char>::default();
    let mut afters = HashSet::<char>::default();
    for i in input {
        befores.insert(i.before);
        afters.insert(i.after);
    }

    let mut firsts = befores.difference(&afters).cloned().collect::<Vec<_>>();
    firsts.sort_unstable_by(|a, b| a.cmp(b));
    firsts
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &[Constraint]) -> String {
    let firsts = find_initial(input);

    let mut unlocked = Vec::<char>::default();
    for f in firsts {
        unlocked.push(f);
    }

    let mut operations = Vec::<char>::default();
    let mut unmet_constraints = input.to_vec();

    while !unmet_constraints.is_empty() {
        let currently_unlocked = unlocked.remove(0);
        operations.push(currently_unlocked);

        let mut met_constraints = Vec::<(usize, Constraint)>::default();

        for (i, uc) in unmet_constraints.iter().enumerate().rev() {
            if uc.before == currently_unlocked {
                met_constraints.push((i, uc.clone()));
            }
        }

        for (i, _) in &met_constraints {
            unmet_constraints.remove(*i);
        }

        for (_, mc) in met_constraints {
            let mut is_unlocked = true;
            for umc in &unmet_constraints {
                if umc.after == mc.after {
                    is_unlocked = false;
                    break;
                }
            }
            if is_unlocked {
                unlocked.push(mc.after);
            }
        }

        unlocked.sort_unstable_by(|a, b| a.cmp(b));
        unlocked.dedup();
    }

    operations.push(unlocked.remove(0));

    operations.iter().collect()
}

#[derive(Debug, PartialEq)]
enum WorkerStatus {
    Free,
    WorkingOnItem(char, u8),
}

impl WorkerStatus {
    fn decrement(&self) -> WorkerStatus {
        match self {
            WorkerStatus::Free => panic!("Tried to decrement a free worker"),
            WorkerStatus::WorkingOnItem(item, seconds) => {
                WorkerStatus::WorkingOnItem(*item, seconds - 1)
            }
        }
    }
}

fn get_item_weight(item: char, base_item_weight: u8) -> u8 {
    ((item as u8) - 64) + base_item_weight
}

fn solve_day7_part2_with_parameters(
    n_workers: usize,
    base_item_weight: u8,
    input: &[Constraint],
) -> u32 {
    let firsts = find_initial(input);

    let mut unlocked = Vec::<char>::default();
    for f in firsts {
        unlocked.push(f);
    }

    let mut workers = Vec::<WorkerStatus>::default();
    for _ in 0..n_workers {
        workers.push(WorkerStatus::Free);
    }

    let mut unmet_constraints = input.to_vec();
    let mut iterations = 0;

    loop {
        let mut free_workers = Vec::<usize>::default();
        let mut workers_to_decrement = Vec::<usize>::default();
        let mut unlocked_this_round = Vec::<char>::default();
        for (i, worker) in workers.iter().enumerate() {
            match worker {
                WorkerStatus::Free => {
                    free_workers.push(i);
                }
                WorkerStatus::WorkingOnItem(item, seconds_left) => {
                    if *seconds_left == 1 {
                        unlocked_this_round.push(*item);
                        free_workers.push(i);
                    } else {
                        workers_to_decrement.push(i)
                    }
                }
            };
        }

        for u in unlocked_this_round {
            let mut met_constraints = Vec::<(usize, Constraint)>::default();
            for (i, uc) in unmet_constraints.iter().enumerate().rev() {
                if uc.before == u {
                    met_constraints.push((i, uc.clone()));
                }
            }

            for (i, _) in &met_constraints {
                unmet_constraints.remove(*i);
            }

            for (_, mc) in met_constraints {
                let mut is_unlocked = true;
                for umc in &unmet_constraints {
                    if umc.after == mc.after {
                        is_unlocked = false;
                        break;
                    }
                }
                if is_unlocked {
                    unlocked.push(mc.after);
                }
            }

            unlocked.sort_unstable_by(|a, b| a.cmp(b));
            unlocked.dedup();
        }

        for f in free_workers {
            if !unlocked.is_empty() {
                let item = unlocked.remove(0);
                workers[f] =
                    WorkerStatus::WorkingOnItem(item, get_item_weight(item, base_item_weight));
            } else {
                workers[f] = WorkerStatus::Free;
            }
        }

        for w in workers_to_decrement {
            let worker = &workers[w];
            workers[w] = worker.decrement();
        }

        if unmet_constraints.is_empty()
            && workers.iter().all(|w| *w == WorkerStatus::Free)
            && unlocked.is_empty()
        {
            break;
        }

        iterations += 1;
    }

    iterations
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &[Constraint]) -> u32 {
    solve_day7_part2_with_parameters(5, 60, input)
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_input() -> Vec<Constraint> {
        vec![
            Constraint {
                before: 'C',
                after: 'A',
            },
            Constraint {
                before: 'C',
                after: 'F',
            },
            Constraint {
                before: 'A',
                after: 'B',
            },
            Constraint {
                before: 'A',
                after: 'D',
            },
            Constraint {
                before: 'B',
                after: 'E',
            },
            Constraint {
                before: 'D',
                after: 'E',
            },
            Constraint {
                before: 'F',
                after: 'E',
            },
        ]
    }

    #[test]
    fn part1_solves_example() {
        let input = make_input();
        let result = solve_day7_part1(&input);
        let expected = "CABDFE";

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_solves_example() {
        let input = make_input();
        let result = solve_day7_part2_with_parameters(2, 0, &input);
        let expected = 15;

        assert_eq!(result, expected);
    }
}
