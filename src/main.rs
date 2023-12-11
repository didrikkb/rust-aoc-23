use std::{
    io::{self, BufRead},
    rc::Rc,
};
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
//  { echo "d1p1"; cat day_1/data.txt; } | cargo run

fn main() {
    let input = io::stdin().lock().lines();
    let mut input: Vec<Rc<str>> = input.into_iter().map(|line| line.unwrap().into()).collect();

    let res = match input.pop().as_deref() {
        Some("d1p1") => Some(day_1::part_1(input)),
        Some("d1p2") => Some(day_1::part_2(input)),
        Some("d2p1") => Some(day_2::part_1(input)),
        Some("d2p2") => Some(day_2::part_2(input)),
        Some("d3p1") => Some(day_3::part_1(input)),
        Some("d3p2") => Some(day_3::part_2(input)),
        Some("d4p1") => Some(day_4::part_1(input)),
        Some("d4p2") => Some(day_4::part_2(input)),
        Some("d5p1") => Some(day_5::part_1(input)),
        Some("d5p2") => Some(day_5::part_2(input)),
        Some("d6p1") => Some(day_6::part_1(input)),
        Some("d6p2") => Some(day_6::part_2(input)),
        Some("d7p1") => Some(day_7::part_1(input)),
        Some("d7p2") => Some(day_7::part_2(input)),
        Some("d8p1") => Some(day_8::part_1(input)),
        Some("d8p2") => Some(day_8::part_2(input)),
        Some("d9p1") => Some(day_9::part_1(input)),
        Some("d9p2") => Some(day_9::part_2(input)),

        _ => {
            println!("Wuut?");
            None
        }
    };

    match res {
        Some(val) => println!("{}", val),
        None => println!("Error"),
    }
}
