// 정거장 복구 7 · 설치와 충전 — 튜플 match 와 Option 이어 붙이기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin install

use station::level7::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        // 발밑이 빈 슬롯이면 모듈을 설치해요. 그다음은 어떻게 할까요?
        match (view.here(), view.carrying()) {
            (Tile::Slot, _) => Action::Install,
            _ => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
