// 해바라기 농장 5 · 홍수와 물 빠짐 — 데이터를 품은 enum
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin flood

use farm::level5::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        // 수위를 보지 않고 곧장 건너요. 실행해서 무슨 일이 생기는지 보세요!
        if !view.carrying() && view.here() == Tile::Crop {
            return Action::Pick;
        }
        let target = if view.carrying() { view.barn_dir() } else { view.crop_dir() };
        match target {
            Some(dir) => Action::Move(dir),
            None if view.carrying() => Action::Drop,
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
