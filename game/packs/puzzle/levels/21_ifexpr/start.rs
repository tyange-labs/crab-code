// 퍼즐 21 · if 는 값을 만든다
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin ifexpr

#[allow(unused_imports)]
use puzzle::level21::*;

fn main() {
    let mut count = 0;
    while front_is_clear() {
        move_forward();
        count += if on_gem() {
            collect();
            1
        };
    }
    unlock(count);
}
