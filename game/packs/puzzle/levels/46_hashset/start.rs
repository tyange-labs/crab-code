// 퍼즐 46 · HashSet 과 collect
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin hashset

#[allow(unused_imports)]
use puzzle::prelude::*;

use std::collections::HashSet;

fn main() {
    let v = read_scroll();
    // v 를 HashSet<u32> 로 모아서 len() 을 세어 보세요
    let kinds = v.len() as u32;
    move_forward();
    unlock(kinds);
}
