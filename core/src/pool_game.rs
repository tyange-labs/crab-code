// 꽃게 AI 모드(조수 웅덩이)에서 화면이 부르는 기능과 레벨 검증.
//
// 플레이어 프로그램의 출력 형식 (runtime/pool_crab.rs):
//   @@T <틱> <꽃게>   이제부터 이 꽃게의 tick 이 돈다 (그 사이의 println! 은 이 꽃게 것)
//   @@A <행동>        tick 이 고른 행동
//   @@S <상태>        tick 을 마친 꽃게의 {:?}
//   @@E               시뮬레이션이 끝났다

use crate::game::diagnostics_json;
use crate::pool::{self, Action, Dir, FoodState, Ground, Mishap, Pool, Tide, LEVEL_COUNT, SPECS};
use crate::pool_lessons::POOL_LESSONS;
use crate::runner::{self, Mode};
use serde_json::{json, Value};

const POOL_SRC: &str = include_str!("pool.rs");
const POOL_CRAB_SRC: &str = include_str!("../runtime/pool_crab.rs");

/// 소스에서 "//>> 참고" 와 "//<< 참고" 사이 구간들.
fn reference_regions(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in src.lines() {
        match line.trim() {
            "//>> 참고" => current = Some(String::new()),
            "//<< 참고" => {
                if let Some(region) = current.take() {
                    out.push(region.trim_end().to_string());
                }
            }
            _ => {
                if let Some(region) = current.as_mut() {
                    region.push_str(line);
                    region.push('\n');
                }
            }
        }
    }
    out
}

/// 에디터의 읽기 전용 crab.rs 탭: 플레이어 코드와 함께 컴파일되는 실제 소스에서
/// 플레이어가 쓰는 부분만 뽑아 모은다. (그래서 실제 코드와 어긋나지 않는다.)
pub fn api_source() -> String {
    let mut regions = reference_regions(POOL_CRAB_SRC);
    regions.extend(reference_regions(POOL_SRC));
    // 읽기 좋은 순서: Crab → View → Action → Dir → Tile → Tide
    let order = ["pub trait Crab", "pub struct View", "pub enum Action", "pub enum Dir", "pub enum Tile", "pub enum Tide"];
    regions.sort_by_key(|r| order.iter().position(|k| r.contains(k)).unwrap_or(order.len()));
    let header = "\
// crab.rs: 꽃게 AI 모드에서 쓸 수 있는 것들 (읽기 전용)
//
// 내 코드(ferris.rs)는 이 파일과 함께 컴파일돼요. 그래서 여기 있는 이름은 use 없이 바로 쓸 수 있어요.
// 게임이 실제로 쓰는 소스에서 그대로 가져온 코드예요.
//
//   F12 · Cmd+클릭   이름의 정의로 가기       Ctrl+-   (Windows: Alt+←) 돌아가기
";
    format!("{header}\n{}\n", regions.join("\n\n"))
}

pub fn lessons_json() -> Value {
    json!({
        "lessons": POOL_LESSONS.iter().map(|l| json!({
            "id": l.id, "title": l.title, "body": l.body,
            "start": l.start, "solution": l.solution, "hint": l.hint,
        })).collect::<Vec<_>>(),
    })
}

fn dir_code(d: Dir) -> &'static str {
    match d {
        Dir::North => "N",
        Dir::East => "E",
        Dir::South => "S",
        Dir::West => "W",
    }
}

/// 바뀌지 않는 지도 정보.
fn map_json(p: &Pool) -> Value {
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
    json!({
        "w": p.w, "h": p.h, "ground": ground,
        "nest": [p.nest.0, p.nest.1],
        "goal": p.goal, "maxTicks": p.max_ticks, "crabs": p.crabs.len(),
        "tide": p.tide.map(|(period, high)| json!({ "period": period, "high": high })),
    })
}

/// 한 틱의 모습.
fn frame_json(p: &Pool) -> Value {
    let tide = match p.tide_now() {
        Tide::Low { turns_left } => json!({ "kind": "low", "turnsLeft": turns_left }),
        Tide::High { turns_left } => json!({ "kind": "high", "turnsLeft": turns_left }),
        Tide::Calm => json!({ "kind": "calm" }),
    };
    json!({
        "tick": p.tick,
        "delivered": p.delivered,
        "tide": tide,
        "crabs": p.crabs.iter().map(|c| json!({
            "x": c.x, "y": c.y, "carrying": c.carrying.is_some(), "facing": dir_code(c.facing),
            "mishap": match c.mishap {
                Mishap::None => "none", Mishap::Bump => "bump", Mishap::Swept => "swept", Mishap::Caught => "caught",
            },
        })).collect::<Vec<_>>(),
        "foods": p.foods.iter().filter(|f| f.state == FoodState::Ground).map(|f| json!({ "x": f.x, "y": f.y })).collect::<Vec<_>>(),
        "gulls": p.gulls.iter().map(|g| json!({ "x": g.x, "y": g.y, "dir": dir_code(g.dir) })).collect::<Vec<_>>(),
        "events": p.events.iter().map(|(c, text)| json!({ "crab": c, "text": text })).collect::<Vec<_>>(),
    })
}

pub fn level_json(level: usize, seed: u64) -> Value {
    let p = pool::build(level, seed);
    json!({ "map": map_json(&p), "frame": frame_json(&p) })
}

pub fn check_code(code: &str) -> Value {
    diagnostics_json(&runner::check_mode(Mode::Pool, code))
}

/// 코드 줄 수: 빈 줄과 주석만 있는 줄은 세지 않는다.
pub fn code_lines(code: &str) -> usize {
    code.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .count()
}

struct Parsed {
    actions: Vec<Vec<Action>>,
    states: Vec<Vec<String>>,
    logs: Vec<Value>,
    ended: bool,
    last: (u32, usize),
}

fn parse(lines: &[String]) -> Parsed {
    let mut p = Parsed { actions: Vec::new(), states: Vec::new(), logs: Vec::new(), ended: false, last: (0, 0) };
    for line in lines {
        if let Some(rest) = line.strip_prefix("@@T ") {
            let mut it = rest.split_whitespace().filter_map(|s| s.parse::<usize>().ok());
            if let (Some(t), Some(c)) = (it.next(), it.next()) {
                p.last = (t as u32, c);
                while p.actions.len() <= t {
                    p.actions.push(Vec::new());
                    p.states.push(Vec::new());
                }
            }
        } else if let Some(rest) = line.strip_prefix("@@A ") {
            if let (Some(a), Some(v)) = (Action::decode(rest), p.actions.get_mut(p.last.0 as usize)) {
                v.push(a);
            }
        } else if let Some(rest) = line.strip_prefix("@@S ") {
            if let Some(v) = p.states.get_mut(p.last.0 as usize) {
                v.push(rest.to_string());
            }
        } else if line == "@@E" {
            p.ended = true;
        } else {
            p.logs.push(json!({ "tick": p.last.0, "crab": p.last.1, "text": line, "kind": "print" }));
        }
    }
    p
}

pub struct Outcome {
    pub won: bool,
    pub message: String,
    pub world: Pool,
}

/// 코드를 컴파일하고 실행한 뒤, 출력된 행동으로 시뮬레이션을 다시 돌린다.
pub fn run_code(code: &str, level: usize, seed: u64) -> Value {
    run_full(code, level, seed).0
}

fn run_full(code: &str, level: usize, seed: u64) -> (Value, Option<Outcome>) {
    let raw = runner::run_raw(Mode::Pool, code, level, seed, 60_000);
    let start = pool::build(level, seed);
    let mut out = json!({
        "compiled": raw.compiled.ok,
        "output": raw.compiled.output,
        "diagnostics": diagnostics_json(&raw.compiled),
        "map": map_json(&start),
    });
    if !raw.compiled.ok {
        return (out, None);
    }
    let mut parsed = parse(&raw.stdout);
    let crab_count = start.crabs.len();
    let mut world = start;
    let mut frames = vec![frame_json(&world)];
    let (mut swept, mut caught) = (0, 0);
    for acts in &parsed.actions {
        if acts.len() < crab_count || world.is_done() {
            break;
        }
        world.step(acts);
        swept += world.crabs.iter().filter(|c| c.mishap == Mishap::Swept).count();
        caught += world.crabs.iter().filter(|c| c.mishap == Mishap::Caught).count();
        frames.push(frame_json(&world));
    }
    let won = world.is_won();
    let message = if won {
        format!("성공! {}틱 만에 먹이 {}개를 모았어요.", world.tick, world.goal)
    } else if raw.timed_out {
        "시간 초과! tick 이 끝나지 않아요. 무한 루프인지 확인해 보세요.".to_string()
    } else if raw.crashed {
        for line in raw.stderr.lines() {
            parsed.logs.push(json!({ "tick": parsed.last.0, "crab": parsed.last.1, "text": line, "kind": "stderr" }));
        }
        format!("꽃게 {} 의 tick 이 {}틱에서 패닉했어요.", parsed.last.1, parsed.last.0)
    } else if world.tick >= world.max_ticks {
        format!("{}틱 안에 다 모으지 못했어요. ({}/{}개)", world.max_ticks, world.delivered, world.goal)
    } else {
        "시뮬레이션이 중간에 끝났어요.".to_string()
    };
    out["frames"] = Value::Array(frames);
    out["states"] = json!(parsed.states);
    out["actions"] = json!(parsed
        .actions
        .iter()
        .map(|acts| acts.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>())
        .collect::<Vec<_>>());
    out["logs"] = Value::Array(parsed.logs);
    out["won"] = json!(won);
    out["message"] = json!(message);
    out["stats"] = json!({
        "ticks": world.tick, "lines": code_lines(code), "delivered": world.delivered,
        "swept": swept, "caught": caught,
    });
    (out, Some(Outcome { won, message, world }))
}

// ── 레벨 검증 ────────────────────────────────────────────────────

/// 모든 레슨의 정답이 여러 시드에서 통과하고, 시작 코드는 통과하지 못하는지 확인한다.
pub fn verify() -> bool {
    let mut all_ok = true;
    if POOL_LESSONS.len() != LEVEL_COUNT {
        println!("FAIL  꽃게 AI 레슨 수({})와 레벨 수({})가 다름", POOL_LESSONS.len(), LEVEL_COUNT);
        return false;
    }
    for (i, lesson) in POOL_LESSONS.iter().enumerate() {
        if lesson.id != SPECS[i].id {
            println!("FAIL  {}번째 레슨 id 불일치: {} / {}", i + 1, lesson.id, SPECS[i].id);
            all_ok = false;
            continue;
        }
        let mut notes = Vec::new();
        let mut worst = 0;
        for seed in 1..=6u64 {
            let (json, outcome) = run_full(lesson.solution, i, seed);
            match outcome {
                None => {
                    notes.push(format!("정답 컴파일 실패\n{}", json["output"].as_str().unwrap_or("")));
                    break;
                }
                Some(o) if !o.won => {
                    notes.push(format!("정답 실패 (seed {seed}): {}", o.message));
                    break;
                }
                Some(o) => worst = worst.max(o.world.tick),
            }
        }
        match run_full(lesson.start, i, 1) {
            (json, None) => notes.push(format!("시작 코드 컴파일 실패\n{}", json["output"].as_str().unwrap_or(""))),
            (_, Some(o)) if o.won => notes.push("시작 코드가 그대로 통과해 버림".into()),
            _ => {}
        }
        if notes.is_empty() {
            println!(
                "ok    {:<8} {}  (정답 최대 {}틱 / 제한 {}틱, {}줄)",
                lesson.id,
                lesson.title,
                worst,
                SPECS[i].max_ticks,
                code_lines(lesson.solution)
            );
        } else {
            all_ok = false;
            println!("FAIL  {:<8} {}", lesson.id, lesson.title);
            for note in notes {
                println!("      {note}");
            }
        }
    }
    runner::cleanup();
    all_ok
}
