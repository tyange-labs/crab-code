// 퍼즐 27 · ? 연산자 — 오류 떠넘기기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin question

#[allow(unused_imports)]
use puzzle::level27::*;

use std::num::ParseIntError;

fn sum(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = a.parse::<u32>();
    let y = b.parse::<u32>();
    Ok(x + y)
}

fn main() {
    let (a, b) = read_two_notes();
    move_forward();
    match sum(&a, &b) {
        Ok(n) => unlock(n),
        Err(e) => {
            println!("읽기 실패: {}", e);
            unlock(0);
        }
    }
}
