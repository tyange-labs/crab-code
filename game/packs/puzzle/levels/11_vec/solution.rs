// 퍼즐 11 · 벡터 Vec
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin vec

#[allow(unused_imports)]
use puzzle::level11::*;

fn main() {
    let v = read_scroll();
    println!("{:?}", v);
    let first = v[0];
    let last = v[v.len() - 1];
    move_forward();
    unlock(first + last);
}
