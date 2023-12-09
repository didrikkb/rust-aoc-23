use std::rc::Rc;

struct Card {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let cards: Vec<Card> = input
        .iter()
        .filter_map(|line| {
            if line.len() < 1 {
                return None;
            }
            return Some(parse_card(&*line));
        })
        .collect();

    let mut res = 0;

    for card in cards {
        let mut temp_sum = 0;
        for own_num in &card.numbers {
            for win_num in &card.winning_numbers {
                if own_num == win_num {
                    if temp_sum > 0 {
                        temp_sum <<= 1;
                    } else {
                        temp_sum += 1;
                    }
                }
            }
        }
        res += temp_sum;
    }

    return res;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let cards: Vec<Card> = input
        .iter()
        .filter_map(|line| {
            if line.len() < 1 {
                return None;
            }
            return Some(parse_card(&*line));
        })
        .collect();

    let mut card_collection = vec![1; cards.len()];

    for (id, card) in cards.into_iter().enumerate() {
        let mut temp_sum = 0;

        for own_num in &card.numbers {
            for win_num in &card.winning_numbers {
                if own_num == win_num {
                    temp_sum += 1;
                }
            }
        }

        for i in id + 1..=id + temp_sum {
            if i >= card_collection.len() {
                break;
            }
            card_collection[i] += card_collection[id];
        }
    }

    return card_collection.into_iter().sum::<usize>() as i32;
}

fn parse_card(line: &str) -> Card {
    let card: Vec<&str> = line.split("|").collect();
    return Card {
        winning_numbers: parse_numbers(card[0]),
        numbers: parse_numbers(card[1]),
    };
}

fn parse_numbers(nums: &str) -> Vec<i32> {
    return nums
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();
}

#[test]
fn part_1_test() {
    assert_eq!(13, part_1(test_data()));
}
#[test]
fn part_2_test() {
    assert_eq!(30, part_2(test_data()));
}

#[allow(dead_code)]
fn test_data() -> Vec<Rc<str>> {
    return vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
    ];
}
