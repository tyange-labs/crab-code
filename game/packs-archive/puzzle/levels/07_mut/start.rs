// 퍼즐 7 · 컴파일러는 친구 — let mut
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin mut

#[allow(unused_imports)]
use puzzle::level7::*;

fn main() {
    let count = 0;
    for _ in 0..4 {
        move_forward();
        collect();
        count += 1;
    }
    move_forward();
    unlock(count);
}
