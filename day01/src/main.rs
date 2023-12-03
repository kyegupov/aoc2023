use std::{env::args, fs::read_to_string};

use anyhow::Result;
use itertools::Itertools;
use regex::{Match, Regex};

fn main() -> Result<()> {
    let arg = args().skip(1).next().unwrap();

    if arg == "a" {
        let mut res = 0u32;
        for line in read_to_string("input01.txt")?.lines() {
            let digits = line.chars().filter(|c| c.is_numeric()).collect_vec();
            res += [digits.first().unwrap(), digits.last().unwrap()]
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        }
        dbg!(res);
    }
    if arg == "b" {
        let rx = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine")?;
        let mut res = 0u32;
        let parse = |x: Match| match x.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            x => x.parse::<u32>().unwrap(),
        };
        for line in read_to_string("input01.txt")?.lines() {
            let first = parse(rx.find(line).unwrap());
            let last = parse(
                (0..line.len())
                    .rev()
                    .find_map(|i| rx.find(&line[i..]))
                    .unwrap(),
            );
            let num = first * 10 + last;
            res += num;
        }
        dbg!(res);
    }

    Ok(())
}
