// 조수 웅덩이 2 · 먹이 나르기 — if 와 Tile
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin carry

use tidepool::level2::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, _view: &View) -> Action {
        // 지난 웅덩이의 두뇌예요. 먹이를 지나쳐 벽에 부딪히기만 해요.
        Action::Move(Dir::East)
    }
}

fn main() {
    play::<Ferris>();
}
