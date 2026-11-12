// 해바라기 농장 3 · 냄새를 따라 — Option 과 match
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin smell

use farm::level3::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            if view.here() == Tile::Barn {
                Action::Store
            } else {
                Action::Move(Dir::West)
            }
        } else if view.here() == Tile::Crop {
            Action::Harvest
        } else {
            Action::Move(Dir::East)
        }
    }
}

fn main() {
    play::<Pip>();
}
