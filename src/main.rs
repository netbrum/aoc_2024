use std::fs;

fn is_solvable(current: usize, values: &[usize]) -> bool {
    let value = *values.last().unwrap();
    let length = values.len();

    if length == 1 {
        return value == current;
    }

    // Remove for the part one solution.
    //
    // The mask is 10^(amount of digits). So 6 would be 10, 15 would be 100, etc.
    //
    // Example 156: 15 6
    //
    // (156 (current) - 6 (value)) % 10 (mask) == 0
    //
    // Now we want to truncate the current number to essentially "reverse"
    // the concatenation, so we divide it by the mask.
    //
    // 156 (current) / 10 (mask) = 15 (It's integers, it gets rounded down.)
    //
    // The resulting 15 is the first two digits of the current number.
    //
    // We then hit the base case of length == 1, with the current number being 15 and the last
    // number in the values also 15!
    let mask = 10_usize.pow(value.checked_ilog10().unwrap_or(0) + 1);
    if current >= value
        && (current - value) % mask == 0
        && is_solvable(current / mask, &values[..length - 1])
    {
        return true;
    }

    if current % value == 0 && is_solvable(current / value, &values[..length - 1]) {
        return true;
    }

    if current >= value && is_solvable(current - value, &values[..length - 1]) {
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
        .filter(|(key, values)| is_solvable(*key, values))
        .map(|(key, _)| key)
        .sum();

    println!("{total}");
}
