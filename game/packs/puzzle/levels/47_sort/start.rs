// 퍼즐 47 · 정렬
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sort

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let v = read_scroll();
    v.sort();
    println!("{:?}", v);
    move_forward();
    unlock(v[v.len() / 2]);
}
