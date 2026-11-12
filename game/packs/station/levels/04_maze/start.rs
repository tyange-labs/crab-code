// 정거장 복구 4 · 기억하는 로봇 — 필드와 &mut self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin maze

use station::level4::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            match view.reactor_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Insert,
            }
        } else if view.here() == Tile::Core {
            Action::Grab
        } else {
            match view.core_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}

fn main() {
    play::<Echo>();
}
