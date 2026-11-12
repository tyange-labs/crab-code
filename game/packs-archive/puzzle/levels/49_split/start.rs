// 퍼즐 49 · 쪼개고 변환하기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin split

#[allow(unused_imports)]
use puzzle::level49::*;

fn main() {
    let note = read_note();
    let mut total = 0;
    for word in note.split_whitespace() {
        println!("조각: {}", word);
        // word 를 숫자로 바꿔 total 에 더하세요
    }
    move_forward();
    unlock(total);
}
