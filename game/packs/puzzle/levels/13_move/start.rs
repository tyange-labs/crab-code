// 퍼즐 13 · 소유권 — 주인은 하나
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin move

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    move_forward();
    let gem = collect();
    move_forward();
    // 두 번째 보석도 주워서 변수에 담아야 할 것 같은데...

    move_forward();
    move_forward();
    deposit(gem);
    move_forward();
    deposit(gem); // 같은 보석을 두 번 올릴 수 있을까요?
}
