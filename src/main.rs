use std::fs;

fn is_solvable(target: usize, current: usize, values: &[usize]) -> bool {
    if values.is_empty() {
        return current == target;
    }

    let value = values[0];

    // Remove for the part one solution.
    if is_solvable(
        target,
        format!("{current}{value}").parse().unwrap(),
        &values[1..],
    ) {
        return true;
    }

    if is_solvable(target, current * value, &values[1..]) {
        return true;
    }

    if is_solvable(target, current + value, &values[1..]) {
        return true;
    }

    false
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let data = lines.map(|line| {
        let (target, values) = line.split_once(": ").unwrap();
        let target = target.parse::<usize>().unwrap();
        let values: Vec<usize> = values
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        (target, values)
    });

    let total: usize = data
        .filter(|(key, values)| is_solvable(*key, 0, values))
        .map(|(key, _)| key)
        .sum();

    println!("{total}");
}
