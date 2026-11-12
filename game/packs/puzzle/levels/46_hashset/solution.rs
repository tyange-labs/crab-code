// 퍼즐 46 · HashSet 과 collect
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin hashset

#[allow(unused_imports)]
use puzzle::prelude::*;

use std::collections::HashSet;

fn main() {
    let v = read_scroll();
    let set: HashSet<u32> = v.into_iter().collect();
    let kinds = set.len() as u32;
    move_forward();
    unlock(kinds);
}
