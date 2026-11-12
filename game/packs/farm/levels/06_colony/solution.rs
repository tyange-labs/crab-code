// 해바라기 농장 6 · 농부 군단 — 같은 코드, 다른 self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin colony

use farm::level6::*;

#[derive(Debug)]
struct Pip {
    id: usize,
}

impl Farmer for Pip {
    fn new(id: usize) -> Self {
        Pip { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Crop {
            return Action::Harvest;
        }
        let target = if view.carrying() { view.barn_dir() } else { view.crop_dir() };
        match target {
            Some(dir) if view.danger(dir) => Action::Wait,
            Some(dir) => Action::Move(dir),
            None if view.carrying() => {
                println!("{}번 농부 배달!", self.id);
                Action::Store
            }
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
