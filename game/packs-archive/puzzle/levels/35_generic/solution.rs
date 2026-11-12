// 퍼즐 35 · 제네릭과 트레이트 바운드
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin generic

#[allow(unused_imports)]
use puzzle::level35::*;

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

fn add<T: Value>(total: &mut u32, item: T) {
    *total += item.value();
}

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            add(&mut total, inspect(&gem));
        }
        if let Some(n) = read_number() {
            add(&mut total, n);
        }
    }
    unlock(total);
}
