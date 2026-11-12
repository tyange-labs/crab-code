// 퍼즐 25 · 패닉 — 일어나면 안 되는 일
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin panic

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let v = read_scroll();
    let last = v[v.len() - 1];
    move_forward();
    unlock(last);
}
