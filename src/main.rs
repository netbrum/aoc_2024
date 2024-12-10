const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(usize, usize);

fn to_heads((y, line): (usize, &Vec<usize>)) -> Vec<Option<Point>> {
    line.iter()
        .enumerate()
        .map(
            |(x, char)| {
                if *char == 0 {
                    Some(Point(x, y))
                } else {
                    None
                }
            },
        )
        .collect()
}

fn to_digits(line: &str) -> Vec<usize> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap_or(1) as usize)
        .collect::<Vec<_>>()
}

fn main() {
    let map: Vec<Vec<usize>> = INPUT.lines().map(to_digits).collect();

    let trailheads: Vec<Point> = map
        .iter()
        .enumerate()
        .flat_map(to_heads)
        .flatten()
        .collect();

    let scores: usize = trailheads
        .iter()
        .map(|head| step(&map, *head, &mut Vec::new(), false))
        .sum();

    println!("Trailhead scores: {scores}");

    let distinct: usize = trailheads
        .iter()
        .map(|head| step(&map, *head, &mut Vec::new(), true))
        .sum();

    println!("Distinct trails: {distinct}");
}

fn step(map: &[Vec<usize>], position: Point, history: &mut Vec<Point>, part2: bool) -> usize {
    if !part2 {
        if history.iter().any(|p| *p == position) {
            return 0;
        }

        history.push(position);
    }

    let height = map.len();
    let width = map[0].len();

    let incline = map[position.1][position.0];
    let next = incline + 1;

    let mut current = 0;

    if incline == 9 {
        return 1;
    }

    if position.0 > 0 && map[position.1][position.0 - 1] == next {
        current += step(map, Point(position.0 - 1, position.1), history, part2);
    }

    if position.0 < width - 1 && map[position.1][position.0 + 1] == next {
        current += step(map, Point(position.0 + 1, position.1), history, part2);
    }

    if position.1 > 0 && map[position.1 - 1][position.0] == next {
        current += step(map, Point(position.0, position.1 - 1), history, part2);
    }

    if position.1 < height - 1 && map[position.1 + 1][position.0] == next {
        current += step(map, Point(position.0, position.1 + 1), history, part2);
    }

    current
}
