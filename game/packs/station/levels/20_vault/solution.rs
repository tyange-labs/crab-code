// 정거장 복구 20 · 금고 — 데이터를 품은 enum 심화
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin vault

use station::level20::*;

#[derive(Debug)]
struct Echo;

/// 그쪽으로 한 걸음. 카드가 없는 잠긴 문 앞이면 부딪히지 않게 기다려요.
fn step(view: &View, dir: Option<Dir>) -> Action {
    match dir {
        Some(d) => match view.look(d) {
            Tile::Door(hue) if !view.keys().contains(&hue) => Action::Wait,
            _ => Action::Move(d),
        },
        None => Action::Wait,
    }
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Reactor, true) => return Action::Insert,
            (Tile::Core, false) => return Action::Grab,
            _ => {}
        }
        if view.carrying() {
            return step(view, view.reactor_dir());
        }
        // 코어 → 키카드 → 열 수 있는 문 순서로
        step(view, view.core_dir().or(view.card_dir()).or(view.door_dir()))
    }
}

fn main() {
    play::<Echo>();
}
