// 퍼즐 8 · 정수 타입과 오버플로
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin int

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let n: u32 = read_dial();
    let code = n * 100;
    move_forward();
    unlock(code);
}
