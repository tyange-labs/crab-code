// 게임 본체가 쓰는 기능을 한데 모은 곳.
// Godot 확장(godot-ext)과 레벨 검증 도구(bin/verify.rs)가 이 함수들을 부른다.
// 화면 쪽과는 JSON 문자열로 주고받는다.

use crate::lessons::{CHAPTERS, LESSONS};
use crate::runner::{self, Compiled, OutLine, RunResult};
use crate::sim::{self, Action, World, LEVEL_COUNT};
use serde_json::{json, Value};

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

pub(crate) fn diagnostics_json(compiled: &Compiled) -> Value {
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

// ── 화면에서 부르는 기능 ─────────────────────────────────────────

/// 장 목록과 레슨 목록.
pub fn lessons_json() -> Value {
    json!({
        "chapters": CHAPTERS.iter().map(|(short, name)| json!({ "short": short, "name": name })).collect::<Vec<_>>(),
        "lessons": LESSONS.iter().map(|l| json!({
            "id": l.id, "chapter": l.chapter, "title": l.title, "body": l.body,
            "start": l.start, "solution": l.solution, "hint": l.hint,
        })).collect::<Vec<_>>(),
    })
}

pub fn get_world(level: usize, seed: u64) -> Value {
    world_json(&sim::build_level(level, seed))
}

/// 코드를 컴파일하고 실행한다. 오래 걸리므로 별도 스레드에서 부른다.
pub fn run_code(code: &str, level: usize, seed: u64) -> Value {
    let result = runner::run(code, level, seed);
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
pub fn check_code(code: &str) -> Value {
    diagnostics_json(&runner::check(code))
}

/// rustfmt 로 정리한 코드. 실패하면 에러 메시지.
pub fn format_code(code: &str) -> Value {
    match runner::format(code) {
        Ok(code) => json!({ "ok": true, "code": code }),
        Err(error) => json!({ "ok": false, "error": error }),
    }
}

pub fn cleanup() {
    runner::cleanup();
}

// ── 레벨 검증 (창 없이) ──────────────────────────────────────────

/// 모든 레슨의 정답이 통과하고, 시작 코드는 통과하지 못하는지 확인한다.
pub fn verify() -> bool {
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
