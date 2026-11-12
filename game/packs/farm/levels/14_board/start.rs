// 해바라기 농장 14 · 마을 게시판 — static 과 Mutex
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin board

use farm::level14::*;
use std::sync::Mutex;

/// 맡은 이랑 목록 (농부 모두가 함께 봐요).
static BOARD: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());

#[derive(Debug)]
struct Pip {
    claim: Option<(i32, i32)>,
}

impl Pip {
    /// 아무도 안 맡은 가장 가까운 빈 이랑을 골라 게시판에 적어요.
    fn claim_plot(&mut self, _view: &View) {
        // 지도 전체를 view.tile_at(x, y) 로 훑어 Ok(Tile::Plot) 이면서 BOARD 에 없는 가장 가까운 칸을 찾아,
        // BOARD.lock().unwrap().push(칸) 으로 적고 self.claim = Some(칸) 으로 기억하게 채워 보세요.
    }

    /// 맡은 이랑을 게시판에서 지워요.
    fn release(&mut self) {
        if let Some(at) = self.claim.take() {
            BOARD.lock().unwrap().retain(|&c| c != at);
        }
    }
}

impl Farmer for Pip {
    fn new(_id: usize) -> Self {
        Pip { claim: None }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Barn, true) => return Action::Store,
            (Tile::Plot, false) if self.claim == Some(view.pos()) => return Action::Plant,
            (Tile::Seed, false) if self.claim == Some(view.pos()) => return Action::Water,
            (Tile::Crop, false) if self.claim == Some(view.pos()) => {
                self.release();
                return Action::Harvest;
            }
            _ => {}
        }
        if view.carrying() {
            return view.barn_dir().map_or(Action::Wait, Action::Move);
        }
        if self.claim.is_none() {
            self.claim_plot(view);
        }
        let Some((x, y)) = self.claim else { return Action::Wait };
        let (px, py) = view.pos();
        if (x, y) == (px, py) {
            return Action::Wait; // 자라는 동안 기다려요
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
    play::<Pip>();
}
