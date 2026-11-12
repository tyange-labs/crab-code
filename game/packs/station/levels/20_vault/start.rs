// 정거장 복구 20 · 금고 — 데이터를 품은 enum 심화
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin vault

use station::level20::*;

#[derive(Debug)]
struct Echo;

/// 그쪽으로 한 걸음. 카드가 없는 잠긴 문 앞이면 부딪히지 않게 기다려요.
fn step(_view: &View, dir: Option<Dir>) -> Action {
    // view.look(d) 가 Tile::Door(hue) 이고 !view.keys().contains(&hue) 이면 Action::Wait,
    // 아니면 Action::Move(d) 가 되게 채워 보세요.
    match dir {
        Some(d) => Action::Move(d),
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
        // 지금은 코어로만 가요. 갈 수 없을 땐 키카드(card_dir)와 문(door_dir)도 쓰게 바꿔 보세요.
        step(view, view.core_dir())
    }
}

fn main() {
    play::<Echo>();
}
