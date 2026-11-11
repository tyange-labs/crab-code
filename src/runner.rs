// 플레이어 코드를 진짜 rustc 로 컴파일하고 실행해서, 행동 목록을 돌려준다.

use crate::sim::Action;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const SIM_SRC: &str = include_str!("sim.rs");
const CRAB_SRC: &str = include_str!("../runtime/crab.rs");
const MAIN_SRC: &str = "#[allow(warnings)]\nmod sim;\n#[allow(warnings)]\nmod crab;\n#[allow(unused_imports)]\nuse crab::*;\ninclude!(\"user.rs\");\n";
const RUN_TIMEOUT: Duration = Duration::from_secs(4);

#[derive(Clone, Debug)]
pub enum OutLine {
    Act(Action),
    /// 행동 횟수 제한을 넘었다 (무한 루프 의심).
    Limit,
    Print(String),
}

#[derive(Debug, Default)]
pub struct RunResult {
    pub compiled: bool,
    /// 컴파일러 출력 (에러 또는 경고).
    pub compiler_output: String,
    pub lines: Vec<OutLine>,
    pub timed_out: bool,
    /// 실행 중 표준 에러 (패닉 메시지 등).
    pub runtime_stderr: String,
    pub crashed: bool,
}

fn work_dir() -> PathBuf {
    std::env::temp_dir().join(format!("crab-code-{}", std::process::id()))
}

fn rustc_command() -> Command {
    // 터미널이 아닌 곳에서 실행되면 PATH 에 cargo 경로가 없을 수 있다.
    if let Ok(home) = std::env::var("HOME") {
        let candidate = PathBuf::from(home).join(".cargo/bin/rustc");
        if Command::new("rustc").arg("--version").output().is_err() && candidate.exists() {
            return Command::new(candidate);
        }
    }
    Command::new("rustc")
}

pub fn run(code: &str, level: usize, seed: u64) -> RunResult {
    let mut result = RunResult::default();
    let dir = work_dir();
    let setup = std::fs::create_dir_all(&dir)
        .and_then(|_| std::fs::write(dir.join("sim.rs"), SIM_SRC))
        .and_then(|_| std::fs::write(dir.join("crab.rs"), CRAB_SRC))
        .and_then(|_| std::fs::write(dir.join("main.rs"), MAIN_SRC))
        .and_then(|_| std::fs::write(dir.join("user.rs"), code));
    if let Err(e) = setup {
        result.compiler_output = format!("임시 폴더를 준비하지 못했어요: {e}");
        return result;
    }

    let exe = dir.join(if cfg!(windows) { "prog.exe" } else { "prog" });
    let compile = rustc_command()
        .current_dir(&dir)
        .args(["--edition", "2021", "--color", "never", "-C", "debuginfo=0", "-o"])
        .arg(&exe)
        .arg("main.rs")
        .output();
    let compile = match compile {
        Ok(out) => out,
        Err(e) => {
            result.compiler_output = format!(
                "rustc 를 실행하지 못했어요: {e}\nRust 가 설치되어 있는지 확인해 주세요 (https://rustup.rs)."
            );
            return result;
        }
    };
    result.compiler_output = String::from_utf8_lossy(&compile.stderr).trim_end().to_string();
    if !compile.status.success() {
        return result;
    }
    result.compiled = true;

    let child = Command::new(&exe)
        .current_dir(&dir)
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
            result.runtime_stderr = format!("프로그램을 실행하지 못했어요: {e}");
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
    result.runtime_stderr = String::from_utf8_lossy(&stderr).trim_end().to_string();
    result.crashed = !result.timed_out && !status.map(|s| s.success()).unwrap_or(false);

    for line in String::from_utf8_lossy(&stdout).lines().take(5000) {
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

pub fn cleanup() {
    let _ = std::fs::remove_dir_all(work_dir());
}

/// 컴파일러 출력에서 플레이어 코드의 첫 에러 줄 번호(1부터)를 찾는다.
pub fn first_error_line(output: &str) -> Option<usize> {
    let idx = output.find("user.rs:")?;
    let rest = &output[idx + "user.rs:".len()..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}
