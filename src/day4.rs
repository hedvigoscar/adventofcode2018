use chrono::{NaiveDateTime, Timelike};
use fxhash::FxHashMap as HashMap;

#[derive(Debug)]
pub enum RecordType {
    BeginsShift(String),
    WakesUp,
    FallsAsleep,
}

#[derive(Debug)]
pub struct Record {
    date: NaiveDateTime,
    record_type: RecordType,
}

#[aoc_generator(day4)]
pub fn parse_day4(input: &str) -> Vec<Record> {
    let mut input = input
        .lines()
        .map(|s| {
            let (date, rest) = s.split_at(18);
            let date = NaiveDateTime::parse_from_str(date, "[%Y-%m-%d %H:%M]")
                .expect("Failed to parse date");

            let record_type = if rest.contains("begins shift") {
                let id = rest
                    .split_whitespace()
                    .nth(1)
                    .expect("Problem unwrapping nth(1) element");
                RecordType::BeginsShift(id.to_string())
            } else if rest.contains("wakes up") {
                RecordType::WakesUp
            } else if rest.contains("falls asleep") {
                RecordType::FallsAsleep
            } else {
                unreachable!();
            };

            Record { date, record_type }
        })
        .collect::<Vec<_>>();
    input.sort_unstable_by(|a, b| a.date.cmp(&b.date));
    input
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(input: &[Record]) -> u32 {
    let mut minutes_slept_by_guard = HashMap::<String, u64>::default();

    let mut current_guard_id = String::default();
    let mut fell_asleep_at = NaiveDateTime::from_timestamp(0, 0);

    for i in input {
        match &i.record_type {
            RecordType::BeginsShift(id) => current_guard_id = id.to_string(),
            RecordType::FallsAsleep => fell_asleep_at = i.date,
            RecordType::WakesUp => {
                let minutes_asleep = i.date.signed_duration_since(fell_asleep_at).num_minutes();
                let minutes_currently_slept =
                    minutes_slept_by_guard.get(&current_guard_id).unwrap_or(&0);
                minutes_slept_by_guard.insert(
                    current_guard_id.clone(),
                    minutes_currently_slept + minutes_asleep as u64,
                );
            }
        }
    }

    let mut minutes_slept_by_guard = minutes_slept_by_guard.iter().collect::<Vec<_>>();
    minutes_slept_by_guard.sort_unstable_by(|(_, a), (_, b)| b.cmp(&a));
    let (max_slept_guard_id, _) = minutes_slept_by_guard[0];

    let mut minute_mapping = HashMap::<u32, u32>::default();

    for i in input {
        match &i.record_type {
            RecordType::BeginsShift(id) => current_guard_id = id.to_string(),
            RecordType::WakesUp => {
                if &current_guard_id != max_slept_guard_id {
                    continue;
                }

                for m in fell_asleep_at.minute()..i.date.minute() {
                    let prev_minute = minute_mapping.get(&m).unwrap_or(&0);
                    minute_mapping.insert(m, prev_minute + 1);
                }
            }
            RecordType::FallsAsleep => {
                if &current_guard_id != max_slept_guard_id {
                    continue;
                }

                fell_asleep_at = i.date;
            }
        }
    }

    let mut minute_mapping = minute_mapping.iter().collect::<Vec<_>>();
    minute_mapping.sort_unstable_by(|(_, a), (_, b)| b.cmp(&a));

    let (_, max_slept_guard_id) = max_slept_guard_id.split_at(1);
    let max_slept_guard_id = max_slept_guard_id
        .parse::<u32>()
        .expect("Error parsing guard id");

    let (max_minute_slept, _) = minute_mapping[0];

    max_slept_guard_id * max_minute_slept
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(input: &[Record]) -> u32 {
    let mut guard_minute_mapping = HashMap::<String, HashMap<u32, u32>>::default();

    let mut current_guard_id = String::default();
    let mut fell_asleep_at = NaiveDateTime::from_timestamp(0, 0);

    for i in input {
        match &i.record_type {
            RecordType::BeginsShift(id) => current_guard_id = id.to_string(),
            RecordType::WakesUp => {
                if !guard_minute_mapping.contains_key(&current_guard_id) {
                    guard_minute_mapping
                        .insert(current_guard_id.clone(), HashMap::<u32, u32>::default());
                }
                let current_guard_record = guard_minute_mapping
                    .get_mut(&current_guard_id)
                    .expect("Guard minute mapping is impossibly not present");
                for m in fell_asleep_at.minute()..i.date.minute() {
                    let prev_minute = current_guard_record.get(&m).unwrap_or(&0);
                    current_guard_record.insert(m, prev_minute + 1);
                }
            }
            RecordType::FallsAsleep => fell_asleep_at = i.date,
        }
    }

    let mut guard_minute_mapping = guard_minute_mapping
        .iter()
        .map(|(id, minutes)| {
            let mut minutes = minutes.iter().collect::<Vec<_>>();
            minutes.sort_unstable_by(|(_, a), (_, b)| b.cmp(&a));

            (id, minutes[0])
        })
        .collect::<Vec<_>>();
    guard_minute_mapping.sort_unstable_by(|(_, (_, a)), (_, (_, b))| b.cmp(&a));

    let (guard_id, (minute, _)) = guard_minute_mapping[0];

    let (_, guard_id) = guard_id.split_at(1);
    let guard_id = guard_id.parse::<u32>().expect("Error parsing guard id");

    guard_id * minute
}
