// 조수 웅덩이 1 · 꽃게의 두뇌, tick
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tick

use tidepool::level1::*;

#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, _view: &View) -> Action {
        Action::Move(Dir::East)
    }
}

fn main() {
    play::<Ferris>();
}
