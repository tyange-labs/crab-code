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
            (Job::Tend, Tile::Slot, _) => return Action::Install,
            (Job::Tend, Tile::Module, _) => return Action::Charge,
            (Job::Tend, Tile::Core, false) => {
                self.job = Job::Deliver;
                return Action::Grab;
            }
            (Job::Deliver, Tile::Reactor, true) => {
                self.job = Job::Tend;
                return Action::Insert;
            }
            _ => {}
        }
        let dir = match self.job {
            Job::Tend => view.core_dir().or(view.slot_dir()),
            Job::Deliver => view.reactor_dir(),
        };
        match dir {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
