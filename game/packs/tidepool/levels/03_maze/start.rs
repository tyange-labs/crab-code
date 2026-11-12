// 조수 웅덩이 3 · 기억하는 꽃게 — 필드와 &mut self
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin maze

use tidepool::prelude::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 냄새가 안 닿으면 food_dir() 은 None 이라 계속 기다리기만 해요.
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
