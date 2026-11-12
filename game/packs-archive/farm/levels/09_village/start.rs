// 해바라기 농장 9 · 마을 농사 — 일을 enum 으로 나누기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin village

use farm::level9::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,
    Deliver,
}

#[derive(Debug)]
struct Pip {
    job: Job,
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip { job: Job::Tend }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (self.job, view.here(), view.carrying()) {
            (Job::Tend, Tile::Plot, _) => Action::Plant,
            // 물 주기, 거두기, 배달하기, 걸어가기를 이어서 짜 보세요.
            _ => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
