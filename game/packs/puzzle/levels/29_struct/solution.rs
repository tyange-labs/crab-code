// 퍼즐 29 · 나만의 스트럭트
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin struct

#[allow(unused_imports)]
use puzzle::level29::*;

struct Counter {
    red: u32,
    blue: u32,
}

fn main() {
    let mut counter = Counter { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            match color_of(&gem) {
                Color::Red => counter.red += 1,
                Color::Blue => counter.blue += 1,
            }
        }
    }
    unlock(counter.red * 10 + counter.blue);
}
