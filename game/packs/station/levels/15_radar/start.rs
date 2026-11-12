// 정거장 복구 15 · 레이더 해독 — &str 과 라이프타임
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin radar

use station::level15::*;

#[derive(Debug)]
struct Echo;

/// "C:4,2" 같은 조각 하나를 해독해요. 알아볼 수 없으면 None.
fn parse_core(_token: &str) -> Option<(i32, i32)> {
    // strip_prefix("C:")? → split_once(',')? → parse().ok()? 로 (x, y) 를 만들어 보세요.
    None
}

/// 레이더 글에서 `at` 에서 가장 가까운 코어 조각을 빌려 와요.
/// 돌려주는 &str 는 radar 에서 잘라 온 것이라 radar 와 같은 수명 'a 를 가져요.
fn closest<'a>(radar: &'a str, at: (i32, i32)) -> Option<&'a str> {
    radar
        .split(' ')
        .filter(|t| parse_core(t).is_some())
        .min_by_key(|t| {
            let (x, y) = parse_core(t).unwrap();
            (x - at.0).abs() + (y - at.1).abs()
        })
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
            return view.reactor_dir().map_or(Action::Wait, Action::Move);
        }
        let (px, py) = view.pos();
        match closest(view.radar(), (px, py)).and_then(parse_core) {
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
