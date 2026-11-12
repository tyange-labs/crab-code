// 퍼즐 5 · 매개변수 — 값을 받는 함수
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin param

#[allow(unused_imports)]
use puzzle::level5::*;

fn walk(n: u32) {
    for _ in 0..n {
        move_forward();
    }
}

fn main() {
    walk(3);
    turn_right();
    walk(4);
    turn_left();
    walk(2);
    collect();
}
