// 퍼즐 41 · 이터레이터 — iter 와 sum
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sum

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let v = read_scroll();
    let total: u32 = 0; // v.iter() 로 합을 구해 보세요
    move_forward();
    unlock(total);
}
