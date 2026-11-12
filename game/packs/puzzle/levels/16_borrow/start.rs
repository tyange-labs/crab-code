// 퍼즐 16 · 빌림 & — 보기만 할게요
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin borrow

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    move_forward();
    let gem = collect();
    match color_of(gem) {
        Color::Red => turn_left(),
        Color::Blue => turn_right(),
    }
    move_forward();
    move_forward();
    deposit(gem);
}
