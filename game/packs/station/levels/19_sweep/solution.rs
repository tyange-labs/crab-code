// 정거장 복구 19 · 순찰 — Iterator 를 직접 만들기
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin sweep

use station::level19::*;

/// 순찰 지점을 끝없이 돌려 내놓는 반복자.
#[derive(Debug)]
struct Sweep {
    points: Vec<(i32, i32)>,
    next_index: usize,
}

impl Sweep {
    fn zigzag() -> Sweep {
        Sweep {
            points: vec![(19, 1), (19, 3), (1, 3), (1, 5), (19, 5), (19, 7), (1, 7)],
            next_index: 0,
        }
    }
}

impl Iterator for Sweep {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let point = self.points[self.next_index];
        self.next_index = (self.next_index + 1) % self.points.len();
        Some(point)
    }
}

#[derive(Debug)]
struct Echo {
    sweep: Sweep,
    target: Option<(i32, i32)>,
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

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo { sweep: Sweep::zigzag(), target: None }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Reactor, true) => return Action::Insert,
            (Tile::Core, false) => return Action::Grab,
            _ => {}
        }
        if view.carrying() {
            return view.reactor_dir().map_or(Action::Wait, Action::Move);
        }
        let (px, py) = view.pos();
        let r = view.sight();
        // 시야 안에 보이는 코어 중 가장 가까운 것
        let mut seen = None;
        for dy in -r..=r {
            for dx in -r..=r {
                let (x, y) = (px + dx, py + dy);
                if view.tile_at(x, y) == Ok(Tile::Core) {
                    let d = dx.abs() + dy.abs();
                    if seen.map_or(true, |(_, best)| d < best) {
                        seen = Some(((x, y), d));
                    }
                }
            }
        }
        if let Some((at, _)) = seen {
            return toward(view, at);
        }
        // 안 보이면 순찰: 지점에 닿았으면 다음 지점을 받는다.
        if self.target.map_or(true, |t| t == (px, py)) {
            self.target = self.sweep.next();
        }
        match self.target {
            Some(t) => toward(view, t),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
