use fxhash::FxHashSet as HashSet;

#[derive(Debug, Clone)]
pub struct PointVelocity {
    point: (i32, i32),
    velocity: (i32, i32),
}

#[aoc_generator(day10)]
pub fn parse_day10(input: &str) -> Vec<PointVelocity> {
    input
        .lines()
        .map(|l| {
            let (_, rest) = l.split_at(10);
            let x = rest
                .split(',')
                .nth(0)
                .expect("Could not extract Point X")
                .trim();
            let (_, rest) = l.split_at(18);
            let y = rest
                .split('>')
                .nth(0)
                .expect("Could not extract Point Y")
                .trim();
            let (_, rest) = l.split_at(36);
            let dx = rest
                .split(',')
                .nth(0)
                .expect("Could not extract Velocity X")
                .trim();
            let (_, rest) = l.split_at(40);
            let dy = rest
                .split('>')
                .nth(0)
                .expect("Could not extract Velocity Y")
                .trim();

            PointVelocity {
                point: (
                    x.parse().expect("Could not parse Point X"),
                    y.parse().expect("Could not extract Point Y"),
                ),
                velocity: (
                    dx.parse().expect("Could not parse Velocity X"),
                    dy.parse().expect("Could not parse Velocity Y"),
                ),
            }
        })
        .collect::<Vec<_>>()
}

fn render(input: &[PointVelocity]) -> String {
    let mut buf = String::default();
    buf.push('\n');

    let max_x = input
        .iter()
        .map(|i| i.point.0)
        .max()
        .expect("Found no min X value");
    let min_x = input
        .iter()
        .map(|i| i.point.0)
        .min()
        .expect("Found no max X value");
    let max_y = input
        .iter()
        .map(|i| i.point.1)
        .max()
        .expect("Found no max Y value");
    let min_y = input
        .iter()
        .map(|i| i.point.1)
        .min()
        .expect("Found no min Y value");

    let points = input.iter().map(|i| i.point).collect::<HashSet<_>>();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        buf.push('\n');
    }
    buf
}

fn move_points(input: &[PointVelocity]) -> Vec<PointVelocity> {
    let mut new_points = Vec::<PointVelocity>::with_capacity(input.len());
    for i in input {
        let (x, y) = i.point;
        let (dx, dy) = i.velocity;
        new_points.push(PointVelocity {
            point: (x + dx, y + dy),
            velocity: (dx, dy),
        });
    }
    new_points
}

fn solve_day10_inner(input: &[PointVelocity]) -> (String, u32) {
    let mut points = input.to_vec();

    let mut least_distance_x = i32::max_value();
    let mut counter = 0;

    loop {
        let max_x = points
            .iter()
            .map(|p| p.point.0 + p.velocity.0)
            .max()
            .expect("Found no maximum point");
        let min_x = points
            .iter()
            .map(|p| p.point.0 + p.velocity.0)
            .min()
            .expect("Found no minimum point");
        let distance_x = max_x - min_x;
        if distance_x > least_distance_x {
            break;
        }
        least_distance_x = distance_x;
        counter += 1;
        points = move_points(&points);
    }

    (render(&points), counter)
}

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &[PointVelocity]) -> String {
    let (res, _) = solve_day10_inner(input);
    res
}

#[aoc(day10, part2)]
pub fn solve_day10_part2(input: &[PointVelocity]) -> u32 {
    let (_, res) = solve_day10_inner(input);
    res
}
