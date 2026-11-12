// 퍼즐 4 · 나만의 함수 — fn
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin fn

#[allow(unused_imports)]
use puzzle::prelude::*;

fn step() {
    move_forward();
    collect();
    turn_left();
    move_forward();
    turn_right();
}

fn main() {
    for _ in 0..4 {
        step();
    }
}
