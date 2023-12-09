use std::rc::Rc;

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let times: Vec<i32> = get_nums_in_line(input[0].clone());
    let distances: Vec<i32> = get_nums_in_line(input[1].clone());

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
    let time = parse_line_as_num(input[0].clone());
    let distance = parse_line_as_num(input[1].clone());

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

fn get_nums_in_line(line: Rc<str>) -> Vec<i32> {
    return line
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
}

fn parse_line_as_num(line: Rc<str>) -> i64 {
    return line
        .as_bytes()
        .iter()
        .filter_map(|&ch| match ch {
            b'0'..=b'9' => Some((ch - b'0') as i64),
            _ => None,
        })
        .reduce(|acc, x| acc * 10 + x)
        .unwrap();
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
