// 해바라기 농장 8 · 수확 철 — Vec 과 반복자
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin harvest

use farm::level8::*;

#[derive(Debug)]
struct Pip;

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            return match view.barn_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Store,
            };
        }
        if view.here() == Tile::Crop {
            return Action::Harvest;
        }
        let (px, py) = view.pos();
        let nearest = view
            .crops()
            .into_iter()
            .min_by_key(|&(x, y)| (x - px).abs() + (y - py).abs());
        match nearest {
            Some((x, y)) => Action::Move(if x > px {
                Dir::East
            } else if x < px {
                Dir::West
            } else if y > py {
                Dir::South
            } else {
                Dir::North
            }),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Pip>();
}
