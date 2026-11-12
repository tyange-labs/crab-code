// 정거장 복구 5 · 진공 통로 — 데이터를 품은 enum
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin vacuum

use station::level5::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Core {
            return Action::Grab;
        }
        let target = if view.carrying() { view.reactor_dir() } else { view.core_dir() };
        let Some(dir) = target else {
            return if view.carrying() { Action::Insert } else { Action::Wait };
        };
        // 통로에 처음 발을 들이기 전에 기압을 본다.
        let entering = view.here() != Tile::Duct && matches!(view.look(dir), Tile::Duct | Tile::Void);
        if entering {
            match view.pressure() {
                Pressure::Safe { turns_left } if turns_left >= 6 => Action::Move(dir),
                _ => Action::Wait,
            }
        } else {
            Action::Move(dir)
        }
    }
}

fn main() {
    play::<Echo>();
}
