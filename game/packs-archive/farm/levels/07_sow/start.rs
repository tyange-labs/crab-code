// 해바라기 농장 7 · 씨 뿌리고 물 주고 — 튜플 match 와 Option 이어 붙이기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sow

use farm::level7::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        // 발밑이 빈 이랑이면 씨앗을 심어요. 그다음은 어떻게 할까요?
        match (view.here(), view.carrying()) {
            (Tile::Plot, _) => Action::Plant,
            _ => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
