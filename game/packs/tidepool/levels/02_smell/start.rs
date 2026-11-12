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
        // 지난 웅덩이의 코드예요. 먹이가 동쪽에만 있지는 않아요!
        if view.carrying() {
            if view.here() == Tile::Nest {
                Action::Drop
            } else {
                Action::Move(Dir::West)
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            Action::Move(Dir::East)
        }
    }
}

fn main() {
    play!(Ferris);
}
