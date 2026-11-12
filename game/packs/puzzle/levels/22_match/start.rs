// 퍼즐 22 · match — 빠짐없이 나누기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin match

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    loop {
        match read_sign() {
            Sign::Left => turn_left(),
            Sign::Finish => break,
        }
        move_forward();
    }
}
