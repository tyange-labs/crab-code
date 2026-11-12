// 퍼즐 36 · 연산자 오버로딩 — Add
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin add

#[allow(unused_imports)]
use puzzle::prelude::*;

use std::ops::Add;

struct Score {
    red: u32,
    blue: u32,
}

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        Score {
            red: self.red + other.red,
            blue: self.blue + other.blue,
        }
    }
}

fn score_of(info: GemInfo) -> Score {
    match info.color {
        Color::Red => Score { red: info.weight, blue: 0 },
        Color::Blue => Score { red: 0, blue: info.weight },
    }
}

fn main() {
    let mut total = Score { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total = total + score_of(inspect(&gem));
        }
    }
    unlock(total.red * 100 + total.blue);
}
