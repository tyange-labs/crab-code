// 퍼즐 34 · 트레이트 — 공통된 능력
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin trait

#[allow(unused_imports)]
use puzzle::level34::*;

trait Value {
    fn value(&self) -> u32;
}

impl Value for GemInfo {
    fn value(&self) -> u32 {
        self.weight * 10
    }
}

impl Value for u32 {
    fn value(&self) -> u32 {
        *self
    }
}

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total += inspect(&gem).value();
        }
        if let Some(n) = read_number() {
            total += n.value();
        }
    }
    unlock(total);
}
