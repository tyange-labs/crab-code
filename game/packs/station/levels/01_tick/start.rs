// 정거장 복구 1 · 로봇의 두뇌, tick
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin tick

use station::level1::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, _view: &View) -> Action {
        // 가만히 있으면 코어에 닿을 수 없어요. 동쪽으로 걸어 볼까요?
        Action::Wait
    }
}

fn main() {
    play::<Echo>();
}
