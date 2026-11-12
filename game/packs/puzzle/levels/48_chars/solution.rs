// 퍼즐 48 · 문자 char
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin chars

#[allow(unused_imports)]
use puzzle::level48::*;

fn main() {
    let note = read_note();
    let count = note.chars().filter(|c| *c == 'a').count() as u32;
    move_forward();
    unlock(count);
}
