// 퍼즐 33 · 패턴 — 가드와 범위
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin pattern

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        match read_number() {
            Some(n) if n % 2 == 0 => sum += n,
            _ => {}
        }
    }
    unlock(sum);
}
