// 퍼즐 18 · 공유 vs. 변경
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin shared

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut bag = Vec::new();
    move_forward();
    bag.push(collect());
    let first = &bag[0];
    println!("첫 보석 색: {:?}", color_of(first));
    move_forward();
    bag.push(collect());
    move_forward();
    for gem in bag {
        move_forward();
        deposit(gem);
    }
}
