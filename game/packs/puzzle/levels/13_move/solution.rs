// 퍼즐 13 · 소유권 — 주인은 하나
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin move

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    move_forward();
    let gem = collect();
    move_forward();
    let gem2 = collect();
    move_forward();
    move_forward();
    deposit(gem);
    move_forward();
    deposit(gem2);
}
