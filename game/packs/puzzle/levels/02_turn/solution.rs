// 퍼즐 2 · 모퉁이 돌기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin turn

#[allow(unused_imports)]
use puzzle::level2::*;

fn main() {
    move_forward();
    move_forward();
    turn_right();
    move_forward();
    move_forward();
    collect();
}
