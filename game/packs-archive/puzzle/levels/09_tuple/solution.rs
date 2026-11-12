// 퍼즐 9 · 튜플 — 값 묶음
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tuple

#[allow(unused_imports)]
use puzzle::level9::*;

fn main() {
    let (ahead, right) = gem_offset();
    println!("앞으로 {}칸, 오른쪽으로 {}칸", ahead, right);
    for _ in 0..ahead {
        move_forward();
    }
    turn_right();
    for _ in 0..right {
        move_forward();
    }
    collect();
}
