// 해바라기 농장 4 · 기억하는 농부 — 필드와 &mut self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin maze

use farm::level4::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            match view.barn_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Store,
            }
        } else if view.here() == Tile::Crop {
            Action::Harvest
        } else {
            match view.crop_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}

fn main() {
    play::<Pip>();
}
