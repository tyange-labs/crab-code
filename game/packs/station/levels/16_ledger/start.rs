// 정거장 복구 16 · 정거장 장부 — Rc 와 RefCell
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin ledger

use station::level16::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 본 칸을 적어 두는 장부.
#[derive(Debug, Default)]
struct Ledger {
    seen: HashMap<(i32, i32), Tile>,
}

/// 주변을 훑어 장부에 적어요.
#[derive(Debug)]
struct Scanner {
    ledger: Rc<RefCell<Ledger>>,
}

impl Scanner {
    fn scan(&self, _view: &View) {
        // view.pos(), view.sight(), view.tile_at(x, y) 로 주변을 보고
        // self.ledger.borrow_mut().seen.insert((x, y), tile) 로 장부에 적게 채워 보세요.
    }
}

/// 장부를 읽어 갈 곳을 골라요.
#[derive(Debug)]
struct Planner {
    ledger: Rc<RefCell<Ledger>>,
}

impl Planner {
    fn target(&self, view: &View) -> Option<(i32, i32)> {
        let (px, py) = view.pos();
        let dist = |&(x, y): &(i32, i32)| (x - px).abs() + (y - py).abs();
        let ledger = self.ledger.borrow();
        let work = ledger
            .seen
            .iter()
            .filter(|(&at, t)| at != (px, py) && matches!(**t, Tile::Slot | Tile::Module | Tile::Core))
            .map(|(&at, _)| at)
            .min_by_key(dist);
        if work.is_some() {
            return work;
        }
        let (w, h) = view.size();
        (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .filter(|at| !ledger.seen.contains_key(at))
            .min_by_key(dist)
    }
}

#[derive(Debug)]
struct Echo {
    scanner: Scanner,
    planner: Planner,
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
        let ledger = Rc::new(RefCell::new(Ledger::default()));
        Echo {
            scanner: Scanner { ledger: Rc::clone(&ledger) },
            planner: Planner { ledger },
        }
    }

    fn tick(&mut self, view: &View) -> Action {
        self.scanner.scan(view);
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
        match self.planner.target(view) {
            Some(at) => toward(view, at),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
