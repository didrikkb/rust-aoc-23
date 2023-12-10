use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Directions {
    dirs: Vec<Direction>,
    i: usize,
}

impl Directions {
    fn new(line: &str) -> Directions {
        Directions {
            dirs: Self::parse(line),
            i: 0,
        }
    }

    fn next(&mut self) -> &Direction {
        let len = self.dirs.len();
        let dir = &self.dirs[self.i % len];
        self.i += 1;
        return dir;
    }

    fn parse(line: &str) -> Vec<Direction> {
        return line
            .as_bytes()
            .into_iter()
            .filter_map(|&dir| match dir {
                b'L' => Some(Direction::Left),
                b'R' => Some(Direction::Right),
                _ => None,
            })
            .collect();
    }

    pub fn reset(&mut self) {
        self.i = 0;
    }
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let mut input = input.iter();
    let mut dirs = Directions::new(input.next().unwrap());
    let map = create_map(input);

    let mut steps = 0;
    let mut curr_pos: Rc<[u8]> = Rc::new([b'A', b'A', b'A']);
    let end_pos: Rc<[u8]> = Rc::new([b'Z', b'Z', b'Z']);

    while curr_pos != end_pos {
        curr_pos = match dirs.next() {
            Direction::Right => map.get(&curr_pos).unwrap().1.clone(),
            Direction::Left => map.get(&curr_pos).unwrap().0.clone(),
        };

        steps += 1;
    }
    return steps;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let mut input = input.iter();
    let mut dirs = Directions::new(input.next().unwrap());
    let map = create_map(input);

    let mut curr_pos: Vec<Rc<[u8]>> = Vec::new();
    for (key, _) in &map {
        if key[2] == b'A' {
            curr_pos.push(key.clone());
        }
    }

    let mut all_steps = vec![];

    for i in 0..curr_pos.len() {
        dirs.reset();
        let mut steps = 0;
        while curr_pos[i][2] != b'Z' {
            let dir = dirs.next();

            curr_pos[i] = match dir {
                Direction::Right => map.get(&curr_pos[i]).unwrap().1.clone(),
                Direction::Left => map.get(&curr_pos[i]).unwrap().0.clone(),
            };
            steps += 1;
        }
        all_steps.push(steps as i64)
    }

    let lcm = lcm_arr(&all_steps);

    println!("Res: {}", lcm);

    return lcm as i32;
}

fn create_map(input: std::slice::Iter<'_, Rc<str>>) -> HashMap<Rc<[u8]>, (Rc<[u8]>, Rc<[u8]>)> {
    let mut map = HashMap::new();
    input.skip(1).for_each(|line| {
        if line.len() > 0 {
            let line: Vec<&str> = line.split_ascii_whitespace().collect();
            map.insert(parse_pos(line[0]), (parse_pos(line[2]), parse_pos(line[3])));
        }
    });
    return map;
}

fn lcm_arr(arr: &[i64]) -> i64 {
    if arr.len() == 2 {
        return lcm(arr[0], arr[1]);
    }
    return lcm(arr[0], lcm_arr(&arr[1..]));
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut c = 1;
    while c != 0 {
        c = a % b;
        a = b;
        b = c;
    }
    return a;
}

fn parse_pos(pos: &str) -> Rc<[u8]> {
    return pos
        .as_bytes()
        .iter()
        .filter_map(|&ch| match ch {
            b'A'..=b'Z' | b'0'..=b'9' => Some(ch),
            _ => None,
        })
        .collect();
}

#[test]
fn part_1_test() {
    assert_eq!((part_1(test_data_1()), part_1(test_data_2())), (2, 6));
}

#[test]
fn part_2_test() {
    let test_data = vec![
        "LR".into(),
        "".into(),
        "11A = (11B, XXX)".into(),
        "11B = (XXX, 11Z)".into(),
        "11Z = (11B, XXX)".into(),
        "22A = (22B, XXX)".into(),
        "22B = (22C, 22C)".into(),
        "22C = (22Z, 22Z)".into(),
        "22Z = (22B, 22B)".into(),
        "XXX = (XXX, XXX)".into(),
    ];
    assert_eq!(part_2(test_data), 6);
}

#[allow(dead_code)]
fn test_data_1() -> Vec<Rc<str>> {
    return vec![
        "RL".into(),
        "".into(),
        "AAA = (BBB, CCC)".into(),
        "BBB = (DDD, EEE)".into(),
        "CCC = (ZZZ, GGG)".into(),
        "DDD = (DDD, DDD)".into(),
        "EEE = (EEE, EEE)".into(),
        "GGG = (GGG, GGG)".into(),
        "ZZZ = (ZZZ, ZZZ)".into(),
    ];
}

#[allow(dead_code)]
fn test_data_2() -> Vec<Rc<str>> {
    return vec![
        "LLR".into(),
        "".into(),
        "AAA = (BBB, BBB)".into(),
        "BBB = (AAA, ZZZ)".into(),
        "ZZZ = (ZZZ, ZZZ)".into(),
    ];
}
