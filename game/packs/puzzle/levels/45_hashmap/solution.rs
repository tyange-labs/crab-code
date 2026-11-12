// 퍼즐 45 · HashMap — 키로 찾기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin hashmap

#[allow(unused_imports)]
use puzzle::prelude::*;

use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    for n in read_scroll() {
        *counts.entry(n).or_insert(0) += 1;
    }
    println!("{:?}", counts);

    let mut best = 0;
    let mut best_count = 0;
    for (n, count) in &counts {
        if *count > best_count {
            best = *n;
            best_count = *count;
        }
    }
    move_forward();
    unlock(best);
}
