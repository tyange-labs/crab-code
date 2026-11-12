// 퍼즐 4 · 나만의 함수 — fn
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin fn

#[allow(unused_imports)]
use puzzle::level4::*;

// 계단 한 칸: 앞으로 가서 보석을 줍고, 위로 한 칸 올라가 다시 오른쪽을 본다.
fn step() {
    move_forward();
    collect();
    turn_left();
    // 여기를 채워 주세요

}

fn main() {
    for _ in 0..4 {
        step();
    }
}
