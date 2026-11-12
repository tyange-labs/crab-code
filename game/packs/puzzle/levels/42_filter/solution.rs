// 퍼즐 42 · 어댑터 — filter 와 map
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin filter

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let v = read_scroll();
    let big = v.iter().filter(|x| **x > 50).count() as u32;
    move_forward();
    unlock(big);
}
