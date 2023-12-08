use std::rc::Rc;

#[derive(Debug)]
struct PartNumber {
    value: i64,
    y: i32,
    x0: i32,
    x1: i32,
}

#[derive(Debug)]
struct Symbol {
    y: i32,
    x: i32,
    sym: u8,
}

pub fn part_1(input: Vec<Rc<str>>) -> i32 {
    let part_nums = find_parts(input.clone());
    let symbols = find_symbols(input);
    let dirs = search_directions();
    let mut sum = 0;

    for symbol in symbols {
        for part in &part_nums {
            if (symbol.y - part.y).abs() > 1 {
                continue;
            }
            for (y, x) in dirs {
                if symbol.y + y == part.y && symbol.x + x >= part.x0 && symbol.x + x <= part.x1 {
                    sum += part.value;
                    break;
                }
            }
        }
    }

    return sum as i32;
}

pub fn part_2(input: Vec<Rc<str>>) -> i32 {
    let part_nums = find_parts(input.clone());
    let symbols: Vec<Symbol> = find_symbols(input)
        .into_iter()
        .filter(|symbol| symbol.sym == b'*')
        .collect();
    let dirs = search_directions();
    let mut sum = 0;

    for symbol in symbols {
        let mut count = 0;
        let mut temp_sum = 1;

        for part in &part_nums {
            if (symbol.y - part.y).abs() > 1 {
                continue;
            }
            for (y, x) in dirs {
                if symbol.y + y == part.y && symbol.x + x >= part.x0 && symbol.x + x <= part.x1 {
                    temp_sum *= part.value;
                    count += 1;
                    break;
                }
            }
        }

        if count == 2 {
            sum += temp_sum;
        }
    }

    return sum as i32;
}

fn find_parts(input: Vec<Rc<str>>) -> Vec<PartNumber> {
    return input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, &num)| match num {
                    b'0'..=b'9' => Some((x as i32, y as i32, num - b'0')),
                    _ => None,
                })
        })
        .fold(Vec::new(), |mut v: Vec<PartNumber>, (x, y, num)| {
            match v.last_mut() {
                Some(prev) if prev.x1 + 1 == x => {
                    prev.x1 += 1;
                    prev.value = prev.value * 10 + num as i64;
                }
                _ => {
                    v.push(PartNumber {
                        value: num as i64,
                        y: y,
                        x0: x,
                        x1: x,
                    });
                }
            }

            return v;
        });
}

fn find_symbols(input: Vec<Rc<str>>) -> Vec<Symbol> {
    return input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, &sym)| {
                    if sym != b'.' && (sym < b'0' || sym > b'9') {
                        return Some(Symbol {
                            x: x as i32,
                            y: y as i32,
                            sym: sym,
                        });
                    }
                    return None;
                })
        })
        .collect::<Vec<Symbol>>();
}

fn search_directions() -> [(i32, i32); 8] {
    return [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];
}

#[test]
fn part_1_test() {
    let data: Vec<Rc<str>> = vec![
        "467..114..".into(),
        "...*......".into(),
        "..35..633.".into(),
        "......#...".into(),
        "617*......".into(),
        ".....+.58.".into(),
        "..592.....".into(),
        "......755.".into(),
        "...$.*....".into(),
        ".664.598..".into(),
    ];
    assert_eq!(part_1(data), 4361);
}

#[test]
fn part_2_test() {
    let data: Vec<Rc<str>> = vec![
        "467..114..".into(),
        "...*......".into(),
        "..35..633.".into(),
        "......#...".into(),
        "617*......".into(),
        ".....+.58.".into(),
        "..592.....".into(),
        "......755.".into(),
        "...$.*....".into(),
        ".664.598..".into(),
    ];
    assert_eq!(part_2(data), 467835);
}
