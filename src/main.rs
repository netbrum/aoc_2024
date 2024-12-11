use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

// Switch to 25 for part 1
const BLINKS: usize = 75;

fn digits(n: usize) -> usize {
    (n.ilog10() + 1) as usize
}

type Stones = HashMap<usize, usize>;

fn main() {
    let mut stones = INPUT
        .split_whitespace()
        .map(|c| (c.parse::<usize>().unwrap(), 1))
        .collect::<Stones>();

    for _ in 0..BLINKS {
        stones = blink(&stones);
    }

    println!(
        "Stones with {BLINKS} blinks: {}",
        stones.values().sum::<usize>()
    );
}

fn blink(old: &Stones) -> Stones {
    let mut stones = HashMap::with_capacity(old.len());

    for (stone, count) in old {
        if *stone == 0 {
            *stones.entry(1).or_default() += count;
            continue;
        }

        let digits = digits(*stone);
        if digits % 2 == 0 {
            let mask = 10_usize.pow(u32::try_from(digits / 2).unwrap());
            *stones.entry(stone / mask).or_default() += count;
            *stones.entry(stone % mask).or_default() += count;
        } else {
            *stones.entry(stone * 2024).or_default() += count;
        }
    }

    stones
}
