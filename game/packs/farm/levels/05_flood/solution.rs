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
        if !view.carrying() && view.here() == Tile::Crop {
            return Action::Pick;
        }
        let target = if view.carrying() { view.barn_dir() } else { view.crop_dir() };
        let Some(dir) = target else {
            return if view.carrying() { Action::Drop } else { Action::Wait };
        };
        // 논에 처음 발을 들이기 전에 수위를 본다.
        let entering = view.here() != Tile::Paddy && matches!(view.look(dir), Tile::Paddy | Tile::Water);
        if entering {
            match view.flood() {
                Flood::Low { turns_left } if turns_left >= 6 => Action::Move(dir),
                _ => Action::Wait,
            }
        } else {
            Action::Move(dir)
        }
    }
}

fn main() {
    play::<Pip>();
}
