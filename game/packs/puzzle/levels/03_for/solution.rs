// 퍼즐 3 · 반복은 for 에게
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin for

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    for i in 0..3 {
        move_forward();
        move_forward();
        collect();
        println!("{}번째 보석!", i + 1);
    }
}
