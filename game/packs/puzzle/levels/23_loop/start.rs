// 퍼즐 23 · loop 와 break 값
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin loop

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let mut steps = 0;
    let total = loop {
        if !front_is_clear() {
            break 0; // 여기서 무엇을 들고 나와야 할까요?
        }
        move_forward();
        steps += 1;
    };
    turn_right();
    unlock(total);
}
