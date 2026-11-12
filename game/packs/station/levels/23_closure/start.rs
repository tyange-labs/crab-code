// 정거장 복구 23 · 정책 — 클로저와 Fn 경계
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin closure

use station::level23::*;

#[derive(Debug)]
struct Echo;

fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// 점수가 가장 낮은 코어를 고른다. `score` 는 코어 위치를 받아 점수를 돌려주는 클로저.
fn pick<F>(cores: &[(i32, i32)], score: F) -> Option<(i32, i32)>
where
    F: Fn((i32, i32)) -> i32,
{
    // cores.iter().copied().min_by_key(|&c| score(c)) 로 점수가 가장 낮은 코어를 고르게 채워 보세요.
    let _ = (cores, score);
    None
}

/// 보이는 경비 드론의 위치들.
fn drones(view: &View) -> Vec<(i32, i32)> {
    let (w, h) = view.size();
    (0..h)
        .flat_map(|y| (0..w).map(move |x| (x, y)))
        .filter(|&(x, y)| view.tile_at(x, y) == Ok(Tile::Sentry))
        .collect()
}

/// 그쪽으로 한 걸음. 드론에게 잡힐 것 같으면 기다린다.
fn go(view: &View, dir: Option<Dir>) -> Action {
    match dir {
        Some(d) if view.danger(d) => Action::Wait,
        Some(d) => Action::Move(d),
        None => Action::Wait,
    }
}

fn toward(view: &View, (x, y): (i32, i32)) -> Action {
    let (px, py) = view.pos();
    go(
        view,
        Some(if x > px {
            Dir::East
        } else if x < px {
            Dir::West
        } else if y > py {
            Dir::South
        } else {
            Dir::North
        }),
    )
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Reactor, true) => return Action::Insert,
            (Tile::Core, false) => return Action::Grab,
            _ => {}
        }
        if view.carrying() {
            return go(view, view.reactor_dir());
        }
        let pos = view.pos();
        let sentries = drones(view);
        // 거리 + 드론 길목(드론 열에서 한 칸 안)이면 벌점. 클로저가 pos 와 sentries 를 가져다 써요.
        let score = |c: (i32, i32)| {
            let near = sentries.iter().filter(|d| (d.0 - c.0).abs() <= 1).count() as i32;
            dist(pos, c) + 8 * near
        };
        match pick(&view.cores(), score) {
            Some(target) => toward(view, target),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
