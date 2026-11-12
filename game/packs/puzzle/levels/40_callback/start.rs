// 퍼즐 40 · 클로저를 받는 함수
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin callback

#[allow(unused_imports)]
use puzzle::prelude::*;

fn repeat(n: u32, f: impl Fn()) {
    // f 를 n번 불러 주세요

}

fn main() {
    repeat(3, || {
        repeat(3, || move_forward());
        collect();
        turn_right();
    });
}
