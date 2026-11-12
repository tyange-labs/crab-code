// 조수 웅덩이 6 · 꽃게 군단 — 같은 코드, 다른 self
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin colony

use tidepool::level6::*;

#[derive(Debug)]
struct Ferris {
    id: usize,
}

impl Crab for Ferris {
    fn new(id: usize) -> Self {
        Ferris { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        // 갈매기를 신경 쓰지 않는 꽃게들이에요.
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        match target {
            Some(dir) => Action::Move(dir),
            None if view.carrying() => {
                println!("{}번 꽃게 배달!", self.id);
                Action::Drop
            }
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Ferris>();
}
