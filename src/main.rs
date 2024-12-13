const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point(isize, isize);

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn numbers(str: &str) -> Point {
    let digits = str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ' ')
        .collect::<String>();

    let numbers = digits
        .split_whitespace()
        .map(|d| d.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    Point(numbers[0], numbers[1])
}

fn parse_input() -> Vec<Machine> {
    let mut lines = INPUT.lines();
    let mut machines = Vec::new();

    while let (Some(a), Some(b), Some(prize), _) =
        (lines.next(), lines.next(), lines.next(), lines.next())
    {
        let a = numbers(a);
        let b = numbers(b);
        let prize = numbers(prize);

        machines.push(Machine { a, b, prize });
    }

    machines
}

fn play(machine: &Machine, part2: bool) -> isize {
    let Point(ax, ay) = machine.a;
    let Point(bx, by) = machine.b;
    let Point(mut px, mut py) = machine.prize;

    if part2 {
        px += 10_000_000_000_000;
        py += 10_000_000_000_000;
    }

    let b = (py * ax - px * ay) / (by * ax - bx * ay);
    let a = (px - b * bx) / ax;

    if ax * a + bx * b == px && ay * a + by * b == py {
        3 * a + b
    } else {
        0
    }
}

fn main() {
    let machines = parse_input();

    let total: isize = machines.iter().map(|m| play(m, false)).sum();
    let total_with_offset: isize = machines.iter().map(|m| play(m, true)).sum();

    println!("Total: {total}");
    println!("Total with offset: {total_with_offset}");
}
