// 퍼즐 31 · #[derive] — 공통 기능 얻기
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin derive

#[allow(unused_imports)]
use puzzle::prelude::*;

#[derive(Debug)]
struct Report {
    steps: u32,
    found: bool,
}

fn main() {
    let mut report = Report { steps: 0, found: false };
    for _ in 0..3 {
        move_forward();
        report.steps += 1;
    }
    collect();
    report.found = true;
    println!("{:?}", report);
}
