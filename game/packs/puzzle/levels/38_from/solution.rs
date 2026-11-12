// 퍼즐 38 · From 과 Into — 타입 변환
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin from

#[allow(unused_imports)]
use puzzle::prelude::*;

struct Digit(u32);

impl From<Color> for Digit {
    fn from(color: Color) -> Digit {
        match color {
            Color::Red => Digit(1),
            Color::Blue => Digit(2),
        }
    }
}

fn main() {
    let mut code = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let d: Digit = color_of(&gem).into();
        code = code * 10 + d.0;
    }
    move_forward();
    unlock(code);
}
