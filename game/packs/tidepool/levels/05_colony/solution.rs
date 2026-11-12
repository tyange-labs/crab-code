// 조수 웅덩이 5 · 꽃게 군단 — 같은 코드, 다른 self
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin colony

use tidepool::prelude::*;

#[derive(Debug)]
struct Ferris {
    id: usize,
}

impl Crab for Ferris {
    fn new(id: usize) -> Self {
        Ferris { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        match target {
            Some(dir) if view.danger(dir) => Action::Wait,
            Some(dir) => Action::Move(dir),
            None if view.carrying() => {
                println!("{}번 꽃게 배달!", self.id);
                Action::Drop
            }
            None => Action::Wait,
        }
    }
}

fn main() {
    play!(Ferris);
}
