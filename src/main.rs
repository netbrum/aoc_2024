use std::ops::AddAssign;

const INPUT: &str = include_str!("input.txt");

const SECONDS: usize = 100;
const HEIGHT: isize = 103;
const WIDTH: isize = 101;
const HORIZONTAL_SPLIT: isize = WIDTH / 2;
const VERTICAL_SPLIT: isize = HEIGHT / 2;

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

#[derive(Debug, Clone, Copy)]
struct Point(isize, isize);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn numbers(input: &str) -> Point {
    let numbers = input
        .split(',')
        .map(|d| d.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    Point(numbers[0], numbers[1])
}

fn quadrants(robots: &[Robot]) -> [usize; 4] {
    let mut quadrants = [0, 0, 0, 0];
    for robot in robots {
        let x = robot.position.0;
        let y = robot.position.1;

        if x < HORIZONTAL_SPLIT && y < VERTICAL_SPLIT {
            quadrants[0] += 1;
        } else if x > HORIZONTAL_SPLIT && y < VERTICAL_SPLIT {
            quadrants[1] += 1;
        } else if x < HORIZONTAL_SPLIT && y > VERTICAL_SPLIT {
            quadrants[2] += 1;
        } else if x > HORIZONTAL_SPLIT && y > VERTICAL_SPLIT {
            quadrants[3] += 1;
        }
    }

    quadrants
}

fn main() {
    let mut robots = Vec::new();

    for line in INPUT.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();

        let p = split[0].strip_prefix("p=").unwrap();
        let v = split[1].strip_prefix("v=").unwrap();

        robots.push(Robot {
            position: numbers(p),
            velocity: numbers(v),
        });
    }

    for _ in 0..SECONDS {
        for robot in &mut robots {
            robot.position += robot.velocity;

            if robot.position.1 >= HEIGHT {
                robot.position.1 -= HEIGHT;
            } else if robot.position.1 < 0 {
                robot.position.1 += HEIGHT;
            }

            if robot.position.0 >= WIDTH {
                robot.position.0 -= WIDTH;
            } else if robot.position.0 < 0 {
                robot.position.0 += WIDTH;
            }
        }
    }

    println!(
        "Total: {:?}",
        quadrants(&robots)
            .into_iter()
            .reduce(|a, b| (a * b))
            .unwrap()
    );
}
