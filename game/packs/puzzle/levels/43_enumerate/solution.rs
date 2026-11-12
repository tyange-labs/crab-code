// 퍼즐 43 · enumerate 와 max_by_key
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin enumerate

#[allow(unused_imports)]
use puzzle::level43::*;

fn main() {
    let v = read_scroll();
    let (idx, _) = v.iter().enumerate().max_by_key(|(_, n)| **n).unwrap();
    for _ in 0..=idx {
        move_forward();
    }
    collect();
}
