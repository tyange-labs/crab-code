// 퍼즐 30 · impl 로 메서드 정의하기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin impl

#[allow(unused_imports)]
use puzzle::prelude::*;

struct Backpack {
    count: u32,
    weight: u32,
}

impl Backpack {
    fn new() -> Backpack {
        Backpack { count: 0, weight: 0 }
    }

    // 발밑의 보석을 줍고 기록한다.
    fn pick(&self) {
        let gem = collect();
        self.count += 1;
        self.weight += inspect(&gem).weight;
    }
}

fn main() {
    let mut pack = Backpack::new();
    for _ in 0..3 {
        move_forward();
        pack.pick();
    }
    println!("보석 {}개, 무게 {}", pack.count, pack.weight);
    move_forward();
    unlock(pack.weight);
}
