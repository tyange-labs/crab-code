// 해바라기 농장 12 · 정찰 — Result 와 ?
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin scout

use farm::level12::*;

#[derive(Debug)]
struct Pip;

/// 그 칸이 가 볼 만한가 (이랑이나 열매)? 안 보이면 Err.
fn worth_a_visit(view: &View, x: i32, y: i32) -> Result<bool, SightError> {
    let tile = view.tile_at(x, y)?;
    Ok(matches!(tile, Tile::Plot | Tile::Seed | Tile::Crop))
}

impl Pip {
    /// 보이는 칸 중 가 볼 만한 곳에서 가장 가까운 곳.
    fn target(view: &View) -> Option<(i32, i32)> {
        let (px, py) = view.pos();
        let (w, h) = view.size();
        let mut best: Option<((i32, i32), i32)> = None;
        for y in 0..h {
            for x in 0..w {
                if (x, y) == (px, py) || !worth_a_visit(view, x, y).unwrap_or(false) {
                    continue;
                }
                let dist = (x - px).abs() + (y - py).abs();
                if best.map_or(true, |(_, d)| dist < d) {
                    best = Some(((x, y), dist));
                }
            }
        }
        best.map(|(at, _)| at)
    }
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip
    }

    fn tick(&mut self, view: &View) -> Action {
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
        let (px, py) = view.pos();
        match Pip::target(view) {
            Some((x, y)) => Action::Move(if x > px {
                Dir::East
            } else if x < px {
                Dir::West
            } else if y > py {
                Dir::South
            } else {
                Dir::North
            }),
            None => {
                // 아직 아무것도 안 보이면 이랑이 있는 쪽(동쪽·남쪽)으로 조금 걸어가 봐요.
                if px < 3 { Action::Move(Dir::East) } else { Action::Wait }
            }
        }
    }
}

fn main() {
    play::<Pip>();
}
