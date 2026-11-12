// 퍼즐 28 · 스트럭트의 필드
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin fields

#[allow(unused_imports)]
use puzzle::level28::*;

fn main() {
    let mut total = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let info = inspect(&gem);
        println!("{:?}", info);
        total += info.weight;
    }
    move_forward();
    unlock(total);
}
