// 해바라기 농장 7 · 씨 뿌리고 물 주고 — 튜플 match 와 Option 이어 붙이기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sow

use farm::level7::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Plot, _) => return Action::Plant,
            (Tile::Seed, _) => return Action::Water,
            (Tile::Crop, false) => return Action::Harvest,
            (Tile::Barn, true) => return Action::Store,
            _ => {}
        }
        let target = if view.carrying() {
            view.barn_dir()
        } else {
            view.plot_dir().or(view.crop_dir())
        };
        match target {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
