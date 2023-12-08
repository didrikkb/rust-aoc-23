use std::rc::Rc;

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let times: Vec<i32> = input[0]
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    let distances: Vec<i32> = input[1]
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    return times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            let mut l = 0;
            let mut r = (time + 1) / 2;

            while l < r {
                let mid = (l + r) >> 1;
                if mid * (time - mid) <= distance {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            return time - 2 * r + 1;
        })
        .reduce(|acc, x| acc * x)
        .unwrap();
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let time = parse_line_as_num( input[0].clone()).unwrap();
    let distance =  parse_line_as_num( input[1].clone()).unwrap();

    let mut l = 0;
    let mut r = (time + 1) / 2;

    while l < r {
        let mid = (l + r) >> 1;
        if mid * (time - mid) <= distance {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return (time - 2 * r + 1) as i32;
}

fn parse_line_as_num(line: Rc<str>) -> Option<i64> {
    return line.as_bytes()
    .iter()
    .filter_map(|&ch| match ch {
        b'0'..=b'9' => Some((ch - b'0') as i64),
        _ => None,
    })
    .reduce(|acc, x| acc * 10 + x);
}

#[test]
fn part_1_test() {
    let data: Vec<Rc<str>> = vec![
        "Time:      7  15   30".into(),
        "Distance:  9  40  200".into(),
    ];
    assert_eq!(part_1(data), 288);
}

#[test]
fn part_2_test() {
    let data: Vec<Rc<str>> = vec![
        "Time:      7  15   30".into(),
        "Distance:  9  40  200".into(),
    ];
    assert_eq!(part_2(data), 71503);
}
