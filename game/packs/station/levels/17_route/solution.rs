// 정거장 복구 17 · 경로 계산 — 스레드와 채널
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin route

use station::level17::*;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// 계산 스레드에 보내는 요청: 열린 칸 표와 출발·도착.
#[derive(Debug)]
struct Request {
    open: Vec<bool>,
    w: i32,
    h: i32,
    from: (i32, i32),
    to: (i32, i32),
}

/// 너비 우선 탐색으로 from 에서 to 까지의 길(방향들)을 구해요. 길이 없으면 빈 목록.
fn find_path(req: &Request) -> Vec<Dir> {
    let idx = |x: i32, y: i32| (y * req.w + x) as usize;
    let mut seen = vec![false; (req.w * req.h) as usize];
    let mut prev: HashMap<(i32, i32), ((i32, i32), Dir)> = HashMap::new();
    let mut queue = VecDeque::new();
    seen[idx(req.from.0, req.from.1)] = true;
    queue.push_back(req.from);
    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == req.to {
            break;
        }
        for (dir, dx, dy) in [(Dir::North, 0, -1), (Dir::East, 1, 0), (Dir::South, 0, 1), (Dir::West, -1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= req.w || ny >= req.h || seen[idx(nx, ny)] || !req.open[idx(nx, ny)] {
                continue;
            }
            seen[idx(nx, ny)] = true;
            prev.insert((nx, ny), ((x, y), dir));
            queue.push_back((nx, ny));
        }
    }
    // 도착에서 거꾸로 따라가며 길을 만든다.
    let mut path = Vec::new();
    let mut cur = req.to;
    while let Some(&(before, dir)) = prev.get(&cur) {
        path.push(dir);
        cur = before;
    }
    path.reverse();
    path
}

#[derive(Debug)]
struct Echo {
    requests: Sender<Request>,
    replies: Receiver<Vec<Dir>>,
    path: VecDeque<Dir>,
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        let (requests, incoming) = channel::<Request>();
        let (outgoing, replies) = channel::<Vec<Dir>>();
        // 계산 스레드: 요청이 올 때마다 길을 계산해 돌려 보낸다.
        thread::spawn(move || {
            for request in incoming {
                outgoing.send(find_path(&request)).ok();
            }
        });
        Echo { requests, replies, path: VecDeque::new() }
    }

    fn tick(&mut self, view: &View) -> Action {
        match (view.here(), view.carrying()) {
            (Tile::Reactor, true) => return Action::Insert,
            (Tile::Core, false) => return Action::Grab,
            _ => {}
        }
        if view.carrying() {
            self.path.clear();
            return view.reactor_dir().map_or(Action::Wait, Action::Move);
        }
        if self.path.is_empty() {
            let (px, py) = view.pos();
            let Some(to) = view
                .cores()
                .into_iter()
                .min_by_key(|&(x, y)| (x - px).abs() + (y - py).abs())
            else {
                return Action::Wait;
            };
            let (w, h) = view.size();
            let open = (0..h)
                .flat_map(|y| (0..w).map(move |x| (x, y)))
                .map(|(x, y)| view.tile_at(x, y).map_or(false, |t| t.is_open()))
                .collect();
            self.requests.send(Request { open, w, h, from: (px, py), to }).unwrap();
            self.path = self.replies.recv().unwrap().into();
        }
        match self.path.pop_front() {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}

fn main() {
    play::<Echo>();
}
