use std::{cmp::max, env::args, fs::read_to_string};

use anyhow::Result;

fn main() -> Result<()> {
    let arg = args().skip(1).next().unwrap();

    if arg == "a" {
        let mut res = 0usize;
        for (i, line) in read_to_string("input02.txt")?.lines().enumerate() {
            let mut ok = true;
            for game in line.split_once(": ").unwrap().1.split("; ") {
                for stmt in game.split(", ") {
                    let (num, color) = stmt.split_once(" ").unwrap();
                    let n = num.parse::<usize>().unwrap();
                    ok &= match color {
                        "red" => n <= 12,
                        "green" => n <= 13,
                        "blue" => n <= 14,
                        _ => unimplemented!(),
                    };
                }
            }
            if ok {
                res += i + 1;
            }
        }
        dbg!(res);
    }

    if arg == "b" {
        let mut res = 0usize;
        for (i, line) in read_to_string("input02.txt")?.lines().enumerate() {
            let mut rgb = [0, 0, 0];
            for game in line.split_once(": ").unwrap().1.split("; ") {
                for stmt in game.split(", ") {
                    let (num, color) = stmt.split_once(" ").unwrap();
                    let n = num.parse::<usize>().unwrap();
                    match color {
                        "red" => rgb[0] = max(rgb[0], n),
                        "green" => rgb[1] = max(rgb[1], n),
                        "blue" => rgb[2] = max(rgb[2], n),
                        _ => unimplemented!(),
                    };
                }
            }
            res += rgb.iter().product::<usize>();
        }
        dbg!(res);
    }
    Ok(())
}
