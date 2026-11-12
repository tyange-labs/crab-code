// 퍼즐 37 · 내 타입을 Copy 로
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin copy

#[allow(unused_imports)]
use puzzle::level37::*;

#[derive(Clone, Copy)]
struct Leg {
    steps: u32,
}

fn walk(leg: Leg) {
    for _ in 0..leg.steps {
        move_forward();
    }
}

fn main() {
    let leg = Leg { steps: 3 };
    walk(leg);
    turn_right();
    walk(leg);
    collect();
}
