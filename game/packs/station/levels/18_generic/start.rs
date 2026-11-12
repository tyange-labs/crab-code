// 정거장 복구 18 · 일반 부품 — 제네릭과 trait 경계
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin generic

use station::level18::*;

/// 위치가 있는 것.
trait Located {
    fn at(&self) -> (i32, i32);
}

#[derive(Debug, Clone, Copy)]
struct CoreSpot {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct SlotSpot {
    x: i32,
    y: i32,
}

impl Located for CoreSpot {
    fn at(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Located for SlotSpot {
    fn at(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// `Located` 인 것들 중 `from` 에서 가장 가까운 것.
fn nearest<T: Located>(_items: &[T], _from: (i32, i32)) -> Option<&T> {
    // items.iter().min_by_key(|item| …) 로 맨해튼 거리 (x - fx).abs() + (y - fy).abs() 가 가장 작은 것을 고르세요.
    // item.at() 으로 위치를 얻어요 (T: Located 경계 덕분이에요).
    None
}

#[derive(Debug)]
struct Echo;

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
            (Tile::Slot, _) => return Action::Install,
            (Tile::Module, _) => return Action::Charge,
            (Tile::Core, false) => return Action::Grab,
            (Tile::Reactor, true) => return Action::Insert,
            _ => {}
        }
        if view.carrying() {
            return view.reactor_dir().map_or(Action::Wait, Action::Move);
        }
        let pos = view.pos();
        let cores: Vec<CoreSpot> = view.cores().into_iter().map(|(x, y)| CoreSpot { x, y }).collect();
        if let Some(core) = nearest(&cores, pos) {
            return toward(view, core.at());
        }
        let (w, h) = view.size();
        let slots: Vec<SlotSpot> = (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .filter(|&(x, y)| matches!(view.tile_at(x, y), Ok(Tile::Slot) | Ok(Tile::Module)))
            .map(|(x, y)| SlotSpot { x, y })
            .collect();
        match nearest(&slots, pos) {
            Some(slot) => toward(view, slot.at()),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
