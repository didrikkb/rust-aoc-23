use std::rc::Rc;

#[derive(Debug)]
struct Range {
    dest_range_start: i64,
    source_range_start: i64,
    range_len: i64,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    println!("{:?}", input);
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
    }

    return location as i32;
}

#[test]
fn part_1_test() {
    assert_eq!(35, part_1(test_data()));
}

#[test]
fn part_2_test() {
    assert_eq!(46, part_2(test_data()));
}

fn test_data() -> Vec<Rc<str>> {
    return vec![
        "seeds: 79 14 55 13".into(),
        "".into(),
        "seed-to-soil map:".into(),
        "50 98 2".into(),
        "52 50 48".into(),
        "".into(),
        "soil-to-fertilizer map:".into(),
        "0 15 37".into(),
        "37 52 2".into(),
        "39 0 15".into(),
        "".into(),
        "fertilizer-to-water map:".into(),
        "49 53 8".into(),
        "0 11 42".into(),
        "42 0 7".into(),
        "57 7 4".into(),
        "".into(),
        "water-to-light map:".into(),
        "88 18 7".into(),
        "18 25 70".into(),
        "".into(),
        "light-to-temperature map:".into(),
        "45 77 23".into(),
        "81 45 19".into(),
        "68 64 13".into(),
        "".into(),
        "temperature-to-humidity map:".into(),
        "0 69 1".into(),
        "1 0 69".into(),
        "".into(),
        "humidity-to-location map:".into(),
        "60 56 37".into(),
        "56 93 4".into(),
    ];
}
