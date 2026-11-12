// 플레이어 코드를 진짜 rustc 로 컴파일하고 실행해서, 행동 목록을 돌려준다.

use crate::sim::Action;
use serde_json::Value;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const SIM_SRC: &str = include_str!("sim.rs");
const CRAB_SRC: &str = include_str!("../runtime/crab.rs");
const MAIN_SRC: &str = "#[allow(warnings)]\nmod sim;\n#[allow(warnings)]\nmod crab;\n#[allow(unused_imports)]\nuse crab::*;\ninclude!(\"user.rs\");\n";
const POOL_SRC: &str = include_str!("pool.rs");
const POOL_CRAB_SRC: &str = include_str!("../runtime/pool_crab.rs");
// 꽃게 AI 모드: 플레이어 코드에는 main 이 없고, 플레이어가 만든 Ferris 를 run 에 넘긴다.
const POOL_MAIN_SRC: &str = "#[allow(warnings)]\nmod pool;\n#[allow(warnings)]\nmod crab;\n#[allow(unused_imports)]\nuse crab::*;\ninclude!(\"user.rs\");\nfn main() {\n    crab::run::<Ferris>();\n}\n";

/// 어떤 게임을 위한 코드인가. 함께 컴파일할 파일이 다르다.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// 꽃게 한 마리를 명령으로 움직이는 퍼즐.
    Puzzle,
    /// Crab 트레이트를 구현해 꽃게들의 두뇌를 만드는 조수 웅덩이.
    Pool,
}

impl Mode {
    fn files(self) -> [(&'static str, &'static str); 3] {
        match self {
            Mode::Puzzle => [("sim.rs", SIM_SRC), ("crab.rs", CRAB_SRC), ("main.rs", MAIN_SRC)],
            Mode::Pool => [("pool.rs", POOL_SRC), ("crab.rs", POOL_CRAB_SRC), ("main.rs", POOL_MAIN_SRC)],
        }
    }

    fn dir_name(self, kind: &str) -> String {
        match self {
            Mode::Puzzle => kind.to_string(),
            Mode::Pool => format!("pool-{kind}"),
        }
    }
}
const RUN_TIMEOUT: Duration = Duration::from_secs(4);

// 실행과 입력 중 검사가 서로의 파일을 덮어쓰지 않도록 폴더와 잠금을 따로 둔다.
static RUN_LOCK: Mutex<()> = Mutex::new(());
static CHECK_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug)]
pub enum OutLine {
    Act(Action),
    /// 행동 횟수 제한을 넘었다 (무한 루프 의심).
    Limit,
    Print(String),
}

/// 에디터에 표시할 컴파일러 진단. 줄과 칸은 1부터 센다.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub message: String,
    pub error: bool,
}

#[derive(Debug, Default)]
pub struct Compiled {
    pub ok: bool,
    /// 사람이 읽는 컴파일러 출력 (에러 또는 경고).
    pub output: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Default)]
pub struct RunResult {
    pub compiled: Compiled,
    pub lines: Vec<OutLine>,
    pub timed_out: bool,
    /// 실행 중 표준 에러 (패닉 메시지 등).
    pub runtime_stderr: String,
    pub crashed: bool,
}

fn work_dir(kind: &str) -> PathBuf {
    std::env::temp_dir().join(format!("crab-code-{}", std::process::id())).join(kind)
}

/// rustc, rustfmt 같은 Rust 도구를 찾는다.
fn tool_command(name: &str) -> Command {
    // 앱을 Finder 에서 실행하면 PATH 에 cargo 경로가 없을 수 있다.
    if let Ok(home) = std::env::var("HOME") {
        let candidate = PathBuf::from(home).join(".cargo/bin").join(name);
        if Command::new(name).arg("--version").output().is_err() && candidate.exists() {
            return Command::new(candidate);
        }
    }
    Command::new(name)
}

fn rustc_command() -> Command {
    tool_command("rustc")
}

/// rustfmt 로 코드를 정리한다. 실패하면 rustfmt 의 에러 메시지를 돌려준다.
pub fn format(code: &str) -> Result<String, String> {
    use std::io::Write;
    let mut child = tool_command("rustfmt")
        .args(["--edition", "2021", "--emit", "stdout", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("rustfmt 를 실행하지 못했어요: {e}"))?;
    let mut stdin = child.stdin.take().unwrap();
    let input = code.to_string();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(input.as_bytes());
    });
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let _ = writer.join();
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim_end().to_string())
    }
}

/// rustc 의 JSON 진단 한 줄을 읽어 출력 글과 에디터용 진단으로 나눈다.
fn read_diagnostic(json: &Value, out: &mut Compiled) {
    if let Some(rendered) = json["rendered"].as_str() {
        out.output.push_str(rendered);
    }
    let level = json["level"].as_str().unwrap_or("");
    if level != "error" && level != "warning" {
        return;
    }
    let spans = json["spans"].as_array().cloned().unwrap_or_default();
    let in_user = |s: &&Value| s["file_name"].as_str() == Some("user.rs");
    let Some(span) = spans.iter().filter(in_user).find(|s| s["is_primary"] == true).or(spans.iter().find(in_user)) else {
        return;
    };
    let mut message = json["message"].as_str().unwrap_or("").to_string();
    if let Some(code) = json["code"]["code"].as_str() {
        message = format!("{message} [{code}]");
    }
    if let Some(label) = span["label"].as_str() {
        message.push_str(&format!("\n{label}"));
    }
    for child in json["children"].as_array().into_iter().flatten() {
        if let (Some(level), Some(text)) = (child["level"].as_str(), child["message"].as_str()) {
            message.push_str(&format!("\n{level}: {text}"));
        }
    }
    let num = |key: &str| span[key].as_u64().unwrap_or(1) as usize;
    out.diagnostics.push(Diagnostic {
        line: num("line_start"),
        col: num("column_start"),
        end_line: num("line_end"),
        end_col: num("column_end"),
        message,
        error: level == "error",
    });
}

/// 컴파일한다. check_only 면 실행 파일을 만들지 않고 검사만 한다 (더 빠르다).
fn compile(mode: Mode, kind: &str, code: &str, check_only: bool) -> (Compiled, PathBuf) {
    let mut out = Compiled::default();
    let dir = work_dir(&mode.dir_name(kind));
    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    let mut setup = std::fs::create_dir_all(&dir);
    for (name, src) in mode.files() {
        setup = setup.and_then(|_| std::fs::write(dir.join(name), src));
    }
    let setup = setup.and_then(|_| std::fs::write(dir.join("user.rs"), code));
    if let Err(e) = setup {
        out.output = format!("임시 폴더를 준비하지 못했어요: {e}");
        return (out, exe);
    }

    let mut cmd = rustc_command();
    cmd.current_dir(&dir)
        .args(["--edition", "2021", "--error-format=json", "-C", "debuginfo=0"]);
    if check_only {
        cmd.arg("--emit=metadata").arg("-o").arg(dir.join("check.rmeta"));
    } else {
        cmd.arg("-o").arg(&exe);
    }
    let result = match cmd.arg("main.rs").output() {
        Ok(result) => result,
        Err(e) => {
            out.output = format!(
                "rustc 를 실행하지 못했어요: {e}\nRust 가 설치되어 있는지 확인해 주세요 (https://rustup.rs)."
            );
            return (out, exe);
        }
    };
    for line in String::from_utf8_lossy(&result.stderr).lines() {
        match serde_json::from_str::<Value>(line) {
            Ok(json) => read_diagnostic(&json, &mut out),
            Err(_) => {
                out.output.push_str(line);
                out.output.push('\n');
            }
        }
    }
    out.output = out.output.trim_end().to_string();
    out.ok = result.status.success();
    (out, exe)
}

/// 실행하지 않고 컴파일 에러만 확인한다 (입력 중 밑줄 표시용).
pub fn check(code: &str) -> Compiled {
    check_mode(Mode::Puzzle, code)
}

pub fn check_mode(mode: Mode, code: &str) -> Compiled {
    let _guard = CHECK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    compile(mode, "check", code, true).0
}

/// 프로그램을 실행한 날것의 결과: 표준 출력 줄들.
#[derive(Debug, Default)]
pub struct RawRun {
    pub compiled: Compiled,
    pub stdout: Vec<String>,
    pub stderr: String,
    pub timed_out: bool,
    pub crashed: bool,
}

/// 퍼즐 모드: 실행해서 행동 목록으로 바꾼다.
pub fn run(code: &str, level: usize, seed: u64) -> RunResult {
    let raw = run_raw(Mode::Puzzle, code, level, seed, 5000);
    let mut result = RunResult {
        compiled: raw.compiled,
        timed_out: raw.timed_out,
        runtime_stderr: raw.stderr,
        crashed: raw.crashed,
        ..Default::default()
    };
    for line in &raw.stdout {
        match line.strip_prefix("@@") {
            Some("X") => result.lines.push(OutLine::Limit),
            Some(rest) => {
                if let Some(action) = Action::decode(rest) {
                    result.lines.push(OutLine::Act(action));
                }
            }
            None => result.lines.push(OutLine::Print(line.to_string())),
        }
    }
    result
}

/// 컴파일하고 실행한다. 표준 출력은 max_lines 줄까지만 받는다.
pub fn run_raw(mode: Mode, code: &str, level: usize, seed: u64, max_lines: usize) -> RawRun {
    let _guard = RUN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let (compiled, exe) = compile(mode, "run", code, false);
    let mut result = RawRun {
        compiled,
        ..Default::default()
    };
    if !result.compiled.ok {
        return result;
    }

    let child = Command::new(&exe)
        .current_dir(work_dir(&mode.dir_name("run")))
        .env("CRAB_LEVEL", level.to_string())
        .env("CRAB_SEED", seed.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            result.crashed = true;
            result.stderr = format!("프로그램을 실행하지 못했어요: {e}");
            return result;
        }
    };

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
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                if started.elapsed() > RUN_TIMEOUT {
                    let _ = child.kill();
                    let _ = child.wait();
                    result.timed_out = true;
                    break None;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(_) => break None,
        }
    };
    let stdout = out_thread.join().unwrap_or_default();
    let stderr = err_thread.join().unwrap_or_default();
    result.stderr = String::from_utf8_lossy(&stderr).trim_end().to_string();
    result.crashed = !result.timed_out && !status.map(|s| s.success()).unwrap_or(false);
    result.stdout = String::from_utf8_lossy(&stdout).lines().take(max_lines).map(String::from).collect();
    result
}

pub fn cleanup() {
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join(format!("crab-code-{}", std::process::id())));
}
