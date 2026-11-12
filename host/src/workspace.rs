// 작업 폴더: 플레이어가 IDE 로 여는 진짜 cargo 워크스페이스이자 git 저장소.
//
//   ~/CrabCode/
//     Cargo.toml              [workspace] (게임팩마다 멤버 하나)
//     .crab/packs/<팩>/       게임팩 크레이트 사본 (게임이 관리한다, git 에서 뺀다)
//     <팩>/Cargo.toml         레벨마다 [[bin]] 하나
//     <팩>/levels/04_tide.rs  플레이어가 고치는 파일
//     .crab/progress.json     클리어 기록 (git 에 남는다)

use crate::git;
use crate::pack::Pack;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn default_root() -> PathBuf {
    if let Ok(dir) = std::env::var("CRAB_WORKSPACE") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("CrabCode")
}

fn write_if_changed(path: &Path, content: &str) -> std::io::Result<()> {
    if std::fs::read_to_string(path).map(|old| old == content).unwrap_or(false) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "target" || name == ".gdignore" {
            continue;
        }
        let (src, dst) = (entry.path(), to.join(&name));
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else if std::fs::read(&src).ok() != std::fs::read(&dst).ok() {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

const GITIGNORE: &str = "/target\n/.crab/packs/\n";

const README: &str = "\
# Crab Code 작업 폴더

Crab Code 게임이 만든 cargo 워크스페이스예요. 이 폴더를 좋아하는 IDE(VS Code, RustRover, Zed …)로 여세요.

- 게임팩마다 폴더가 하나씩 있고, `levels/` 안의 파일 하나가 레벨 하나예요.
- 파일을 저장하면 게임 창이 바로 빌드하고 실행해서 보여 줘요.
- 터미널에서 직접 돌려 볼 수도 있어요: `cargo run --bin tick`
- 레벨을 깨면 게임이 git 에 커밋을 남겨요. `git log` 로 지금까지의 풀이를 볼 수 있어요.
- `.crab/packs/` 는 게임팩의 소스예요. IDE 에서 F12(정의로 이동)를 누르면 여기로 와요. 게임이 관리하니 고치지 마세요.
";

const VSCODE_EXTENSIONS: &str = "{\n  \"recommendations\": [\"rust-lang.rust-analyzer\"]\n}\n";

pub fn pack_crate_dir(root: &Path, pack: &Pack) -> PathBuf {
    root.join(".crab").join("packs").join(&pack.id)
}

fn play_manifest(pack: &Pack) -> String {
    let mut s = format!(
        "# Crab Code 가 만든 파일이에요. 게임팩: {}\n[package]\nname = \"{}-play\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\nautobins = false\n\n[dependencies]\n{} = {{ path = \"../.crab/packs/{}\" }}\n",
        pack.name(),
        pack.id,
        pack.crate_name,
        pack.id
    );
    for level in &pack.levels {
        s.push_str(&format!("\n[[bin]]\nname = \"{}\"\npath = \"levels/{}.rs\"\n", level.id, level.dir_name));
    }
    s
}

fn root_manifest(root: &Path) -> String {
    let mut members: Vec<String> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("Cargo.toml").exists() && !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    members.sort();
    let list = members.iter().map(|m| format!("\"{m}\"")).collect::<Vec<_>>().join(", ");
    format!(
        "# Crab Code 가 만든 파일이에요.\n[workspace]\nresolver = \"2\"\nmembers = [{list}]\n\n# 저장할 때마다 빌드하니 빠르게.\n[profile.dev]\ndebug = 0\n"
    )
}

/// 작업 폴더를 준비한다: 없으면 만들고, 게임팩 크레이트를 최신으로 맞추고, 없는 레벨 파일을 만든다.
/// 처음 만들 때는 git 저장소도 만들고 첫 커밋을 남긴다.
/// identity 는 이 저장소의 커밋에만 쓸 (이름, 이메일). 서명은 언제나 끈다.
pub fn ensure(root: &Path, pack: &Pack, identity: Option<(&str, &str)>) -> Result<Value, String> {
    let err = |e: std::io::Error| format!("작업 폴더({})를 준비하지 못했어요: {e}", root.display());
    let fresh = !root.join(".git").exists();
    std::fs::create_dir_all(root).map_err(err)?;
    copy_dir(&pack.dir.join("crate"), &pack_crate_dir(root, pack)).map_err(err)?;
    write_if_changed(&root.join(pack.id.as_str()).join("Cargo.toml"), &play_manifest(pack)).map_err(err)?;
    let mut created = Vec::new();
    for level in &pack.levels {
        let file = root.join(pack.level_file(level));
        if !file.exists() {
            write_if_changed(&file, &level.start).map_err(err)?;
            created.push(level.id.clone());
        }
    }
    write_if_changed(&root.join("Cargo.toml"), &root_manifest(root)).map_err(err)?;
    if !root.join(".gitignore").exists() {
        write_if_changed(&root.join(".gitignore"), GITIGNORE).map_err(err)?;
    }
    if !root.join("README.md").exists() {
        write_if_changed(&root.join("README.md"), README).map_err(err)?;
    }
    if !root.join(".vscode/extensions.json").exists() {
        write_if_changed(&root.join(".vscode/extensions.json"), VSCODE_EXTENSIONS).map_err(err)?;
    }
    let mut note = String::new();
    if git::available() {
        if fresh {
            git::init(root)?;
        }
        git::configure_local(root, identity)?;
        if fresh {
            git::commit_all(root, "Crab Code 작업 폴더를 만들었어요")?;
        } else if !created.is_empty() {
            git::commit_paths(root, &[pack.id.as_str()], &format!("{}: 새 레벨 {}개", pack.name(), created.len()))?;
        }
    } else {
        note = "git 이 없어서 기록을 커밋하지 못해요. git 을 설치하면 클리어할 때마다 풀이가 남아요.".into();
    }
    let files: serde_json::Map<String, Value> = pack
        .levels
        .iter()
        .map(|l| (l.id.clone(), json!(root.join(pack.level_file(l)).to_string_lossy())))
        .collect();
    Ok(json!({ "ok": true, "root": root.to_string_lossy(), "files": files, "note": note, "fresh": fresh }))
}

/// 레벨 파일을 처음 코드로 되돌린다. 지금 코드는 먼저 커밋해 둔다 (git 으로 언제든 되찾을 수 있게).
pub fn reset_level(root: &Path, pack: &Pack, level_id: &str) -> Result<Value, String> {
    let level = pack.level(level_id).ok_or("그런 레벨이 없어요")?;
    let rel = pack.level_file(level);
    let file = root.join(&rel);
    if git::available() {
        let _ = git::commit_paths(root, &[rel.to_string_lossy().as_ref()], &format!("{} — 처음 코드로 되돌리기 전", level.title));
    }
    std::fs::write(&file, &level.start).map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true, "file": file.to_string_lossy() }))
}

// ── 클리어 기록 ───────────────────────────────────────────────────

fn progress_path(root: &Path) -> PathBuf {
    root.join(".crab").join("progress.json")
}

pub fn progress(root: &Path) -> Value {
    std::fs::read_to_string(progress_path(root))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| json!({}))
}

/// 클리어를 기록하고 커밋한다. 최고 기록이면 태그도 단다.
pub fn record_clear(root: &Path, pack: &Pack, level_id: &str, ticks: u64, lines: u64) -> Result<Value, String> {
    let level = pack.level(level_id).ok_or("그런 레벨이 없어요")?;
    let mut all = progress(root);
    let entry = &mut all[pack.id.as_str()][level_id];
    let prev_ticks = entry["best"]["ticks"].as_u64();
    let prev_lines = entry["best"]["lines"].as_u64();
    let better = match (prev_ticks, prev_lines) {
        (Some(t), Some(l)) => ticks < t || (ticks == t && lines < l),
        _ => true,
    };
    entry["cleared"] = json!(true);
    if better {
        entry["best"] = json!({ "ticks": ticks, "lines": lines });
    }
    let text = serde_json::to_string_pretty(&all).unwrap_or_default();
    write_if_changed(&progress_path(root), &text).map_err(|e| e.to_string())?;
    let mut commit = Value::Null;
    if git::available() {
        let index = pack.levels.iter().position(|l| l.id == level_id).unwrap_or(0) + 1;
        let unit = pack.meta["tick"].as_str().unwrap_or("틱");
        let msg = format!("{} {} 클리어 — {}{unit}, {}줄{}", pack.name(), index, ticks, lines, if better && prev_ticks.is_some() { " (최고 기록)" } else { "" });
        let rel = pack.level_file(level);
        let paths = [rel.to_string_lossy().to_string(), ".crab/progress.json".to_string()];
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        commit = match git::commit_paths(root, &path_refs, &msg) {
            Ok(hash) => {
                if better {
                    let _ = git::tag(root, &format!("best/{}/{}", pack.id, level_id), &msg);
                }
                json!({ "ok": true, "hash": hash, "message": msg })
            }
            Err(e) => json!({ "ok": false, "error": e }),
        };
    }
    Ok(json!({ "better": better, "previous": prev_ticks.map(|t| json!({ "ticks": t, "lines": prev_lines })), "commit": commit }))
}
