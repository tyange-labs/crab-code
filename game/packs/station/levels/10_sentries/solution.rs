// 정거장 복구 10 · 오작동 드론 — impl 로 코드 나누기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sentries

use station::level10::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,
    Deliver,
}

#[derive(Debug)]
struct Echo {
    job: Job,
}

impl Echo {
    /// 그쪽으로 한 걸음. 위험하면 기다려요.
    fn go(view: &View, dir: Option<Dir>) -> Action {
        match dir {
            Some(d) if view.danger(d) => Action::Wait,
            Some(d) => Action::Move(d),
            None => Action::Wait,
        }
    }
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
        Self::go(view, dir)
    }
}

fn main() {
    play::<Echo>();
}
