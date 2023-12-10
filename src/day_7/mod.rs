use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Hand {
    cards: u64,
    bid: i32,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let hands = parse_hands(input, true);

    return calculate_tot_winnings(hands) as i32;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let hands = parse_hands(input, false);

    return calculate_tot_winnings(hands) as i32;
}

fn calculate_tot_winnings(mut hands: Vec<Hand>) -> i64 {
    hands.sort_unstable_by_key(|hand| hand.cards);

    return hands
        .iter()
        .enumerate()
        .fold(0, |total_winnings, (rank, hand)| {
            total_winnings + hand.bid as i64 * (rank as i64 + 1)
        });
}

fn parse_hands(input: Vec<Rc<str>>, is_part_1: bool) -> Vec<Hand> {
    return input
        .into_iter()
        .filter_map(|line| {
            if line.len() < 1 {
                return None;
            }
            Some(parse_hand(&*line, is_part_1))
        })
        .collect();
}

fn parse_hand(line: &str, is_part_1: bool) -> Hand {
    let data: Vec<&str> = line.split_ascii_whitespace().collect();
    let hand = data[0].as_bytes();

    return Hand {
        cards: match is_part_1 {
            true => hand_type_1(hand) + find_value_of_cards_1(hand),
            false => hand_type_2(hand) + find_value_of_cards_2(hand),
        },
        bid: data[1].parse::<i32>().unwrap(),
    };
}

fn hand_type_1(cards: &[u8]) -> u64 {
    let card_map = cards.iter().fold(HashMap::new(), |mut map, card| {
        match map.get_mut(card) {
            Some(val) => *val += 1,
            None => {
                map.insert(*card, 1u8);
            }
        }
        return map;
    });

    fn five_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() == 1 {
            return 7;
        }
        return four_of_a_kind(card_map);
    }

    fn four_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        for (_, value) in &card_map {
            if *value == 4 {
                return 6;
            }
        }
        return full_house(card_map);
    }

    fn full_house(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() == 2 {
            return 5;
        }
        return three_of_a_kind(card_map);
    }

    fn three_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        for (_, value) in &card_map {
            if *value == 3 {
                return 4;
            }
        }
        return two_pairs(card_map);
    }

    fn two_pairs(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() == 3 {
            return 3;
        }
        return one_pair(card_map);
    }

    fn one_pair(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() == 4 {
            return 2;
        }
        return 1;
    }

    return five_of_a_kind(card_map) * 100000000000;
}

fn find_value_of_cards_1(cards: &[u8]) -> u64 {
    let mut offset = 1;
    let mut hand_value = 0;

    for &card in cards.into_iter().rev() {
        hand_value += card_values_part_1(card) * offset;
        offset *= 100;
    }
    return hand_value;
}

fn card_values_part_1(card: u8) -> u64 {
    return match card {
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    };
}

fn hand_type_2(cards: &[u8]) -> u64 {
    if cards.contains(&b'J') == false {
        return hand_type_1(cards);
    }

    let card_map = cards.iter().fold(HashMap::new(), |mut map, card| {
        match map.get_mut(card) {
            Some(val) => *val += 1,
            None => {
                map.insert(*card, 1u8);
            }
        }
        return map;
    });

    fn five_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() <= 2 {
            return 7;
        }
        return four_of_a_kind(card_map);
    }

    fn four_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() < 4 {
            for (_, &value) in &card_map {
                if (value == 3) || (value == 2 && *card_map.get(&b'J').unwrap() != 1) {
                    return 6;
                }
            }
        }
        return full_house(card_map);
    }

    fn full_house(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() == 3 && *card_map.get(&b'J').unwrap() != 3 {
            return 5;
        }
        return three_of_a_kind(card_map);
    }

    fn three_of_a_kind(card_map: HashMap<u8, u8>) -> u64 {
        if card_map.len() < 5 {
            return 4;
        }
        return 2;
    }

    return five_of_a_kind(card_map) * 100000000000;
}

fn find_value_of_cards_2(cards: &[u8]) -> u64 {
    let mut offset = 1;
    let mut hand_value = 0;
    for &card in cards.into_iter().rev() {
        hand_value += card_values_part_2(card) * offset;
        offset *= 100;
    }
    return hand_value;
}

fn card_values_part_2(card: u8) -> u64 {
    return match card {
        b'J' => 2,
        b'2' => 3,
        b'3' => 4,
        b'4' => 5,
        b'5' => 6,
        b'6' => 7,
        b'7' => 8,
        b'8' => 9,
        b'9' => 10,
        b'T' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    };
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(test_data()), 6440);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(test_data()), 5905);
}

#[allow(dead_code)]
fn test_data() -> Vec<Rc<str>> {
    return vec![
        "32T3K 765".into(),
        "T55J5 684".into(),
        "KK677 28".into(),
        "KTJJT 220".into(),
        "QQQJA 483".into(),
    ];
}
