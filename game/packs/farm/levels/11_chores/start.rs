// 해바라기 농장 11 · 일 목록 — trait 과 Box<dyn>
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin chores

use farm::level11::*;

/// 지금 할 수 있는 일이면 `Some(행동)`, 아니면 `None`.
trait Chore: std::fmt::Debug {
    fn try_do(&self, view: &View) -> Option<Action>;
}

#[derive(Debug)]
struct Plant;
impl Chore for Plant {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Plot).then_some(Action::Plant)
    }
}

#[derive(Debug)]
struct Water;
impl Chore for Water {
    fn try_do(&self, _view: &View) -> Option<Action> {
        // 발밑이 Tile::Seed 일 때만 Some(Action::Water) 를 돌려주게 채워 보세요.
        None
    }
}

#[derive(Debug)]
struct Harvest;
impl Chore for Harvest {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Crop && !view.carrying()).then_some(Action::Harvest)
    }
}

#[derive(Debug)]
struct Store;
impl Chore for Store {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Barn && view.carrying()).then_some(Action::Store)
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
struct Pip {
    chores: Vec<Box<dyn Chore>>,
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip {
            chores: vec![Box::new(Store), Box::new(Harvest), Box::new(Plant), Box::new(Water), Box::new(Walk)],
        }
    }

    fn tick(&mut self, view: &View) -> Action {
        self.chores.iter().find_map(|c| c.try_do(view)).unwrap_or(Action::Wait)
    }
}

fn main() {
    play::<Pip>();
}
