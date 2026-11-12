// 퍼즐 9 · 튜플 — 값 묶음
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tuple

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let (ahead, right) = gem_offset();
    println!("앞으로 {}칸, 오른쪽으로 {}칸", ahead, right);
    // ahead 칸 걷고, 오른쪽으로 돌아서 right 칸 걸으세요

}
