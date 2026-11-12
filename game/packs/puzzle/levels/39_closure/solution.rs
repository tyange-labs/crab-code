// 퍼즐 39 · 클로저 — 변수를 붙잡는 함수
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin closure

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut count = 0;
    let mut grab = || {
        move_forward();
        collect();
        count += 1;
    };
    grab();
    grab();
    grab();
    grab();
    move_forward();
    unlock(count);
}
