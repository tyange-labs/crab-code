// 정거장 복구 22 · 에너지 — 참조를 품은 구조체와 라이프타임
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin energy

use station::level22::*;

#[derive(Debug)]
struct Echo;

fn dist(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

/// View 를 빌려서 에너지 계산을 한곳에 모은다. (참조를 품으니 라이프타임 'a 가 필요해요)
struct Budget<'a> {
    view: &'a View,
}

impl<'a> Budget<'a> {
    fn new(view: &'a View) -> Budget<'a> {
        Budget { view }
    }

    /// `from` 에서 가장 가까운 충전 지점 (반응로 또는 패드).
    fn nearest_charge(&self, from: (i32, i32)) -> (i32, i32) {
        let mut points = self.view.pads();
        points.push(self.view.reactor());
        points.into_iter().min_by_key(|&p| dist(from, p)).unwrap()
    }

    /// 지금 위치에서 `target` 까지 갔다가, 거기서 가장 가까운 충전 지점까지 돌아올 에너지가 되나?
    fn can_visit_and_return(&self, _target: (i32, i32)) -> bool {
        // dist(지금 위치, target) + dist(target, 가장 가까운 충전 지점) <= self.view.energy() 이면 true.
        // 가장 가까운 충전 지점은 self.nearest_charge(target) 로 얻어요.
        false
    }

    fn can_reach(&self, target: (i32, i32)) -> bool {
        dist(self.view.pos(), target) <= self.view.energy()
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
        let budget = Budget::new(view);
        let pos = view.pos();
        if view.carrying() {
            let reactor = view.reactor();
            // 반응로까지 갈 에너지가 되면 곧장, 모자라면 가까운 패드에서 먼저 충전
            let target = if budget.can_reach(reactor) { reactor } else { budget.nearest_charge(pos) };
            return toward(view, target);
        }
        let Some(core) = view.cores().into_iter().min_by_key(|&c| dist(pos, c)) else {
            return Action::Wait;
        };
        if budget.can_visit_and_return(core) {
            toward(view, core)
        } else {
            // 코어에서 가장 가까운 충전 지점으로 먼저 가서 가득 채우고 출발해요.
            let charge = budget.nearest_charge(core);
            if charge == pos {
                Action::Wait
            } else {
                toward(view, charge)
            }
        }
    }
}

fn main() {
    play::<Echo>();
}
