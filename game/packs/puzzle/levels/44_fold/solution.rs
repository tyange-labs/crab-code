// 퍼즐 44 · 범위와 fold
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin fold

#[allow(unused_imports)]
use puzzle::level44::*;

fn main() {
    let n = read_dial();
    let code = (1..=n).fold(1, |acc, x| acc * x);
    move_forward();
    unlock(code);
}
