use std::rc::Rc;

#[derive(Debug)]
struct Range {
    dest_range_start: i64,
    source_range_start: i64,
    range_len: i64,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let mut input = input.iter();

    let seeds: Vec<i64> = input
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let mut maps = Vec::new();
    let mut ranges = Vec::new();

    while let Some(line) = input.next() {
        if line.len() == 0 {
            continue;
        }

        if line.contains("map") {
            maps.push(ranges);
            ranges = Vec::new();
            continue;
        }

        let range = line
            .split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        let range = Range {
            dest_range_start: range[0],
            source_range_start: range[1],
            range_len: range[2],
        };

        ranges.push(range);
    }
    maps.push(ranges);

    let mut location = i64::MAX;

    for seed in seeds {
        let mut curr_val = seed;
        for map in maps.iter() {
            for range in map {
                if curr_val >= range.source_range_start
                    && curr_val < range.source_range_start + range.range_len
                {
                    curr_val = range.dest_range_start + (curr_val - range.source_range_start);
                    break;
                }
            }
        }
        location = location.min(curr_val);
    }
    return location as i32;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let mut input = input.iter();

    let seeds: Vec<i64> = input
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect();

    let mut maps = Vec::new();
    let mut ranges = Vec::new();

    while let Some(line) = input.next() {
        if line.len() == 0 {
            continue;
        }

        if line.contains("map") {
            maps.push(ranges);
            ranges = Vec::new();
            continue;
        }

        let range = line
            .split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        let range = Range {
            dest_range_start: range[0],
            source_range_start: range[1],
            range_len: range[2],
        };

        ranges.push(range);
    }
    maps.push(ranges);

    let mut location = i64::MAX;

    for seed_pair_idx in (0..seeds.len()).step_by(2) {
        let (start, end) = (
            seeds[seed_pair_idx],
            seeds[seed_pair_idx] + seeds[seed_pair_idx + 1],
        );
        for seed in start..end {
            let mut curr_val = seed;
            for map in maps.iter() {
                for range in map {
                    if curr_val >= range.source_range_start
                        && curr_val < range.source_range_start + range.range_len
                    {
                        curr_val = range.dest_range_start + (curr_val - range.source_range_start);
                        break;
                    }
                }
            }
            location = location.min(curr_val);
        }
        println!("Pair {} of {} pairs", seed_pair_idx/2, seeds.len()/2);
    }


    return location as i32;
}
