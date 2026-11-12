// 해바라기 농장 1 · 농부의 두뇌, tick
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tick

use farm::level1::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, _view: &View) -> Action {
        Action::Move(Dir::East)
    }
}

fn main() {
    play::<Pip>();
}
