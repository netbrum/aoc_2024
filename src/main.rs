#![expect(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn is_antenna(c: char) -> bool {
    c != '.'
}

type Map = HashMap<char, Vec<(isize, isize)>>;

fn part_one(map: &Map, (width, height): (usize, usize)) -> usize {
    let mut grid = vec![vec![0; width]; height];

    for positions in map.values() {
        for (i, first) in positions.iter().enumerate() {
            for (j, second) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                let x = second.0 + (second.0 - first.0);
                let y = second.1 + (second.1 - first.1);

                if x >= 0 && y >= 0 && x < width as isize && y < height as isize {
                    grid[y as usize][x as usize] = 1;
                }
            }
        }
    }

    let total = grid.iter().map(|g| g.iter().sum::<usize>()).sum::<usize>();

    total
}

fn part_two(map: &Map, (width, height): (usize, usize)) -> usize {
    let mut grid = vec![vec![0; width]; height];

    for positions in map.values() {
        for (i, first) in positions.iter().enumerate() {
            for (j, second) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                let distance = (second.0 - first.0, second.1 - first.1);
                let (mut x, mut y) = second;

                while x >= 0 && y >= 0 && x < width as isize && y < height as isize {
                    grid[y as usize][x as usize] = 1;

                    x += distance.0;
                    y += distance.1;
                }
            }
        }
    }

    let total = grid.iter().map(|g| g.iter().sum::<usize>()).sum::<usize>();

    total
}

fn main() {
    let mut map: Map = HashMap::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, char) in line.chars().enumerate().filter(|(_, c)| is_antenna(*c)) {
            map.entry(char).or_default().push((x as isize, y as isize));
        }
    }

    let lines = INPUT.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines[0].len();

    let antinodes = part_one(&map, (width, height));
    let harmonic_antinodes = part_two(&map, (width, height));

    println!("Total: {antinodes}");
    println!("Total with resonant harmonics: {harmonic_antinodes}");
}
