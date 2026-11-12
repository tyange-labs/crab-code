// 퍼즐 43 · enumerate 와 max_by_key
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin enumerate

#[allow(unused_imports)]
use puzzle::level43::*;

fn main() {
    let v = read_scroll();
    let idx = 0; // 가장 큰 숫자의 번호를 찾아야 해요
    for _ in 0..=idx {
        move_forward();
    }
    collect();
}
