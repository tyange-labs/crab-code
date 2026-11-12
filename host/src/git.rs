// git 명령을 부르는 얇은 층. 플레이어 컴퓨터의 git 을 그대로 쓴다.

use std::path::Path;
use std::process::Command;

fn git(root: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(root);
    // 이름·이메일이 어디에도 설정되지 않은 컴퓨터에서도 커밋이 되도록 기본값을 준다.
    if !configured(root, "user.name") {
        cmd.args(["-c", "user.name=Sprout Code"]);
    }
    if !configured(root, "user.email") {
        cmd.args(["-c", "user.email=crab-code@localhost"]);
    }
    cmd
}

/// 이 저장소에서 보이는 설정 (저장소 설정 → 전역 설정 순).
fn configured(root: &Path, key: &str) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["config", "--get", key])
        .output()
        .map(|o| o.status.success() && !o.stdout.is_empty())
        .unwrap_or(false)
}

/// 전역 설정의 이름·이메일 (작업 폴더를 만들 때 기본값으로 보여 준다).
pub fn global_identity() -> (String, String) {
    let get = |key: &str| {
        Command::new("git")
            .args(["config", "--global", "--get", key])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    };
    (get("user.name"), get("user.email"))
}

/// 작업 폴더 저장소에만 적용하는 설정: 커밋할 이름·이메일, 그리고 서명 끄기.
/// 혼자 보는 학습 기록이라 서명이 필요 없고, 서명 키 암호를 묻느라 게임이 멈추면 안 된다.
pub fn configure_local(root: &Path, identity: Option<(&str, &str)>) -> Result<(), String> {
    let set = |key: &str, value: &str| run(Command::new("git").arg("-C").arg(root).args(["config", "--local", key, value]));
    if let Some((name, email)) = identity {
        set("user.name", name)?;
        set("user.email", email)?;
    }
    set("commit.gpgsign", "false")?;
    set("tag.gpgsign", "false")?;
    Ok(())
}

fn run(cmd: &mut Command) -> Result<String, String> {
    let out = cmd.output().map_err(|e| format!("git 을 실행하지 못했어요: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

pub fn available() -> bool {
    // 검증처럼 기록이 필요 없을 때는 CRAB_NO_GIT 으로 끈다.
    if std::env::var("CRAB_NO_GIT").is_ok() {
        return false;
    }
    Command::new("git").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

pub fn init(root: &Path) -> Result<(), String> {
    run(git(root).args(["init", "-q"])).map(|_| ())
}

fn has_staged(root: &Path) -> bool {
    // diff --cached --quiet 은 바뀐 게 있으면 실패(1)로 끝난다.
    git(root).args(["diff", "--cached", "--quiet"]).status().map(|s| !s.success()).unwrap_or(false)
}

/// 전부 커밋한다. 바뀐 게 없으면 빈 문자열.
pub fn commit_all(root: &Path, message: &str) -> Result<String, String> {
    run(git(root).args(["add", "-A"]))?;
    commit_staged(root, message)
}

/// 주어진 경로만 커밋한다. 바뀐 게 없으면 빈 문자열.
pub fn commit_paths(root: &Path, paths: &[&str], message: &str) -> Result<String, String> {
    run(git(root).arg("add").arg("--").args(paths))?;
    commit_staged(root, message)
}

fn commit_staged(root: &Path, message: &str) -> Result<String, String> {
    if !has_staged(root) {
        return Ok(String::new());
    }
    run(git(root).args(["commit", "-q", "-m", message]))?;
    run(git(root).args(["rev-parse", "--short", "HEAD"]))
}

/// 태그를 단다 (이미 있으면 옮긴다). 서명 설정(tag.gpgSign)이 켜져 있어도 되도록 메시지를 붙인다.
pub fn tag(root: &Path, name: &str, message: &str) -> Result<(), String> {
    run(git(root).args(["tag", "-f", "-a", name, "-m", message])).map(|_| ())
}

/// 마지막 커밋: (짧은 해시, 제목, "3분 전" 같은 상대 시각)
pub fn last_commit(root: &Path) -> Option<(String, String, String)> {
    let line = run(git(root).args(["log", "-1", "--format=%h%x09%s%x09%ct"])).ok()?;
    let mut parts = line.splitn(3, '\t');
    let (hash, subject) = (parts.next()?.to_string(), parts.next()?.to_string());
    let when: u64 = parts.next()?.trim().parse().ok()?;
    Some((hash, subject, ago(when)))
}

fn ago(unix: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(unix);
    let s = now.saturating_sub(unix);
    match s {
        0..=44 => "방금".into(),
        45..=3599 => format!("{}분 전", (s + 30) / 60),
        3600..=86_399 => format!("{}시간 전", s / 3600),
        _ => format!("{}일 전", s / 86_400),
    }
}
