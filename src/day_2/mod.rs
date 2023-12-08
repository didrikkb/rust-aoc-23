use std::rc::Rc;

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let games: Vec<Game> = parse_games(input);
    let mut res = 0;

    for game in games {
        let mut valid = true;

        for round in game.rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                valid = false;
            }
        }

        if valid {
            res += game.id;
        }
    }

    return res;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let games: Vec<Game> = parse_games(input);
    let mut res = 0;

    for game in games {
        let (mut reds, mut greens, mut blues) = (0, 0, 0);

        for round in game.rounds {
            reds = reds.max(round.red);
            greens = greens.max(round.green);
            blues = blues.max(round.blue);
        }

        res += reds * blues * greens;
    }

    return res;
}

fn parse_games(games: Vec<Rc<str>>) -> Vec<Game> {
    return games
        .iter()
        .enumerate()
        .filter_map(|(id, game)| match game.find(":") {
            Some(x) => {
                let rounds = &game[x + 1..];
                Some(Game {
                    id: id as i32 + 1,
                    rounds: parse_rounds(rounds),
                })
            }
            None => None,
        })
        .collect();
}

fn parse_rounds(rounds: &str) -> Vec<Round> {
    return rounds.split(";").map(|round| parse_round(round)).collect();
}

fn parse_round(round: &str) -> Round {
    return round.split(",").fold(
        Round {
            red: 0,
            blue: 0,
            green: 0,
        },
        |mut round, cube| {
            let num_and_color: Vec<&str> = cube.split_ascii_whitespace().collect();
            match num_and_color[1] {
                "blue" => round.blue += num_and_color[0].parse::<i32>().unwrap(),
                "green" => round.green += num_and_color[0].parse::<i32>().unwrap(),
                "red" => round.red += num_and_color[0].parse::<i32>().unwrap(),
                _ => {}
            }
            return round;
        },
    );
}

#[test]
fn part_1_test() {
    let data = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".into(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".into(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".into(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".into(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into(),
    ];
    assert_eq!(8, part_1(data));
}

#[test]
fn part_2_test() {
    let data = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".into(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".into(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".into(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".into(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".into(),
    ];
    assert_eq!(2286, part_2(data));
}
