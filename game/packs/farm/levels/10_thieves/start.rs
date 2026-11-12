// 해바라기 농장 10 · 도둑 고블린 — impl 로 코드 나누기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin thieves

use farm::level10::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,
    Deliver,
}

#[derive(Debug)]
struct Pip {
    job: Job,
}

impl Pip {
    /// 그쪽으로 한 걸음. 위험하면 기다려요.
    fn go(_view: &View, _dir: Option<Dir>) -> Action {
        // 위험한 쪽(view.danger)이면 기다리고, 아니면 걸어가게 채워 보세요.
        Action::Wait
    }
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
        Self::go(view, dir)
    }
}

fn main() {
    play::<Pip>();
}
