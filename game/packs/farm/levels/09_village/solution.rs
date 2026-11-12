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
            (Job::Tend, Tile::Plot, _) => return Action::Plant,
            (Job::Tend, Tile::Seed, _) => return Action::Water,
            (Job::Tend, Tile::Crop, false) => {
                self.job = Job::Deliver;
                return Action::Harvest;
            }
            (Job::Deliver, Tile::Barn, true) => {
                self.job = Job::Tend;
                return Action::Store;
            }
            _ => {}
        }
        let dir = match self.job {
            Job::Tend => view.crop_dir().or(view.plot_dir()),
            Job::Deliver => view.barn_dir(),
        };
        match dir {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
