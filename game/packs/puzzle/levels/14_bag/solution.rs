// 퍼즐 14 · 벡터로 옮겨 담기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin bag

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut bag = Vec::new();
    for _ in 0..3 {
        move_forward();
        bag.push(collect());
    }
    move_forward();
    for gem in bag {
        move_forward();
        deposit(gem);
    }
}
