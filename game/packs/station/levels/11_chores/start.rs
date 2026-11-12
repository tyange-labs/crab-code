// 정거장 복구 11 · 작업 목록 — trait 과 Box<dyn>
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin chores

use station::level11::*;

/// 지금 할 수 있는 일이면 `Some(행동)`, 아니면 `None`.
trait Chore: std::fmt::Debug {
    fn try_do(&self, view: &View) -> Option<Action>;
}

#[derive(Debug)]
struct Install;
impl Chore for Install {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Slot).then_some(Action::Install)
    }
}

#[derive(Debug)]
struct Charge;
impl Chore for Charge {
    fn try_do(&self, _view: &View) -> Option<Action> {
        // 발밑이 Tile::Module 일 때만 Some(Action::Charge) 를 돌려주게 채워 보세요.
        None
    }
}

#[derive(Debug)]
struct Grab;
impl Chore for Grab {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Core && !view.carrying()).then_some(Action::Grab)
    }
}

#[derive(Debug)]
struct Insert;
impl Chore for Insert {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Reactor && view.carrying()).then_some(Action::Insert)
    }
}

#[derive(Debug)]
struct Walk;
impl Chore for Walk {
    fn try_do(&self, _view: &View) -> Option<Action> {
        // 걸어갈 방향을 골라 Some(Action::Move(dir)) 를 돌려주게 채워 보세요. 위험한 쪽이면 None.
        None
    }
}

#[derive(Debug)]
struct Echo {
    chores: Vec<Box<dyn Chore>>,
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo {
            chores: vec![Box::new(Insert), Box::new(Grab), Box::new(Install), Box::new(Charge), Box::new(Walk)],
        }
    }

    fn tick(&mut self, view: &View) -> Action {
        self.chores.iter().find_map(|c| c.try_do(view)).unwrap_or(Action::Wait)
    }
}

fn main() {
    play::<Echo>();
}
