// 조수 웅덩이 5 · 밀물과 썰물 — 데이터를 품은 enum
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tide

use tidepool::level5::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 물때를 보지 않고 곧장 건너요. 실행해서 무슨 일이 생기는지 보세요!
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        match target {
            Some(dir) => Action::Move(dir),
            None if view.carrying() => Action::Drop,
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Ferris>();
}
