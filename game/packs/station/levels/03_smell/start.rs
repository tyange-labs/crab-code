// 정거장 복구 3 · 신호를 따라 — Option 과 match
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin smell

use station::level3::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            if view.here() == Tile::Reactor {
                Action::Insert
            } else {
                Action::Move(Dir::West)
            }
        } else if view.here() == Tile::Core {
            Action::Grab
        } else {
            Action::Move(Dir::East)
        }
    }
}

fn main() {
    play::<Echo>();
}
