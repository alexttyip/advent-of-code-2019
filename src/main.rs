use std::fs::File;
use std::io::Write;

use reqwest::{blocking, Error};
use reqwest::header::COOKIE;

use crate::solutions::{day01, day02, day03, day04, day05, day06, day07, day08};

mod solutions;

#[allow(unused)]
fn scrape_input() {
    const COOKIE_VALUE: &str = "session=53616c7465645f5f697cbf87ea19d83af146a2b572b5ef2a7374c290f79f8c615f9c359b56489a566bfdf7013caa22f5c1470bbc3b3089962565219fd4e1d7cc";

    fn send_request(day: &u8) -> Result<String, Error> {
        let url = format!("https://adventofcode.com/2019/day/{day}/input");

        let client = blocking::Client::new();
        let res = client.get(url)
            .header(COOKIE, COOKIE_VALUE)
            .send()?;
        res.text()
    }

    for day in 1..=25 {
        let path = format!("./inputs/day{day:02}.txt");

        let mut output = File::create(path).unwrap();
        write!(output, "{}", send_request(&day).unwrap()).unwrap();
    }
}

fn main() {
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    day06::run();
    day07::run();
    day08::run();
    // day09::run();
    // day10::run();
    // day11::run();
    // day12::run();
    // day13::run();
    // day14::run();
    // day15::run();
    // day16::run();
    // day17::run();
    // day18::run();
    // day19::run();
    // day20::run();
    // day21::run();
    // day22::run();
    // day23::run();
    // day24::run();
    // day25::run();
}
