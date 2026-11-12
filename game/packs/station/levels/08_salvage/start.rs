// 정거장 복구 8 · 회수 작전 — Vec 과 반복자
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin salvage

use station::level8::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.here() == Tile::Core && !view.carrying() {
            return Action::Grab;
        }
        // 가장 가까운 코어를 골라 그쪽으로 가 보세요. view.cores() 가 코어 위치 목록이에요.
        Action::Wait
    }
}

fn main() {
    play::<Echo>();
}
