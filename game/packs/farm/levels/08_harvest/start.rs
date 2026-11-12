// 해바라기 농장 8 · 수확 철 — Vec 과 반복자
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin harvest

use farm::level8::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.here() == Tile::Crop && !view.carrying() {
            return Action::Harvest;
        }
        // 가장 가까운 열매를 골라 그쪽으로 가 보세요. view.crops() 가 열매 위치 목록이에요.
        Action::Wait
    }
}

fn main() {
    play::<Pip>();
}
