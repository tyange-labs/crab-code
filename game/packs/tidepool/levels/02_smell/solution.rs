// 조수 웅덩이 2 · 냄새를 따라 — Option 과 match
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin smell

use tidepool::prelude::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            match view.nest_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Drop,
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            match view.food_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}

fn main() {
    play!(Ferris);
}
