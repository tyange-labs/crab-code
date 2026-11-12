// 정거장 복구 7 · 설치와 충전 — 튜플 match 와 Option 이어 붙이기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin install

use station::level7::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Slot, _) => return Action::Install,
            (Tile::Module, _) => return Action::Charge,
            (Tile::Core, false) => return Action::Grab,
            (Tile::Reactor, true) => return Action::Insert,
            _ => {}
        }
        let target = if view.carrying() {
            view.reactor_dir()
        } else {
            view.slot_dir().or(view.core_dir())
        };
        match target {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
