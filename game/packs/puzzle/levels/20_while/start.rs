// 퍼즐 20 · while 과 if — 조건
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin while

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    while front_is_clear() {
        move_forward();
        // 보석이 있을 때만 주워야 해요.
        // 없는데 collect() 하면 프로그램이 멈춰요!

    }
}
