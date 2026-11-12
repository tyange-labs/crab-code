// 정거장 복구 2 · 코어 나르기 — if 와 Tile
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin carry

use station::level2::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, _view: &View) -> Action {
        // 지난 레벨의 두뇌예요. 코어를 지나쳐 벽에 부딪히기만 해요.
        Action::Move(Dir::East)
    }
}

fn main() {
    play::<Echo>();
}
