// 레벨 하나를 cargo 로 빌드하고 실행해서, 게임팩이 출력한 프레임들을 모은다.
//
// 게임팩 프로그램의 출력 (한 줄에 하나, "\u{1}crab " 다음에 JSON):
//   {"t":"start", "map":{…}}                  지도처럼 바뀌지 않는 정보
//   {"t":"frame", "tick":n, …}                한 틱의 모습 (0틱부터)
//   {"t":"think", "tick":n, "agent":i}        이제 i 번 행위자의 두뇌가 돈다 (이어지는 println! 은 i 의 것)
//   {"t":"agent", "tick":n, "agent":i, "action":"…", "state":"…"}   고른 행동과 {:?} 상태
//   {"t":"end", "won":bool, "message":"…", "stats":{…}}
// end 없이 끝나는 게임팩(플레이어가 main 을 직접 쓰는 퍼즐 등)은 frame 마다 "outcome":{won, message} 와
// "stats" 를 실어 보내고, 마지막 frame 의 것을 결과로 쓴다.
// 머리글이 없는 줄은 플레이어의 println! 이다 (바로 앞의 think 또는 frame 의 틱에 붙인다).

use crate::pack::Pack;
use serde_json::{json, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PREFIX: &str = "\u{1}crab ";
const RUN_TIMEOUT: Duration = Duration::from_secs(5);

/// rustc, cargo 같은 Rust 도구를 찾는다.
fn tool(name: &str) -> Command {
    // 앱을 Finder 에서 실행하면 PATH 에 cargo 경로가 없을 수 있다.
    if let Ok(home) = std::env::var("HOME") {
        let candidate = PathBuf::from(home).join(".cargo/bin").join(name);
        if Command::new(name).arg("--version").output().is_err() && candidate.exists() {
            return Command::new(candidate);
        }
    }
    Command::new(name)
}

/// 코드 줄 수: 빈 줄과 주석만 있는 줄은 세지 않는다.
pub fn code_lines(code: &str) -> usize {
    code.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .count()
}

struct Build {
    ok: bool,
    output: String,
    diagnostics: Vec<Value>,
    exe: Option<PathBuf>,
}

fn build(root: &Path, pack: &Pack, level_id: &str) -> Build {
    let mut b = Build { ok: false, output: String::new(), diagnostics: Vec::new(), exe: None };
    let result = tool("cargo")
        .current_dir(root)
        .args(["build", "--offline", "--quiet", "--message-format=json", "--bin", level_id, "--manifest-path"])
        .arg(root.join(&pack.id).join("Cargo.toml"))
        .stdin(Stdio::null())
        .output();
    let result = match result {
        Ok(r) => r,
        Err(e) => {
            b.output = format!("cargo 를 실행하지 못했어요: {e}\nRust 가 설치되어 있는지 확인해 주세요 (https://rustup.rs).");
            return b;
        }
    };
    for line in String::from_utf8_lossy(&result.stdout).lines() {
        let Ok(msg) = serde_json::from_str::<Value>(line) else { continue };
        match msg["reason"].as_str() {
            Some("compiler-message") => {
                let m = &msg["message"];
                if let Some(rendered) = m["rendered"].as_str() {
                    b.output.push_str(rendered);
                }
                let level = m["level"].as_str().unwrap_or("");
                if level == "error" || level == "warning" {
                    if let Some(span) = m["spans"].as_array().and_then(|s| s.iter().find(|s| s["is_primary"] == true)) {
                        b.diagnostics.push(json!({
                            "file": span["file_name"], "line": span["line_start"], "col": span["column_start"],
                            "level": level, "message": m["message"],
                        }));
                    }
                }
            }
            Some("compiler-artifact") => {
                if msg["target"]["name"].as_str() == Some(level_id) {
                    if let Some(exe) = msg["executable"].as_str() {
                        b.exe = Some(PathBuf::from(exe));
                    }
                }
            }
            _ => {}
        }
    }
    // cargo 자체의 에러 (Cargo.toml 이 망가졌다 등) 는 표준 에러로 나온다.
    let stderr = String::from_utf8_lossy(&result.stderr);
    if !result.status.success() && !stderr.trim().is_empty() {
        b.output.push_str(stderr.trim_end());
        b.output.push('\n');
    }
    b.output = b.output.trim_end().to_string();
    b.ok = result.status.success() && b.exe.is_some();
    b
}

struct Exec {
    stdout: String,
    stderr: String,
    timed_out: bool,
    crashed: bool,
}

fn exec(exe: &Path, root: &Path, level_id: &str, seed: u64) -> Result<Exec, String> {
    let mut child = Command::new(exe)
        .current_dir(root)
        .env("CRAB_HOST", "1")
        .env("CRAB_LEVEL", level_id)
        .env("CRAB_SEED", seed.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("프로그램을 실행하지 못했어요: {e}"))?;
    // 파이프가 가득 차서 멈추지 않도록 별도 스레드에서 읽는다.
    let mut out_pipe = child.stdout.take().unwrap();
    let mut err_pipe = child.stderr.take().unwrap();
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = err_pipe.read_to_end(&mut buf);
        buf
    });
    let started = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() > RUN_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                timed_out = true;
                break None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(_) => break None,
        }
    };
    Ok(Exec {
        stdout: String::from_utf8_lossy(&out_thread.join().unwrap_or_default()).to_string(),
        stderr: String::from_utf8_lossy(&err_thread.join().unwrap_or_default()).trim_end().to_string(),
        timed_out,
        crashed: !timed_out && !status.map(|s| s.success()).unwrap_or(false),
    })
}

/// 레벨의 처음 모습 (플레이어 코드가 컴파일되지 않을 때 보여 준다).
/// 게임팩 크레이트의 `preview` 실행 파일을 돌린다: preview <레벨> <시드> → start 와 첫 frame.
pub fn preview(root: &Path, pack: &Pack, level_id: &str, seed: u64) -> Value {
    let crate_dir = crate::workspace::pack_crate_dir(root, pack);
    let out = tool("cargo")
        .current_dir(&crate_dir)
        .args(["run", "--offline", "--quiet", "--bin", "preview", "--", level_id, &seed.to_string()])
        .stdin(Stdio::null())
        .output();
    let Ok(out) = out else {
        return json!({ "ok": false });
    };
    let mut map = Value::Null;
    let mut frames = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let Some(rest) = line.strip_prefix(PREFIX) else { continue };
        let Ok(msg) = serde_json::from_str::<Value>(rest) else { continue };
        match msg["t"].as_str() {
            Some("start") => map = msg["map"].clone(),
            Some("frame") => frames.push(msg),
            _ => {}
        }
    }
    json!({ "ok": !frames.is_empty(), "map": map, "frames": frames })
}

/// 레벨을 빌드하고 실행한 결과 (게임 화면이 그대로 쓰는 JSON).
pub fn build_and_run(root: &Path, pack: &Pack, level_id: &str, seed: u64) -> Value {
    let Some(level) = pack.level(level_id) else {
        return json!({ "compiled": false, "output": format!("'{level_id}' 라는 레벨이 없어요."), "diagnostics": [] });
    };
    let file = root.join(pack.level_file(level));
    let code = std::fs::read_to_string(&file).unwrap_or_default();
    let b = build(root, pack, level_id);
    let mut out = json!({
        "compiled": b.ok, "output": b.output, "diagnostics": b.diagnostics,
        "level": level_id, "file": file.to_string_lossy(),
    });
    let Some(exe) = b.exe.filter(|_| b.ok) else {
        let p = preview(root, pack, level_id, seed);
        if p["ok"].as_bool().unwrap_or(false) {
            out["preview"] = p;
        }
        return out;
    };
    let run = match exec(&exe, root, level_id, seed) {
        Ok(r) => r,
        Err(e) => {
            out["compiled"] = json!(false);
            out["output"] = json!(e);
            return out;
        }
    };

    let mut map = Value::Null;
    let mut frames: Vec<Value> = Vec::new();
    let mut actions: Vec<Vec<Value>> = Vec::new();
    let mut states: Vec<Vec<Value>> = Vec::new();
    let mut logs: Vec<Value> = Vec::new();
    let mut end = Value::Null;
    let mut last = (0u64, 0u64);
    for line in run.stdout.lines().take(200_000) {
        let Some(rest) = line.strip_prefix(PREFIX) else {
            logs.push(json!({ "tick": last.0, "agent": last.1, "text": line, "kind": "print" }));
            continue;
        };
        let Ok(msg) = serde_json::from_str::<Value>(rest) else { continue };
        match msg["t"].as_str() {
            Some("start") => map = msg["map"].clone(),
            Some("frame") => {
                last.0 = msg["tick"].as_u64().unwrap_or(last.0);
                frames.push(msg);
            }
            Some("think") => last = (msg["tick"].as_u64().unwrap_or(0), msg["agent"].as_u64().unwrap_or(0)),
            Some("agent") => {
                let (t, i) = (msg["tick"].as_u64().unwrap_or(0) as usize, msg["agent"].as_u64().unwrap_or(0) as usize);
                while actions.len() <= t {
                    actions.push(Vec::new());
                    states.push(Vec::new());
                }
                while actions[t].len() <= i {
                    actions[t].push(Value::Null);
                    states[t].push(Value::Null);
                }
                actions[t][i] = msg["action"].clone();
                states[t][i] = msg["state"].clone();
            }
            Some("end") => end = msg,
            _ => {}
        }
    }
    // end 가 없으면 마지막 frame 의 결과를 쓴다.
    if end.is_null() {
        if let Some(f) = frames.last().filter(|f| f["outcome"].is_object()) {
            end = json!({ "won": f["outcome"]["won"], "message": f["outcome"]["message"], "stats": f["stats"] });
        }
    }
    let noun = pack.agent_noun();
    let message = if run.timed_out {
        "시간 초과! 프로그램이 끝나지 않아요. 무한 루프인지 확인해 보세요.".to_string()
    } else if run.crashed {
        for l in run.stderr.lines() {
            logs.push(json!({ "tick": last.0, "agent": last.1, "text": l, "kind": "stderr" }));
        }
        if frames.is_empty() {
            "프로그램이 시작하자마자 멈췄어요.".to_string()
        } else {
            format!("{noun} #{} 의 두뇌가 {}틱에서 패닉했어요.", last.1, last.0)
        }
    } else if end.is_null() {
        "프로그램이 아무것도 하지 않고 끝났어요. main 에서 게임 함수를 불렀는지 확인해 보세요.".to_string()
    } else {
        end["message"].as_str().unwrap_or("").to_string()
    };
    if !run.stderr.is_empty() && !run.crashed {
        for l in run.stderr.lines() {
            logs.push(json!({ "tick": last.0, "agent": last.1, "text": l, "kind": "stderr" }));
        }
    }
    let mut stats = end["stats"].clone();
    if !stats.is_object() {
        stats = json!({});
    }
    stats["lines"] = json!(code_lines(&code));
    if frames.is_empty() {
        // 첫 모습도 나오기 전에 멈췄으면 미리 보기로 레벨의 처음 모습을 보여 준다.
        let p = preview(root, pack, level_id, seed);
        if p["ok"].as_bool().unwrap_or(false) {
            out["preview"] = p;
        }
    }
    out["map"] = map;
    out["frames"] = Value::Array(frames);
    out["actions"] = json!(actions);
    out["states"] = json!(states);
    out["logs"] = Value::Array(logs);
    out["won"] = json!(end["won"].as_bool().unwrap_or(false));
    out["message"] = json!(message);
    out["stats"] = stats;
    out
}
