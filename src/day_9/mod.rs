use std::rc::Rc;

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    return parse_data(input)
        .into_iter()
        .map(|line| predict_next_val(line))
        .sum::<i64>() as i32;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    return parse_data(input)
        .into_iter()
        .map(|line| predict_prev_val(line))
        .sum::<i64>() as i32;
}

fn parse_data(input: Vec<Rc<str>>) -> Vec<Rc<[i64]>> {
    return input.into_iter().map(|line| parse_line(line)).collect();
}

fn parse_line(line: Rc<str>) -> Rc<[i64]> {
    return line
        .split_ascii_whitespace()
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Rc<[i64]>>();
}

fn predict_next_val(nums: Rc<[i64]>) -> i64 {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }
    let new_nums: Rc<[i64]> = create_diff_seq(nums.clone());
    return predict_next_val(new_nums) + nums.last().unwrap();
}

fn predict_prev_val(nums: Rc<[i64]>) -> i64 {
    if nums.iter().all(|x| *x == 0) {
        return 0;
    }
    let new_nums: Rc<[i64]> = create_diff_seq(nums.clone());
    return nums.first().unwrap() - predict_prev_val(new_nums);
}

fn create_diff_seq(seq: Rc<[i64]>) -> Rc<[i64]> {
    return seq.windows(2).map(|nums| nums[1] - nums[0]).collect();
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(test_data()), 114);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(test_data()), 2);
}

#[allow(dead_code)]
fn test_data() -> Vec<Rc<str>> {
    return vec![
        "0 3 6 9 12 15".into(),
        "1 3 6 10 15 21".into(),
        "10 13 16 21 30 45".into(),
    ];
}
