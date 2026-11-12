// 조수 웅덩이 1 · 꽃게의 두뇌, tick
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tick

use tidepool::prelude::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 매 틱 동쪽으로 걷기만 해요. 먹이를 만나면? 들고 나면?
        Action::Move(Dir::East)
    }
}

fn main() {
    play!(Ferris);
}
