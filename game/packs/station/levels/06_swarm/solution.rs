// 정거장 복구 6 · 로봇 군단 — 같은 코드, 다른 self
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin swarm

use station::level6::*;

#[derive(Debug)]
struct Echo {
    id: usize,
}

impl Bot for Echo {
    fn new(id: usize) -> Self {
        Echo { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Core {
            return Action::Grab;
        }
        let target = if view.carrying() { view.reactor_dir() } else { view.core_dir() };
        match target {
            Some(dir) if view.danger(dir) => Action::Wait,
            Some(dir) => Action::Move(dir),
            None if view.carrying() => {
                println!("{}번 로봇이 코어를 배달했어요!", self.id);
                Action::Insert
            }
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
