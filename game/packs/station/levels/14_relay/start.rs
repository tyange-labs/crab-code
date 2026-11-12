// 정거장 복구 14 · 통신 채널 — static 과 Mutex
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin relay

use station::level14::*;
use std::sync::Mutex;

/// 맡은 슬롯 목록 (로봇 모두가 함께 봐요).
static BOARD: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());

#[derive(Debug)]
struct Echo {
    claim: Option<(i32, i32)>,
}

impl Echo {
    /// 아무도 안 맡은 가장 가까운 빈 슬롯을 골라 게시판에 적어요.
    fn claim_slot(&mut self, _view: &View) {
        // 지도 전체를 view.tile_at(x, y) 로 훑어 Ok(Tile::Slot) 이면서 BOARD 에 없는 가장 가까운 칸을 찾아,
        // BOARD.lock().unwrap().push(칸) 으로 적고 self.claim = Some(칸) 으로 기억하게 채워 보세요.
    }

    /// 맡은 슬롯을 게시판에서 지워요.
    fn release(&mut self) {
        if let Some(at) = self.claim.take() {
            BOARD.lock().unwrap().retain(|&c| c != at);
        }
    }
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo { claim: None }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Reactor, true) => return Action::Insert,
            (Tile::Slot, false) if self.claim == Some(view.pos()) => return Action::Install,
            (Tile::Module, false) if self.claim == Some(view.pos()) => return Action::Charge,
            (Tile::Core, false) if self.claim == Some(view.pos()) => {
                self.release();
                return Action::Grab;
            }
            _ => {}
        }
        if view.carrying() {
            return view.reactor_dir().map_or(Action::Wait, Action::Move);
        }
        if self.claim.is_none() {
            self.claim_slot(view);
        }
        let Some((x, y)) = self.claim else { return Action::Wait };
        let (px, py) = view.pos();
        if (x, y) == (px, py) {
            return Action::Wait; // 부팅하는 동안 기다려요
        }
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
}

fn main() {
    play::<Echo>();
}
