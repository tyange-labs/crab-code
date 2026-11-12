// 해바라기 농장 4 · 기억하는 농부 — 필드와 &mut self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin maze

use farm::level4::*;

#[derive(Debug)]
struct Pip {
    heading: Dir,
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip { heading: Dir::East }
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            return match view.barn_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Drop,
            };
        }
        if view.here() == Tile::Crop {
            return Action::Pick;
        }
        if let Some(dir) = view.crop_dir() {
            return Action::Move(dir);
        }
        // 오른손 법칙: 오른쪽, 앞, 왼쪽, 뒤 순서로 처음 열린 쪽.
        let h = self.heading;
        for dir in [h.turn_right(), h, h.turn_left(), h.opposite()] {
            if view.look(dir).is_open() {
                self.heading = dir;
                return Action::Move(dir);
            }
        }
        Action::Wait
    }
}

fn main() {
    play::<Pip>();
}
