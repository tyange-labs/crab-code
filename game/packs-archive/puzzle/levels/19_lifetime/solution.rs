// 퍼즐 19 · 수명 — 레퍼런스는 얼마나 사는가
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin lifetime

#[allow(unused_imports)]
use puzzle::level19::*;

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let (a, b) = read_two_notes();
    let word = longer(&a, &b);
    move_forward();
    unlock(word.len() as u32);
}
