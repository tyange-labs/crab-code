// 정거장 복구 5 · 진공 통로 — 데이터를 품은 enum
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin vacuum

use station::level5::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        // 기압을 보지 않고 곧장 건너요. 실행해서 무슨 일이 생기는지 보세요!
        if !view.carrying() && view.here() == Tile::Core {
            return Action::Grab;
        }
        let target = if view.carrying() { view.reactor_dir() } else { view.core_dir() };
        match target {
            Some(dir) => Action::Move(dir),
            None if view.carrying() => Action::Insert,
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
