// 게임 엔진: 꽃게 한 마리가 사는 세계와, 행동할 때마다 게임 창으로 보내는 출력.
//
// 플레이어는 이 파일을 몰라도 된다. 플레이어가 쓰는 함수는 레벨마다의 모듈(level1.rs …)에 있고,
// 그 함수들이 여기 있는 날것의 함수(보석을 번호로 다룬다)를 부른다.

#![allow(dead_code)] // 세계 시뮬레이션의 일부는 게임 쪽 기능에만 쓰인다.

mod bridge;
mod out;
pub(crate) mod world;

use out::Json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use world::{Action, Color, Sign, World};

const PREFIX: &str = "\u{1}crab ";
const STEP_LIMIT: usize = 400;

static WORLD: Mutex<Option<World>> = Mutex::new(None);
static STEPS: AtomicUsize = AtomicUsize::new(0);

fn host() -> bool {
    std::env::var("CRAB_HOST").is_ok()
}

fn emit(json: Json) {
    if host() {
        println!("{PREFIX}{}", json.finish());
    }
}

fn level_id() -> String {
    std::env::var("CRAB_LEVEL").ok().unwrap_or_else(|| {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
            .unwrap_or_default()
    })
}

fn seed() -> u64 {
    std::env::var("CRAB_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1)
    })
}

fn color_name(color: Color) -> &'static str {
    match color {
        Color::Red => "red",
        Color::Blue => "blue",
    }
}

fn world_json(w: &World) -> Json {
    let walls = format!("[{}]", w.walls.iter().map(|b| if *b { "true" } else { "false" }).collect::<Vec<_>>().join(","));
    let gems = w.gems.iter().map(|g| {
        let mut j = Json::object();
        j.num("x", g.x)
            .num("y", g.y)
            .str("color", color_name(g.color))
            .num("weight", g.weight)
            .bool("shiny", g.shiny)
            .bool("held", g.held)
            .bool("placed", g.placed);
        j
    });
    let pedestals = w.pedestals.iter().map(|p| {
        let mut j = Json::object();
        j.num("x", p.x).num("y", p.y);
        match p.want {
            Some(c) => j.str("want", color_name(c)),
            None => j.raw("want", "null"),
        };
        j
    });
    let signs = w.signs.iter().map(|(x, y, s)| {
        let mut j = Json::object();
        j.num("x", *x).num("y", *y).str(
            "sign",
            match s {
                Sign::Left => "left",
                Sign::Right => "right",
                Sign::Finish => "finish",
                Sign::Straight => "straight",
            },
        );
        j
    });
    let numbers = w.numbers.iter().map(|(x, y, n)| {
        let mut j = Json::object();
        j.num("x", *x).num("y", *y).num("n", *n);
        j
    });
    let mut out = Json::object();
    out.num("w", w.w)
        .num("h", w.h)
        .raw("walls", &walls)
        .num("x", w.x)
        .num("y", w.y)
        .num("dir", w.dir)
        .array("gems", gems)
        .array("pedestals", pedestals)
        .array("signs", signs)
        .array("numbers", numbers)
        .bool("showWeights", w.show_weights);
    match &w.door {
        Some(d) => {
            let mut j = Json::object();
            j.num("x", d.x).num("y", d.y).bool("open", d.open);
            out.json("door", j);
        }
        None => {
            out.raw("door", "null");
        }
    }
    match w.flag {
        Some((x, y)) => out.raw("flag", &format!("{{\"x\":{x},\"y\":{y}}}")),
        None => out.raw("flag", "null"),
    };
    match w.dial {
        Some(d) => out.num("dial", d),
        None => out.raw("dial", "null"),
    };
    out.raw("scroll", &format!("[{}]", w.scroll.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")));
    match &w.note {
        Some(n) => out.str("note", n),
        None => out.raw("note", "null"),
    };
    match &w.note2 {
        Some(n) => out.str("note2", n),
        None => out.raw("note2", "null"),
    };
    out
}

fn outcome(w: &World, failure: Option<&str>) -> Json {
    let mut j = Json::object();
    match failure {
        Some(msg) => j.bool("won", false).str("message", msg),
        None => match w.todo_hint() {
            Some(todo) => j.bool("won", false).str("message", &format!("아직이에요. {todo}")),
            None => j.bool("won", true).str("message", "성공! 목표를 달성했어요."),
        },
    };
    j
}

fn frame(w: &World, action: &str, bump: bool, failure: Option<&str>) -> Json {
    let steps = STEPS.load(Ordering::SeqCst);
    let mut f = Json::object();
    f.str("t", "frame")
        .num("tick", steps)
        .str("action", action)
        .bool("bump", bump)
        .json("world", world_json(w))
        .json("outcome", outcome(w, failure));
    if let Some(msg) = failure {
        let mut ev = Json::object();
        ev.num("agent", 0).str("text", msg);
        f.array("events", std::iter::once(ev));
    }
    let mut stats = Json::object();
    stats.num("ticks", steps);
    f.json("stats", stats);
    f
}

fn with<T>(f: impl FnOnce(&mut World) -> T) -> T {
    let mut guard = WORLD.lock().unwrap_or_else(|e| e.into_inner());
    let world = guard.get_or_insert_with(|| {
        let id = level_id();
        let Some(level) = world::SPECS.iter().position(|s| s.id == id) else {
            eprintln!("'{id}' 라는 레벨은 없어요. 파일 이름(실행 파일 이름)을 바꾸지 않았는지 확인해 주세요.");
            std::process::exit(2);
        };
        let w = world::build_level(level, seed());
        let mut start = Json::object();
        start.str("t", "start").str("pack", "puzzle").str("level", &id);
        let mut map = Json::object();
        map.num("agents", 1);
        start.json("map", map);
        emit(start);
        emit(frame(&w, "start", false, None));
        w
    });
    f(world)
}

fn action_name(action: Action) -> &'static str {
    match action {
        Action::Forward => "forward",
        Action::Left => "left",
        Action::Right => "right",
        Action::Collect => "collect",
        Action::Deposit(_) => "deposit",
        Action::Polish(_) => "polish",
        Action::Unlock(_) => "unlock",
    }
}

fn act(action: Action) {
    // 세계를 먼저 만들어 첫 모습이 0번째 행동으로 나가게 한다.
    with(|_| ());
    if STEPS.fetch_add(1, Ordering::SeqCst) >= STEP_LIMIT {
        let msg = "너무 많이 움직였어요. 반복문이 끝나지 않는 건 아닐까요?";
        with(|w| emit(frame(w, "limit", false, Some(msg))));
        if !host() {
            eprintln!("{msg}");
        }
        std::process::exit(0);
    }
    let failed = with(|w| match w.apply(action) {
        Ok(()) => {
            emit(frame(w, action_name(action), false, None));
            None
        }
        Err(msg) => {
            emit(frame(w, action_name(action), true, Some(&msg)));
            Some(msg)
        }
    });
    if let Some(msg) = failed {
        if !host() {
            eprintln!("{msg}");
        }
        std::process::exit(0);
    }
    if !host() {
        // 터미널에서 돌리면 목표를 이룬 순간 알려 준다.
        if with(|w| w.is_won()) {
            println!("성공! 목표를 달성했어요.");
        }
    }
}

/// 게임 창의 미리 보기: 레벨의 처음 모습만 출력한다.
pub(crate) fn preview(level_id: &str, seed: u64) {
    std::env::set_var("CRAB_HOST", "1");
    std::env::set_var("CRAB_LEVEL", level_id);
    std::env::set_var("CRAB_SEED", seed.to_string());
    with(|_| ());
}

// ── 레벨 모듈이 부르는 날것의 함수 (보석은 번호로 다룬다) ──────────────────

pub(crate) fn move_forward() {
    act(Action::Forward);
}

pub(crate) fn turn_left() {
    act(Action::Left);
}

pub(crate) fn turn_right() {
    act(Action::Right);
}

/// 주운 보석의 번호.
pub(crate) fn collect() -> usize {
    let id = with(|w| w.gem_here()).unwrap_or(usize::MAX);
    act(Action::Collect);
    id
}

pub(crate) fn deposit(gem: usize) {
    act(Action::Deposit(gem));
}

pub(crate) fn color_of(gem: usize) -> Color {
    with(|w| w.gems[gem].color)
}

pub(crate) fn polish(gem: usize) {
    act(Action::Polish(gem));
}

pub(crate) fn unlock(code: u32) {
    act(Action::Unlock(code));
}

pub(crate) fn front_is_clear() -> bool {
    with(|w| w.front_is_clear())
}

pub(crate) fn on_gem() -> bool {
    with(|w| w.gem_here().is_some())
}

pub(crate) fn read_sign() -> Sign {
    with(|w| w.read_sign())
}

pub(crate) fn read_number() -> Option<u32> {
    with(|w| w.read_number())
}

/// (색, 무게, 광이 나는지)
pub(crate) fn inspect(gem: usize) -> (Color, u32, bool) {
    with(|w| {
        let g = &w.gems[gem];
        (g.color, g.weight, g.shiny)
    })
}

pub(crate) fn read_dial() -> u32 {
    with(|w| w.dial.unwrap_or(0))
}

pub(crate) fn read_scroll() -> Vec<u32> {
    with(|w| w.scroll.clone())
}

pub(crate) fn read_note() -> String {
    with(|w| w.note.clone().unwrap_or_default())
}

pub(crate) fn read_two_notes() -> (String, String) {
    with(|w| (w.note.clone().unwrap_or_default(), w.note2.clone().unwrap_or_default()))
}

pub(crate) fn gem_offset() -> (u32, u32) {
    with(|w| w.gem_offset())
}
