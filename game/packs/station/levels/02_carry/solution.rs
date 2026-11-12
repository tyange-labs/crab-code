// 정거장 복구 2 · 코어 나르기 — if 와 Tile
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin carry

use station::level2::*;

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
