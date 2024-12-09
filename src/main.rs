const INPUT: &str = include_str!("input.txt");

fn to_disk(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(index, chars)| {
            let digit = chars[0].to_digit(10).unwrap() as usize;
            let free = chars.get(1).unwrap_or(&'0').to_digit(10).unwrap() as usize;

            let mut blocks = vec![Some(index); digit];
            blocks.append(&mut vec![None; free]);

            blocks
        })
        .collect()
}

fn part_one(mut disk: Vec<Option<usize>>) -> usize {
    let mut start = 0;
    let mut end = disk.len() - 1;

    while start < end {
        while disk[end].is_none() {
            end -= 1;
        }

        while start != end && disk[start].is_some() {
            start += 1;
        }

        disk.swap(start, end);
    }

    let total: usize = disk
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, num)| index * num)
        .sum();

    total
}

fn part_two(mut disk: Vec<Option<usize>>) -> usize {
    let mut block_start = 0;

    let mut current_id = disk[0];

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut gaps: Vec<(usize, usize)> = Vec::new();

    for (index, file) in disk.iter().enumerate() {
        if current_id != *file {
            if disk[block_start].is_none() {
                gaps.push((block_start, index));
            } else {
                blocks.push((block_start, index));
            }

            block_start = index;
            current_id = *file;
        }
    }

    blocks.push((block_start, disk.len()));

    for (block_start, block_end) in blocks.iter().rev() {
        let block_size = block_end - block_start;

        for (gap_start, gap_end) in &mut gaps {
            let gap_size = *gap_end - *gap_start;

            if gap_size >= block_size && block_start > gap_start {
                for i in 0..block_size {
                    disk.swap(*gap_start + i, block_start + i);
                }

                *gap_start += block_size;

                break;
            }
        }
    }

    let total: usize = disk
        .iter()
        .enumerate()
        .filter(|(_, file)| file.is_some())
        .map(|(index, num)| index * num.unwrap())
        .sum();

    total
}

fn main() {
    let disk = to_disk(INPUT.trim());

    let checksum = part_one(disk.clone());
    let checksum_fm = part_two(disk);

    println!("Total checksum: {checksum}");
    println!("Total checksum (moving whole files): {checksum_fm}");
}
