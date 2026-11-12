// 정거장 복구 4 · 기억하는 로봇 — 필드와 &mut self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin maze

use station::level4::*;

#[derive(Debug)]
struct Echo {
    heading: Dir,
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo { heading: Dir::East }
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            return match view.reactor_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Insert,
            };
        }
        if view.here() == Tile::Core {
            return Action::Grab;
        }
        if let Some(dir) = view.core_dir() {
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
    play::<Echo>();
}
