// 퍼즐 24 · Option — 있을 수도, 없을 수도
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin option

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        if let Some(n) = read_number() {
            sum += n;
        }
    }
    unlock(sum);
}
