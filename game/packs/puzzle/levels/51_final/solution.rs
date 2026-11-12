// 퍼즐 51 · 졸업 시험 — 소용돌이 미로
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin final

#[allow(unused_imports)]
use puzzle::level51::*;

fn main() {
    while read_sign() != Sign::Finish {
        if on_gem() {
            collect();
        }
        if front_is_clear() {
            move_forward();
        } else {
            turn_left();
        }
    }
}
