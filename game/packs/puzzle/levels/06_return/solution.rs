// 퍼즐 6 · 반환값과 세미콜론
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin return

#[allow(unused_imports)]
use puzzle::level6::*;

fn square(x: u32) -> u32 {
    x * x
}

fn main() {
    let n = read_dial();
    move_forward();
    unlock(square(n));
}
