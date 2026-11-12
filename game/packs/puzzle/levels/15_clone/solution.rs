// 퍼즐 15 · Copy 와 clone — 이동의 예외
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin clone

#[allow(unused_imports)]
use puzzle::prelude::*;

fn main() {
    let note = read_note();
    let backup = note.clone();
    println!("쪽지: {}", note);
    move_forward();
    unlock(backup.len() as u32);
}
