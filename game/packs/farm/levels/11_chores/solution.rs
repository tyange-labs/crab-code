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
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Seed).then_some(Action::Water)
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
    fn try_do(&self, view: &View) -> Option<Action> {
        let dir = if view.carrying() {
            view.barn_dir()
        } else {
            view.crop_dir().or(view.plot_dir())
        };
        match dir {
            Some(d) if view.danger(d) => None,
            Some(d) => Some(Action::Move(d)),
            None => None,
        }
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
