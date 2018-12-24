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

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &[Constraint]) -> String {
    let mut operations = Vec::<char>::default();
    let mut unmet_constraints = input.to_vec();

    let mut befores = HashSet::<char>::default();
    let mut afters = HashSet::<char>::default();
    for i in input {
        befores.insert(i.before);
        afters.insert(i.after);
    }

    let mut firsts = befores.difference(&afters).collect::<Vec<_>>();
    firsts.sort_unstable_by(|a, b| a.cmp(b));

    let mut unlocked = Vec::<char>::default();
    for f in firsts {
        unlocked.push(*f);
    }

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_solves_example() {
        let input = vec![
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
        ];

        let result = solve_day7_part1(&input);
        let expected = "CABDFE";

        assert!(
            result == expected,
            "Result mismatch: {} != {}",
            result,
            expected
        );
    }
}
