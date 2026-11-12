// 퍼즐 17 · 가변 빌림 &mut — 광내기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin mutborrow

#[allow(unused_imports)]
use puzzle::level17::*;

fn main() {
    move_forward();
    let gem = collect();
    polish(&mut gem);
    move_forward();
    move_forward();
    move_forward();
    deposit(gem);
}
