// 조수 웅덩이 4 · 밀물과 썰물 — 데이터를 품은 enum
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tide

use tidepool::prelude::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        let Some(dir) = target else {
            return if view.carrying() { Action::Drop } else { Action::Wait };
        };
        // 갯벌에 처음 발을 들이기 전에 물때를 본다.
        let entering = view.here() != Tile::Flat && matches!(view.look(dir), Tile::Flat | Tile::Water);
        if entering {
            match view.tide() {
                Tide::Low { turns_left } if turns_left >= 6 => Action::Move(dir),
                _ => Action::Wait,
            }
        } else {
            Action::Move(dir)
        }
    }
}

fn main() {
    play!(Ferris);
}
