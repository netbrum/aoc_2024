#![expect(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn corners(map: &[Vec<char>], (x, y): (isize, isize)) -> usize {
    (0..=3)
        .map(|d| (DIRECTIONS[d], DIRECTIONS[(d + 1) % 4]))
        .map(|((dx1, dy1), (dx2, dy2))| {
            let plant = if let Some(row) = map.get(y as usize) {
                row.get(x as usize)
            } else {
                None
            };

            let left = if let Some(row) = map.get((y + dy1) as usize) {
                row.get((x + dx1) as usize)
            } else {
                None
            };

            let right = if let Some(row) = map.get((y + dy2) as usize) {
                row.get((x + dx2) as usize)
            } else {
                None
            };

            let corner = if let Some(row) = map.get((y + dy1 + dy2) as usize) {
                row.get((x + dx1 + dx2) as usize)
            } else {
                None
            };

            (plant, left, right, corner)
        })
        .filter(|(plant, left, right, corner)| {
            left != plant && right != plant || (left == plant && right == plant && corner != plant)
        })
        .count()
}

fn plots(
    map: &[Vec<char>],
    visited: &mut HashSet<(isize, isize)>,
    (x, y): (usize, usize),
) -> usize {
    if visited.contains(&(x as isize, y as isize)) {
        return 0;
    }

    let height = map.len();
    let width = map[0].len();

    let plant = map[y][x];
    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    let mut area = 1;
    let mut perimeter = 4;
    let mut corner_count = corners(map, (x as isize, y as isize));

    visited.insert((x as isize, y as isize));

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in DIRECTIONS {
            let x = x as isize + dx;
            let y = y as isize + dy;

            if x < 0
                || y < 0
                || x >= width as isize
                || y >= height as isize
                || map[y as usize][x as usize] != plant
            {
                continue;
            }

            perimeter -= 1;

            if !visited.contains(&(x, y)) {
                area += 1;
                perimeter += 4;
                corner_count += corners(map, (x, y));
                queue.push_back((x as usize, y as usize));
                visited.insert((x, y));
            }
        }
    }

    // Switch `corner_count` with `perimeter` for part 1
    area * corner_count
}

fn main() {
    let map = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    let mut total = 0;

    for (y, line) in map.iter().enumerate() {
        for x in 0..line.len() {
            total += plots(&map, &mut visited, (x, y));
        }
    }

    println!("Total: {total}");
}
