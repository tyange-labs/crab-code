// 퍼즐 32 · 데이터를 품은 이늄
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin enumdata

#[allow(unused_imports)]
use puzzle::level32::*;

enum Move {
    Forward(u32),
    Left,
    Right,
}

fn run(m: Move) {
    match m {
        Move::Left => turn_left(),
        Move::Right => turn_right(),
    }
}

fn main() {
    let plan = vec![
        Move::Forward(2),
        Move::Left,
        Move::Forward(3),
        Move::Right,
        Move::Forward(3),
    ];
    for m in plan {
        run(m);
    }
    collect();
}
