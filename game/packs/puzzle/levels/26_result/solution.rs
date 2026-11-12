// 퍼즐 26 · Result — 실패할 수 있는 일
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin result

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let note = read_note();
    move_forward();
    match note.parse::<u32>() {
        Ok(n) => unlock(n),
        Err(_) => unlock(0),
    }
}
