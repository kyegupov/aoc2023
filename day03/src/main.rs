use std::{collections::BTreeMap, env::args, fs::read_to_string};

use anyhow::Result;

use board2d::*;

fn main() -> Result<()> {
    let arg = args().skip(1).next().unwrap();

    if arg == "a" {
        let mut res = 0usize;
        let board = chars_board_from(&mut read_to_string("input03.txt")?.lines());

        for (&y, row) in &board.0 {
            let mut accum = 0;
            let mut sym = false;
            for (&x, c) in row {
                let xy = XY(x, y);
                if c.is_numeric() {
                    for n in xy.neighbors8() {
                        match board.get(n) {
                            Some(cc) if cc.is_numeric() => {}
                            Some('.') => {}
                            None => {}
                            _ => {
                                sym = true;
                            }
                        }
                    }

                    accum = accum * 10 + char::to_digit(*c, 10).unwrap() as usize;
                } else {
                    if sym {
                        res += accum;
                    }
                    sym = false;
                    accum = 0;
                }
            }
            if sym {
                res += accum;
            }
        }
        dbg!(res);
    }

    if arg == "b" {
        let mut res = 0usize;
        let board = chars_board_from(&mut read_to_string("input03.txt")?.lines());

        let mut gears = BTreeMap::new();

        for (&y, row) in &board.0 {
            let mut accum = 0;
            let mut asterisk = None;
            for (&x, c) in row {
                let xy = XY(x, y);
                if c.is_numeric() {
                    for n in xy.neighbors8() {
                        match board.get(n) {
                            Some('*') => {
                                asterisk = Some(n);
                            }
                            _ => {}
                        }
                    }

                    accum = accum * 10 + char::to_digit(*c, 10).unwrap() as usize;
                } else {
                    if asterisk.is_some() && accum > 0 {
                        dbg!(asterisk, gears.get(&asterisk.unwrap()));
                        if let Some(other) = gears.get(&asterisk.unwrap()) {
                            res += accum * other;
                        } else {
                            gears.insert(asterisk.unwrap(), accum);
                        }
                    }
                    asterisk = None;
                    accum = 0;
                }
            }
            if asterisk.is_some() && accum > 0 {
                if let Some(other) = gears.get(&asterisk.unwrap()) {
                    res += accum * other;
                } else {
                    gears.insert(asterisk.unwrap(), accum);
                }
            }
        }
        dbg!(res);
    }

    Ok(())
}
