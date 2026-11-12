// crab-host 를 Godot 에 연결하는 확장.
//
// GDScript 에서 `CrabCore.new()` 로 만들어 쓴다. 값은 JSON 문자열로 주고받는다.
// 빌드·실행은 오래 걸리므로 `start_run` 이 백그라운드 스레드를 띄우고,
// 화면은 매 프레임 `poll_run` 으로 결과가 나왔는지 확인한다.

use crab_host::{git, pack, run, watch::Watcher, workspace};
use godot::prelude::*;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct CrabExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CrabExtension {}

/// 백그라운드 작업 하나의 결과를 담는 칸. 마지막으로 시작한 작업의 결과만 남긴다.
#[derive(Default, Clone)]
struct Slot(Arc<Mutex<(u64, Option<String>)>>);

impl Slot {
    fn start(&self, job: impl FnOnce() -> Value + Send + 'static) -> u64 {
        let token = {
            let mut s = self.0.lock().unwrap();
            s.0 += 1;
            s.1 = None;
            s.0
        };
        let slot = self.0.clone();
        std::thread::spawn(move || {
            let mut out = job();
            out["token"] = json!(token);
            let mut s = slot.lock().unwrap();
            if s.0 == token {
                s.1 = Some(out.to_string());
            }
        });
        token
    }

    fn take(&self) -> String {
        self.0.lock().unwrap().1.take().unwrap_or_default()
    }
}

fn gs(v: &Value) -> GString {
    GString::from(&v.to_string())
}

fn err(e: impl ToString) -> GString {
    gs(&json!({ "ok": false, "error": e.to_string() }))
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct CrabCore {
    run_slot: Slot,
    watcher: Arc<Mutex<Watcher>>,
}

#[godot_api]
impl CrabCore {
    /// 기본 작업 폴더 (~/SproutCode, CRAB_WORKSPACE 로 바꿀 수 있다).
    #[func]
    fn default_workspace(&self) -> GString {
        GString::from(&workspace::default_root().to_string_lossy().to_string())
    }

    /// packs_dir 아래의 게임팩 목록 (JSON 배열).
    #[func]
    fn packs(&self, packs_dir: GString) -> GString {
        let list: Vec<Value> = pack::list(Path::new(&packs_dir.to_string())).iter().map(|p| p.to_json()).collect();
        gs(&json!(list))
    }

    /// 작업 폴더를 준비한다 (없으면 만들고 git 저장소도 만든다).
    /// git_name·git_email 이 비어 있지 않으면 이 저장소의 커밋에만 그 이름·이메일을 쓴다. 서명은 끈다.
    #[func]
    fn ensure_workspace(&self, root: GString, packs_dir: GString, pack_id: GString, git_name: GString, git_email: GString) -> GString {
        let root = PathBuf::from(root.to_string());
        let (name, email) = (git_name.to_string(), git_email.to_string());
        let identity = (!name.trim().is_empty() && !email.trim().is_empty()).then_some((name.trim(), email.trim()));
        match pack::find(Path::new(&packs_dir.to_string()), &pack_id.to_string()) {
            Ok(p) => match workspace::ensure(&root, &p, identity) {
                Ok(v) => {
                    self.watcher.lock().unwrap().forget_changes(&root, &p);
                    gs(&v)
                }
                Err(e) => err(e),
            },
            Err(e) => err(e),
        }
    }

    /// 레벨을 빌드하고 실행하기 시작한다. 결과는 poll_run 으로 받는다.
    #[func]
    fn start_run(&self, root: GString, packs_dir: GString, pack_id: GString, level_id: GString, seed: i64) -> i64 {
        let (root, packs_dir, pack_id, level_id) =
            (root.to_string(), packs_dir.to_string(), pack_id.to_string(), level_id.to_string());
        self.run_slot.start(move || match pack::find(Path::new(&packs_dir), &pack_id) {
            Ok(p) => run::build_and_run(Path::new(&root), &p, &level_id, seed as u64),
            Err(e) => json!({ "compiled": false, "output": e, "diagnostics": [] }),
        }) as i64
    }

    /// 실행 결과가 나왔으면 JSON, 아직이면 빈 문자열.
    #[func]
    fn poll_run(&self) -> GString {
        GString::from(&self.run_slot.take())
    }

    /// 지난번 이후 IDE 에서 저장된 레벨 id 들 (JSON 배열).
    #[func]
    fn poll_changes(&self, root: GString, packs_dir: GString, pack_id: GString) -> GString {
        match pack::find(Path::new(&packs_dir.to_string()), &pack_id.to_string()) {
            Ok(p) => gs(&json!(self.watcher.lock().unwrap().poll(Path::new(&root.to_string()), &p))),
            Err(_) => GString::from("[]"),
        }
    }

    /// 레벨 파일을 처음 코드로 되돌린다 (지금 코드는 먼저 커밋해 둔다).
    #[func]
    fn reset_level(&self, root: GString, packs_dir: GString, pack_id: GString, level_id: GString) -> GString {
        let root = PathBuf::from(root.to_string());
        match pack::find(Path::new(&packs_dir.to_string()), &pack_id.to_string()) {
            Ok(p) => {
                let out = workspace::reset_level(&root, &p, &level_id.to_string());
                self.watcher.lock().unwrap().forget_changes(&root, &p);
                match out {
                    Ok(v) => gs(&v),
                    Err(e) => err(e),
                }
            }
            Err(e) => err(e),
        }
    }

    /// 클리어 기록 (JSON: {팩: {레벨: {cleared, best:{ticks, lines}}}}).
    #[func]
    fn progress(&self, root: GString) -> GString {
        gs(&workspace::progress(Path::new(&root.to_string())))
    }

    /// 클리어를 기록하고 git 에 커밋한다.
    #[func]
    fn record_clear(&self, root: GString, packs_dir: GString, pack_id: GString, level_id: GString, ticks: i64, lines: i64) -> GString {
        match pack::find(Path::new(&packs_dir.to_string()), &pack_id.to_string()) {
            Ok(p) => match workspace::record_clear(
                Path::new(&root.to_string()),
                &p,
                &level_id.to_string(),
                ticks as u64,
                lines as u64,
            ) {
                Ok(v) => gs(&v),
                Err(e) => err(e),
            },
            Err(e) => err(e),
        }
    }

    /// 마지막 커밋 (JSON: {hash, subject, when} 또는 null).
    #[func]
    fn last_commit(&self, root: GString) -> GString {
        match git::last_commit(Path::new(&root.to_string())) {
            Some((hash, subject, when)) => gs(&json!({ "hash": hash, "subject": subject, "when": when })),
            None => GString::from("null"),
        }
    }

    /// 전역 git 설정의 이름·이메일 (JSON: {name, email}). 작업 폴더를 만들 때 기본값으로 보여 준다.
    #[func]
    fn git_identity(&self) -> GString {
        let (name, email) = git::global_identity();
        gs(&json!({ "name": name, "email": email }))
    }

    #[func]
    fn git_available(&self) -> bool {
        git::available()
    }
}
