// Crab Code — 꽃게 페리스를 Rust 코드로 움직이며 배우는 퍼즐 게임.
//
// Tauri 앱이다. 이 파일(백엔드)은 플레이어 코드를 rustc 로 컴파일·실행하고 월드를 시뮬레이션한다.
// 화면(ui/ 폴더)은 웹뷰에서 돌며, 에디터는 Monaco 를 쓴다.
//
// 실행:        cargo run --release
// 레벨 검증:   cargo run --release -- --verify   (모든 정답/시작 코드를 실제로 컴파일해 본다)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lessons;
mod runner;
#[allow(dead_code)] // 일부 함수는 플레이어 프로그램 쪽에서만 쓴다.
mod sim;

use lessons::{CHAPTERS, LESSONS};
use runner::{Compiled, OutLine, RunResult};
use serde_json::{json, Value};
use sim::{Action, World, LEVEL_COUNT};
use std::path::PathBuf;

fn color_name(color: sim::Color) -> &'static str {
    match color {
        sim::Color::Red => "red",
        sim::Color::Blue => "blue",
    }
}

/// 화면이 그릴 수 있도록 월드를 JSON 으로 바꾼다.
fn world_json(w: &World) -> Value {
    json!({
        "w": w.w,
        "h": w.h,
        "walls": w.walls,
        "x": w.x,
        "y": w.y,
        "dir": w.dir,
        "gems": w.gems.iter().map(|g| json!({
            "x": g.x, "y": g.y, "color": color_name(g.color), "weight": g.weight,
            "shiny": g.shiny, "held": g.held, "placed": g.placed,
        })).collect::<Vec<_>>(),
        "pedestals": w.pedestals.iter().map(|p| json!({
            "x": p.x, "y": p.y, "want": p.want.map(color_name),
        })).collect::<Vec<_>>(),
        "signs": w.signs.iter().map(|(x, y, sign)| json!({
            "x": x, "y": y,
            "sign": match sign {
                sim::Sign::Left => "left",
                sim::Sign::Right => "right",
                sim::Sign::Finish => "finish",
                sim::Sign::Straight => "straight",
            },
        })).collect::<Vec<_>>(),
        "numbers": w.numbers.iter().map(|(x, y, n)| json!({ "x": x, "y": y, "n": n })).collect::<Vec<_>>(),
        "door": w.door.as_ref().map(|d| json!({ "x": d.x, "y": d.y, "open": d.open })),
        "flag": w.flag.map(|(x, y)| json!({ "x": x, "y": y })),
        "dial": w.dial,
        "scroll": w.scroll,
        "note": w.note,
        "note2": w.note2,
        "showWeights": w.show_weights,
    })
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

fn diagnostics_json(compiled: &Compiled) -> Value {
    compiled
        .diagnostics
        .iter()
        .map(|d| {
            json!({
                "line": d.line, "col": d.col, "endLine": d.end_line, "endCol": d.end_col,
                "message": d.message, "error": d.error,
            })
        })
        .collect()
}

/// 실행 결과를 재생하면서, 화면이 차례로 보여 줄 단계들과 최종 결과를 만든다.
/// 돌려주는 값: (단계 목록, 성공 여부, 결과 메시지)
fn replay(result: &RunResult, level: usize, seed: u64) -> (Vec<Value>, bool, String) {
    let mut world = sim::build_level(level, seed);
    let mut steps = Vec::new();
    for line in &result.lines {
        match line {
            OutLine::Print(text) => steps.push(json!({ "type": "print", "text": text })),
            OutLine::Limit => {
                return (steps, false, "너무 많이 움직였어요. 반복문이 끝나지 않는 건 아닐까요?".into());
            }
            OutLine::Act(action) => match world.apply(*action) {
                Ok(()) => steps.push(json!({
                    "type": "act", "action": action_name(*action), "world": world_json(&world),
                })),
                Err(message) => {
                    steps.push(json!({ "type": "bump", "forward": *action == Action::Forward }));
                    return (steps, false, message);
                }
            },
        }
    }
    if result.timed_out {
        return (steps, false, "시간 초과! 프로그램이 끝나지 않아요. 무한 루프인지 확인해 보세요.".into());
    }
    if result.crashed {
        for line in result.runtime_stderr.lines() {
            steps.push(json!({ "type": "stderr", "text": line }));
        }
        return (steps, false, "프로그램이 실행 중에 멈췄어요 (패닉).".into());
    }
    match world.todo_hint() {
        Some(todo) => (steps, false, format!("아직이에요. {todo}")),
        None => (steps, true, "성공! 목표를 달성했어요.".into()),
    }
}

// ── 화면에서 부르는 명령 ─────────────────────────────────────────

#[tauri::command]
fn get_lessons() -> Value {
    // 개발용 환경 변수: CRAB_START=레벨번호, CRAB_DEMO=1 (정답 자동 실행) 또는 start (시작 코드 실행).
    // 둘 중 하나라도 있으면 저장 파일을 읽지도 쓰지도 않는다.
    let start_level = std::env::var("CRAB_START").ok().and_then(|v| v.parse::<usize>().ok());
    let demo = std::env::var("CRAB_DEMO").ok();
    json!({
        "dev": start_level.is_some() || demo.is_some(),
        "startLevel": start_level,
        "demo": demo,
        "chapters": CHAPTERS.iter().map(|(short, name)| json!({ "short": short, "name": name })).collect::<Vec<_>>(),
        "lessons": LESSONS.iter().map(|l| json!({
            "id": l.id, "chapter": l.chapter, "title": l.title, "body": l.body,
            "start": l.start, "solution": l.solution, "hint": l.hint,
        })).collect::<Vec<_>>(),
    })
}

#[tauri::command]
fn get_world(level: usize, seed: u64) -> Value {
    world_json(&sim::build_level(level, seed))
}

/// 코드를 컴파일하고 실행한다. 오래 걸리므로 별도 스레드에서 돈다.
#[tauri::command(async)]
fn run_code(code: String, level: usize, seed: u64) -> Value {
    let result = runner::run(&code, level, seed);
    let mut out = json!({
        "compiled": result.compiled.ok,
        "output": result.compiled.output,
        "diagnostics": diagnostics_json(&result.compiled),
        "world": get_world(level, seed),
    });
    if result.compiled.ok {
        let (steps, won, message) = replay(&result, level, seed);
        out["steps"] = Value::Array(steps);
        out["won"] = json!(won);
        out["message"] = json!(message);
    }
    out
}

/// 실행하지 않고 컴파일 에러만 확인한다 (입력 중 밑줄 표시용).
#[tauri::command(async)]
fn check_code(code: String) -> Value {
    diagnostics_json(&runner::check(&code))
}

fn save_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(|home| PathBuf::from(home).join(".crab-code"))
}

#[tauri::command]
fn load_save() -> Value {
    let mut codes = serde_json::Map::new();
    let mut cleared: Vec<String> = Vec::new();
    if let Some(dir) = save_dir() {
        for lesson in LESSONS {
            if let Ok(code) = std::fs::read_to_string(dir.join(format!("{}.rs", lesson.id))) {
                codes.insert(lesson.id.to_string(), json!(code));
            }
        }
        if let Ok(ids) = std::fs::read_to_string(dir.join("cleared.txt")) {
            cleared = ids.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
        }
    }
    json!({ "codes": codes, "cleared": cleared })
}

#[tauri::command]
fn save_code(id: String, code: String) {
    // 알려진 레슨 id 만 받는다 (파일 이름으로 쓰이므로).
    if !LESSONS.iter().any(|l| l.id == id) {
        return;
    }
    if let Some(dir) = save_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join(format!("{id}.rs")), code);
    }
}

#[tauri::command]
fn save_cleared(ids: Vec<String>) {
    let known: Vec<&str> = ids.iter().map(|s| s.as_str()).filter(|id| LESSONS.iter().any(|l| l.id == *id)).collect();
    if let Some(dir) = save_dir() {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join("cleared.txt"), known.join("\n"));
    }
}

// ── 레벨 검증 (창 없이) ──────────────────────────────────────────

/// 모든 레슨의 정답이 통과하고, 시작 코드는 통과하지 못하는지 확인한다.
fn verify() -> bool {
    let mut all_ok = true;
    if LESSONS.len() != LEVEL_COUNT {
        println!("FAIL  레슨 수({})와 레벨 수({})가 다름", LESSONS.len(), LEVEL_COUNT);
        return false;
    }
    let wins = |code: &str, level: usize, seed: u64| -> Result<(), String> {
        let result = runner::run(code, level, seed);
        if !result.compiled.ok {
            return Err(format!("컴파일 실패\n{}", result.compiled.output));
        }
        let (_, won, message) = replay(&result, level, seed);
        if won {
            Ok(())
        } else {
            Err(message)
        }
    };
    for (i, lesson) in LESSONS.iter().enumerate() {
        if lesson.id != sim::SPECS[i].id {
            println!("FAIL  {}번째 레슨 id 불일치: {} / {}", i + 1, lesson.id, sim::SPECS[i].id);
            all_ok = false;
            continue;
        }
        let mut notes = Vec::new();
        for seed in 1..=6u64 {
            if let Err(e) = wins(lesson.solution, i, seed) {
                notes.push(format!("정답 실패 (seed {seed}): {e}"));
                break;
            }
        }
        let checked = runner::check(lesson.start);
        match lesson.start_error {
            Some(code) => {
                if checked.ok || !checked.output.contains(code) {
                    notes.push(format!("시작 코드가 {code} 에러를 내지 않음"));
                }
                if !checked.diagnostics.iter().any(|d| d.error) {
                    notes.push("에디터에 표시할 에러 위치를 찾지 못함".into());
                }
            }
            None => {
                if !checked.ok {
                    notes.push(format!("시작 코드 컴파일 실패\n{}", checked.output));
                } else if wins(lesson.start, i, 1).is_ok() {
                    notes.push("시작 코드가 그대로 통과해 버림".into());
                }
            }
        }
        if notes.is_empty() {
            println!("ok    {:<10} {}", lesson.id, lesson.title);
        } else {
            all_ok = false;
            println!("FAIL  {:<10} {}", lesson.id, lesson.title);
            for note in notes {
                println!("      {note}");
            }
        }
    }
    runner::cleanup();
    all_ok
}

fn main() {
    if std::env::args().any(|a| a == "--verify") {
        std::process::exit(if verify() { 0 } else { 1 });
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_lessons,
            get_world,
            run_code,
            check_code,
            load_save,
            save_code,
            save_cleared
        ])
        .build(tauri::generate_context!())
        .expect("Tauri 앱을 시작하지 못했습니다")
        .run(|_, event| {
            if let tauri::RunEvent::Exit = event {
                runner::cleanup();
            }
        });
}
