// 게임 엔진의 바깥쪽: 실행 루프와, 게임 창으로 보내는 출력.
//
// 레벨 모듈(level1.rs …)의 play() 가 run() 을 부른다. 플레이어의 Crab 은 레벨마다 타입이 달라서
// "꽃게를 만드는 법"과 "꽃게에게 묻는 법"을 클로저로 받는다.

#![allow(dead_code)] // 세계 시뮬레이션의 일부는 게임 쪽 기능에만 쓰인다.

mod bridge;
mod out;
pub(crate) mod world;

use out::Json;
use std::fmt::Debug;
use world::{Dir, FoodState, Ground, Mishap, Pool, Tide as T};

/// 게임 창과 주고받는 줄의 머리. 이 글자로 시작하지 않는 줄은 플레이어의 `println!` 이다.
const PREFIX: &str = "\u{1}crab ";

pub(crate) fn emit(json: Json) {
    println!("{PREFIX}{}", json.finish());
}

fn dir_code(d: Dir) -> &'static str {
    match d {
        Dir::North => "N",
        Dir::East => "E",
        Dir::South => "S",
        Dir::West => "W",
    }
}

fn map_json(p: &Pool) -> Json {
    let ground: String = p
        .ground
        .iter()
        .map(|g| match g {
            Ground::Sand => '.',
            Ground::Rock => '#',
            Ground::Water => 'w',
            Ground::Flat => '~',
            Ground::Nest => 'N',
        })
        .collect();
    let mut tide = Json::object();
    match p.tide {
        Some((period, high)) => {
            tide.num("period", period).num("high", high);
        }
        None => tide = Json::null(),
    }
    let mut map = Json::object();
    map.num("w", p.w)
        .num("h", p.h)
        .str("ground", &ground)
        .raw("nest", &format!("[{},{}]", p.nest.0, p.nest.1))
        .num("goal", p.goal)
        .num("maxTicks", p.max_ticks)
        .num("crabs", p.crabs.len())
        .json("tide", tide);
    map
}

fn frame_json(p: &Pool) -> Json {
    let mut tide = Json::object();
    match p.tide_now() {
        T::Low { turns_left } => tide.str("kind", "low").num("turnsLeft", turns_left),
        T::High { turns_left } => tide.str("kind", "high").num("turnsLeft", turns_left),
        T::Calm => tide.str("kind", "calm"),
    };
    let crabs = p.crabs.iter().map(|c| {
        let mut j = Json::object();
        j.num("x", c.x)
            .num("y", c.y)
            .bool("carrying", c.carrying.is_some())
            .str("facing", dir_code(c.facing))
            .str(
                "mishap",
                match c.mishap {
                    Mishap::None => "none",
                    Mishap::Bump => "bump",
                    Mishap::Swept => "swept",
                    Mishap::Caught => "caught",
                },
            );
        j
    });
    let foods = p.foods.iter().filter(|f| f.state == FoodState::Ground).map(|f| {
        let mut j = Json::object();
        j.num("x", f.x).num("y", f.y);
        j
    });
    let gulls = p.gulls.iter().map(|g| {
        let mut j = Json::object();
        j.num("x", g.x).num("y", g.y).str("dir", dir_code(g.dir));
        j
    });
    let events = p.events.iter().map(|(c, text)| {
        let mut j = Json::object();
        j.num("agent", *c).str("text", text);
        j
    });
    let mut frame = Json::object();
    frame
        .str("t", "frame")
        .num("tick", p.tick)
        .num("delivered", p.delivered)
        .json("tide", tide)
        .array("crabs", crabs)
        .array("foods", foods)
        .array("gulls", gulls)
        .array("events", events);
    frame
}

fn seed() -> u64 {
    std::env::var("CRAB_SEED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(1)
        })
}

/// 게임 창의 미리 보기: 레벨의 지도와 처음 모습만 출력한다.
pub(crate) fn preview(level_id: &str, seed: u64) {
    let Some(level) = world::SPECS.iter().position(|s| s.id == level_id) else {
        std::process::exit(2);
    };
    let world = world::build(level, seed);
    let mut start = Json::object();
    start.str("t", "start").str("pack", "tidepool").str("level", level_id).json("map", map_json(&world));
    emit(start);
    emit(frame_json(&world));
}

/// 레벨 하나를 끝까지 돌린다.
/// new: 꽃게 번호로 플레이어의 꽃게를 만든다. think: 꽃게에게 엔진의 View 를 보여 주고 엔진의 Action 을 받는다.
pub(crate) fn run<C: Debug>(level_id: &str, new: impl Fn(usize) -> C, think: impl Fn(&mut C, world::View) -> world::Action) {
    // 게임 창이 실행하면 CRAB_HOST 가 있다. 터미널에서 cargo run 하면 사람이 읽을 요약만 찍는다.
    let host = std::env::var("CRAB_HOST").is_ok();
    let Some(level) = world::SPECS.iter().position(|s| s.id == level_id) else {
        eprintln!("'{level_id}' 라는 웅덩이는 없어요.");
        std::process::exit(2);
    };
    let mut world = world::build(level, seed());
    if host {
        let mut start = Json::object();
        start.str("t", "start").str("pack", "tidepool").str("level", level_id).json("map", map_json(&world));
        emit(start);
        emit(frame_json(&world));
    }
    let mut crabs: Vec<C> = (0..world.crabs.len()).map(&new).collect();
    while !world.is_done() {
        let mut actions = Vec::with_capacity(crabs.len());
        for (i, crab) in crabs.iter_mut().enumerate() {
            let view = world.view(i);
            if host {
                let mut think_msg = Json::object();
                think_msg.str("t", "think").num("tick", world.tick).num("agent", i);
                emit(think_msg);
            }
            let action = think(crab, view);
            if host {
                let mut agent = Json::object();
                agent
                    .str("t", "agent")
                    .num("tick", world.tick)
                    .num("agent", i)
                    .str("action", &format!("{action:?}"))
                    .str("state", &format!("{crab:?}"));
                emit(agent);
            }
            actions.push(action);
        }
        world.step(&actions);
        if host {
            emit(frame_json(&world));
        }
    }
    let won = world.is_won();
    let message = if won && world.reach {
        format!("성공! {}틱 만에 먹이에 닿았어요.", world.tick)
    } else if won {
        format!("성공! {}틱 만에 먹이 {}개를 모았어요.", world.tick, world.goal)
    } else if world.reach {
        format!("{}틱 안에 먹이에 닿지 못했어요.", world.max_ticks)
    } else {
        format!("{}틱 안에 다 모으지 못했어요. ({}/{}개)", world.max_ticks, world.delivered, world.goal)
    };
    if host {
        let mut stats = Json::object();
        stats.num("ticks", world.tick).num("delivered", world.delivered);
        let mut end = Json::object();
        end.str("t", "end").bool("won", won).str("message", &message).json("stats", stats);
        emit(end);
    } else {
        println!("{message}");
        println!("(게임 창에서 보려면 Crab Code 를 켜 두고 파일을 저장하세요.)");
    }
}
