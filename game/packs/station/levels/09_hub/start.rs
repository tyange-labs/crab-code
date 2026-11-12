// 정거장 복구 9 · 허브 가동 — 일을 enum 으로 나누기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin hub

use station::level9::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,
    Deliver,
}

#[derive(Debug)]
struct Echo {
    job: Job,
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo { job: Job::Tend }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (self.job, view.here(), view.carrying()) {
            (Job::Tend, Tile::Slot, _) => Action::Install,
            // 충전하기, 집기, 배달하기, 걸어가기를 이어서 짜 보세요.
            _ => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
