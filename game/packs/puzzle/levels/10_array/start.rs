// 퍼즐 10 · 배열
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin array

#[allow(unused_imports)]
use puzzle::level10::*;

fn main() {
    let legs = [5, 3];
    for n in legs {
        for _ in 0..n {
            move_forward();
        }
        turn_right();
    }
    collect();
}
