// 해바라기 농장 2 · 열매 나르기 — if 와 Tile
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin carry

use farm::level2::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, _view: &View) -> Action {
        // 지난 밭의 두뇌예요. 열매를 지나쳐 벽에 부딪히기만 해요.
        Action::Move(Dir::East)
    }
}

fn main() {
    play::<Pip>();
}
