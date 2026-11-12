// 게임팩 읽기.
//
// 게임팩 폴더 구조:
//   pack.json                 id, name(이름), crate(크레이트 이름), tagline, agent(행위자 이름), colors
//   crate/                    세계 규칙과 플레이어용 API (std 만 쓰는 Rust 크레이트)
//   levels/NN_<id>/           레벨마다 lesson.md(첫 줄 "# 제목"), start.rs, solution.rs, hint.md
//   view/view.gd              그 세계를 그리는 Godot 화면 (게임 쪽에서만 쓴다)

use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Level {
    /// 실행 파일 이름이자 레벨 이름 (예: "tide").
    pub id: String,
    /// 폴더 이름 (예: "04_tide"). 작업 폴더의 파일 이름으로도 쓴다.
    pub dir_name: String,
    pub title: String,
    pub lesson: String,
    pub hint: String,
    pub start: String,
    pub solution: String,
    /// 처음 코드가 일부러 내는 컴파일 에러 코드 (예: "E0384"). 그 에러를 읽고 고치는 레벨.
    pub start_error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Pack {
    pub id: String,
    pub crate_name: String,
    pub dir: PathBuf,
    pub meta: Value,
    pub levels: Vec<Level>,
}

impl Pack {
    pub fn name(&self) -> &str {
        self.meta["name"].as_str().unwrap_or(&self.id)
    }

    /// 행위자를 부르는 이름 (예: "꽃게").
    pub fn agent_noun(&self) -> &str {
        self.meta["agent"].as_str().unwrap_or("행위자")
    }

    pub fn level(&self, id: &str) -> Option<&Level> {
        self.levels.iter().find(|l| l.id == id)
    }

    /// 작업 폴더 안에서 이 레벨의 코드 파일 (작업 폴더 기준 상대 경로).
    pub fn level_file(&self, level: &Level) -> PathBuf {
        PathBuf::from(&self.id).join("levels").join(format!("{}.rs", level.dir_name))
    }

    pub fn to_json(&self) -> Value {
        let mut meta = self.meta.clone();
        meta["levels"] = Value::Array(
            self.levels
                .iter()
                .map(|l| {
                    json!({
                        "id": l.id, "title": l.title, "lesson": l.lesson, "hint": l.hint,
                        "solution": l.solution, "file": self.level_file(l).to_string_lossy(),
                    })
                })
                .collect(),
        );
        meta
    }
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{} 을(를) 읽지 못했어요: {e}", path.display()))
}

pub fn load(dir: &Path) -> Result<Pack, String> {
    let meta: Value = serde_json::from_str(&read(&dir.join("pack.json"))?)
        .map_err(|e| format!("{}/pack.json 이 올바른 JSON 이 아니에요: {e}", dir.display()))?;
    let id = meta["id"].as_str().ok_or("pack.json 에 id 가 없어요")?.to_string();
    let crate_name = meta["crate"].as_str().unwrap_or(&id).to_string();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(dir.join("levels"))
        .map_err(|e| format!("{}/levels 를 읽지 못했어요: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    let mut levels = Vec::new();
    for d in dirs {
        let dir_name = d.file_name().unwrap().to_string_lossy().to_string();
        let level_id = dir_name.split_once('_').map(|(_, rest)| rest).unwrap_or(&dir_name).to_string();
        let lesson = read(&d.join("lesson.md"))?;
        let (title, body) = match lesson.strip_prefix("# ") {
            Some(rest) => {
                let (t, b) = rest.split_once('\n').unwrap_or((rest, ""));
                (t.trim().to_string(), b.trim().to_string())
            }
            None => (level_id.clone(), lesson.trim().to_string()),
        };
        levels.push(Level {
            id: level_id,
            dir_name,
            title,
            lesson: body,
            hint: read(&d.join("hint.md")).unwrap_or_default().trim().to_string(),
            start: read(&d.join("start.rs"))?,
            solution: read(&d.join("solution.rs")).unwrap_or_default(),
            start_error: read(&d.join("start_error.txt")).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        });
    }
    Ok(Pack { id, crate_name, dir: dir.to_path_buf(), meta, levels })
}

/// packs_dir 아래의 게임팩들 (읽지 못한 팩은 건너뛴다).
pub fn list(packs_dir: &Path) -> Vec<Pack> {
    let mut packs: Vec<Pack> = std::fs::read_dir(packs_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("pack.json").exists())
        .filter_map(|e| load(&e.path()).ok())
        .collect();
    packs.sort_by(|a, b| a.id.cmp(&b.id));
    packs
}

pub fn find(packs_dir: &Path, id: &str) -> Result<Pack, String> {
    load(&packs_dir.join(id))
}
