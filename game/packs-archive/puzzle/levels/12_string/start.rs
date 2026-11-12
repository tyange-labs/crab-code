// 퍼즐 12 · 문자열과 usize
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin string

#[allow(unused_imports)]
use puzzle::level12::*;

fn main() {
    let note = read_note();
    move_forward();
    unlock(note.len());
}
