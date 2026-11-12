// 정거장 복구 8 · 회수 작전 — Vec 과 반복자
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin salvage

use station::level8::*;

#[derive(Debug)]
struct Echo;

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            return match view.reactor_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Insert,
            };
        }
        if view.here() == Tile::Core {
            return Action::Grab;
        }
        let (px, py) = view.pos();
        let nearest = view
            .cores()
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
    play::<Echo>();
}
