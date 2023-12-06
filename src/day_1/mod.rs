use std::rc::Rc;

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    return input
        .iter()
        .map(|line| {
            let first_digit = line
                .as_bytes()
                .iter()
                .find(|&&ch| ch >= b'0' && ch <= b'9')
                .unwrap()
                - b'0';
            let last_digit = line
                .as_bytes()
                .iter()
                .rfind(|&&ch| ch >= b'0' && ch <= b'9')
                .unwrap()
                - b'0';
            return (first_digit * 10 + last_digit) as i32;
        })
        .sum();
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    return input
        .iter()
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;

            for i in 0..line.len() {
                if let Some(val) = find_first_value(&line[i..]) {
                    first_digit = val;
                    break;
                }
            }

            for i in (0..=line.len()).rev() {
                if let Some(val) = find_last_value(&line[..i]) {
                    last_digit = val;
                    break;
                }
            }

            return (first_digit * 10 + last_digit) as i32;
        })
        .sum();
}

fn find_first_value(word: &str) -> Option<i32> {
    for x in words_and_values() {
        if word.starts_with(x.0) {
            return Some(x.1);
        }
    }
    return None;
}

fn find_last_value(word: &str) -> Option<i32> {
    for x in words_and_values() {
        if word.ends_with(x.0) {
            return Some(x.1);
        }
    }
    return None;
}

fn words_and_values() -> [(&'static str, i32); 18] {
    [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
}

#[test]
fn part_1_test() {
    let input: Vec<Rc<str>> = vec![
        "1abc2".into(),
        "pqr3stu8vwx".into(),
        "a1b2c3d4e5f".into(),
        "treb7uchet".into(),
    ];
    assert_eq!(part_1(input), 142);
}

#[test]
fn part_2_test() {
    let input: Vec<Rc<str>> = vec![
        "two1nine".into(),
        "eightwothree".into(),
        "abcone2threexyz".into(),
        "xtwone3four".into(),
        "4nineeightseven2".into(),
        "zoneight234".into(),
        "7pqrstsixteen".into(),
    ];
    assert_eq!(part_2(input), 281);
}
