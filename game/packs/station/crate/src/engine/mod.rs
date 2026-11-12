// 게임 엔진의 바깥쪽: 실행 루프와, 게임 창으로 보내는 출력.
//
// 레벨 모듈(level1.rs …)의 play() 가 run() 을 부른다. 플레이어의 Bot 은 레벨마다 타입이 달라서
// "로봇을 만드는 법"과 "로봇에게 묻는 법"을 클로저로 받는다.

#![allow(dead_code)] // 세계 시뮬레이션의 일부는 게임 쪽 기능에만 쓰인다.

mod bridge;
mod out;
pub(crate) mod world;

use out::Json;
use std::fmt::Debug;
use world::{Dir, CoreState, Ground, Mishap, Field, Pressure as T, SlotStage};

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

fn map_json(p: &Field) -> Json {
    let ground: String = p
        .ground
        .iter()
        .map(|g| match g {
            Ground::Deck => '.',
            Ground::Wall => '#',
            Ground::Void => 'w',
            Ground::Duct => '~',
            Ground::Reactor => 'N',
            Ground::Slot => 'p',
            Ground::Door => 'D',
            Ground::Pad => 'C',
        })
        .collect();
    let mut pressure = Json::object();
    match p.pressure {
        Some((period, high)) => {
            pressure.num("period", period).num("high", high);
        }
        None => pressure = Json::null(),
    }
    let mut map = Json::object();
    map.num("w", p.w)
        .num("h", p.h)
        .str("ground", &ground)
        .raw("reactor", &format!("[{},{}]", p.reactor.0, p.reactor.1))
        .num("goal", p.goal)
        .num("maxTicks", p.max_ticks)
        .num("bots", p.bots.len())
        .bool("fog", !p.slot_sense)
        .num("sight", p.sight)
        .num("energyMax", p.energy_max)
        .json("pressure", pressure);
    map
}

fn frame_json(p: &Field) -> Json {
    let mut pressure = Json::object();
    match p.pressure_now() {
        T::Safe { turns_left } => pressure.str("kind", "low").num("turnsLeft", turns_left),
        T::Vacuum { turns_left } => pressure.str("kind", "high").num("turnsLeft", turns_left),
        T::Stable => pressure.str("kind", "calm"),
    };
    let bots = p.bots.iter().map(|c| {
        let mut j = Json::object();
        j.num("x", c.x)
            .num("y", c.y)
            .num("energy", c.energy)
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
    let cores = p.cores.iter().filter(|f| f.state == CoreState::Ground).map(|f| {
        let mut j = Json::object();
        j.num("x", f.x).num("y", f.y);
        j
    });
    let slots = p.slots.iter().map(|k| {
        let mut j = Json::object();
        j.num("x", k.x).num("y", k.y);
        match k.stage {
            SlotStage::Empty => j.str("stage", "empty"),
            SlotStage::Installed => j.str("stage", "seed"),
            SlotStage::Charging { left } => j.str("stage", "booting").num("left", left),
        };
        j
    });
    let doors = p.doors.iter().map(|d| {
        let mut j = Json::object();
        j.num("x", d.x).num("y", d.y).str("hue", d.hue.code()).bool("open", d.open);
        j
    });
    let cards = p.cards.iter().map(|c| {
        let mut j = Json::object();
        j.num("x", c.x).num("y", c.y).str("hue", c.hue.code());
        j
    });
    let sentries = p.sentries.iter().map(|g| {
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
        .json("pressure", pressure)
        .array("bots", bots)
        .array("cores", cores)
        .array("slots", slots)
        .array("sentries", sentries)
        .array("doors", doors)
        .array("cards", cards)
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
    start.str("t", "start").str("pack", "station").str("level", level_id).json("map", map_json(&world));
    emit(start);
    emit(frame_json(&world));
}

/// 레벨 하나를 끝까지 돌린다.
/// new: 로봇 번호로 플레이어의 로봇을 만든다. think: 로봇에게 엔진의 View 를 보여 주고 엔진의 Action 을 받는다.
pub(crate) fn run<C: Debug>(level_id: &str, new: impl Fn(usize) -> C, think: impl Fn(&mut C, world::View) -> world::Action) {
    // 게임 창이 실행하면 CRAB_HOST 가 있다. 터미널에서 cargo run 하면 사람이 읽을 요약만 찍는다.
    let host = std::env::var("CRAB_HOST").is_ok();
    let Some(level) = world::SPECS.iter().position(|s| s.id == level_id) else {
        eprintln!("'{level_id}' 라는 구역은 없어요.");
        std::process::exit(2);
    };
    let mut world = world::build(level, seed());
    if host {
        let mut start = Json::object();
        start.str("t", "start").str("pack", "station").str("level", level_id).json("map", map_json(&world));
        emit(start);
        emit(frame_json(&world));
    }
    let mut bots: Vec<C> = (0..world.bots.len()).map(&new).collect();
    while !world.is_done() {
        let mut actions = Vec::with_capacity(bots.len());
        for (i, bot) in bots.iter_mut().enumerate() {
            let view = world.view(i);
            if host {
                let mut think_msg = Json::object();
                think_msg.str("t", "think").num("tick", world.tick).num("agent", i);
                emit(think_msg);
            }
            let action = think(bot, view);
            if host {
                let mut agent = Json::object();
                agent
                    .str("t", "agent")
                    .num("tick", world.tick)
                    .num("agent", i)
                    .str("action", &format!("{action:?}"))
                    .str("state", &format!("{bot:?}"));
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
        format!("성공! {}틱 만에 코어에 닿았어요.", world.tick)
    } else if won {
        format!("성공! {}틱 만에 코어 {}개를 배달했어요.", world.tick, world.goal)
    } else if world.reach {
        format!("{}틱 안에 코어에 닿지 못했어요.", world.max_ticks)
    } else {
        format!("{}틱 안에 다 배달하지 못했어요. ({}/{}개)", world.max_ticks, world.delivered, world.goal)
    };
    if host {
        let mut stats = Json::object();
        stats.num("ticks", world.tick).num("delivered", world.delivered);
        let mut end = Json::object();
        end.str("t", "end").bool("won", won).str("message", &message).json("stats", stats);
        emit(end);
    } else {
        println!("{message}");
        println!("(게임 창에서 보려면 게임을 켜 두고 파일을 저장하세요.)");
    }
}
