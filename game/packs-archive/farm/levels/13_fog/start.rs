// 해바라기 농장 13 · 안개 속 농장 — HashMap 으로 지도 기억하기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin fog

use farm::level13::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Pip {
    seen: HashMap<(i32, i32), Tile>,
}

impl Pip {
    /// 주변을 훑어 기억을 고쳐 써요.
    fn look_around(&mut self, _view: &View) {
        // view.pos(), view.sight(), view.tile_at(x, y) 로 주변 칸을 보고
        // Ok(tile) 이면 self.seen.insert((x, y), tile) 로 기억하게 채워 보세요.
    }

    /// 가 볼 곳: 기억 속의 이랑·열매 중 가장 가까운 곳, 없으면 아직 못 본 가장 가까운 칸.
    fn target(&self, view: &View) -> Option<(i32, i32)> {
        let (px, py) = view.pos();
        let dist = |&(x, y): &(i32, i32)| (x - px).abs() + (y - py).abs();
        let work = self
            .seen
            .iter()
            .filter(|(&at, t)| at != (px, py) && matches!(**t, Tile::Plot | Tile::Seed | Tile::Crop))
            .map(|(&at, _)| at)
            .min_by_key(dist);
        if work.is_some() {
            return work;
        }
        let (w, h) = view.size();
        (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .filter(|at| !self.seen.contains_key(at))
            .min_by_key(dist)
    }
}

fn toward(view: &View, (x, y): (i32, i32)) -> Action {
    let (px, py) = view.pos();
    Action::Move(if x > px {
        Dir::East
    } else if x < px {
        Dir::West
    } else if y > py {
        Dir::South
    } else {
        Dir::North
    })
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip { seen: HashMap::new() }
    }

    fn tick(&mut self, view: &View) -> Action {
        self.look_around(view);
        match (view.here(), view.carrying()) {
            (Tile::Plot, _) => return Action::Plant,
            (Tile::Seed, _) => return Action::Water,
            (Tile::Crop, false) => return Action::Harvest,
            (Tile::Barn, true) => return Action::Store,
            _ => {}
        }
        if view.carrying() {
            return match view.barn_dir() {
                Some(d) => Action::Move(d),
                None => Action::Wait,
            };
        }
        match self.target(view) {
            Some(at) => toward(view, at),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
