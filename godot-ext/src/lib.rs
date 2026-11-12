// crab-core 를 Godot 에 연결하는 확장.
//
// GDScript 에서 `CrabCore.new()` 로 만들어 쓴다. 값은 JSON 문자열로 주고받는다.
// 컴파일은 오래 걸리므로 `start_run` / `start_check` 가 백그라운드 스레드를 띄우고,
// 화면은 매 프레임 `poll_run` / `poll_check` 로 결과가 나왔는지 확인한다.

use godot::prelude::*;
use serde_json::{json, Value};
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
            // 그 사이에 새 작업이 시작됐으면 이 결과는 버린다.
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

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct CrabCore {
    run_slot: Slot,
    check_slot: Slot,
}

#[godot_api]
impl CrabCore {
    /// 장 목록과 레슨 목록 (JSON).
    #[func]
    fn lessons(&self) -> GString {
        GString::from(&crab_core::game::lessons_json().to_string())
    }

    /// 레벨의 처음 상태 (JSON).
    #[func]
    fn world(&self, level: i64, seed: i64) -> GString {
        GString::from(&crab_core::game::get_world(level.max(0) as usize, seed as u64).to_string())
    }

    /// 코드를 컴파일하고 실행하기 시작한다. 결과는 poll_run 으로 받는다.
    #[func]
    fn start_run(&self, code: GString, level: i64, seed: i64) -> i64 {
        let code = code.to_string();
        self.run_slot
            .start(move || crab_core::game::run_code(&code, level.max(0) as usize, seed as u64)) as i64
    }

    /// 실행 결과가 나왔으면 JSON, 아직이면 빈 문자열.
    #[func]
    fn poll_run(&self) -> GString {
        GString::from(&self.run_slot.take())
    }

    /// 컴파일 에러만 확인하기 시작한다. 결과는 poll_check 로 받는다.
    #[func]
    fn start_check(&self, code: GString) -> i64 {
        let code = code.to_string();
        self.check_slot
            .start(move || json!({ "diagnostics": crab_core::game::check_code(&code) })) as i64
    }

    #[func]
    fn poll_check(&self) -> GString {
        GString::from(&self.check_slot.take())
    }

    // ── 꽃게 AI 모드 (조수 웅덩이) ──

    /// 꽃게 AI 레슨 목록 (JSON).
    #[func]
    fn pool_lessons(&self) -> GString {
        GString::from(&crab_core::pool_game::lessons_json().to_string())
    }

    /// 읽기 전용 crab.rs 탭에 보여 줄 소스.
    #[func]
    fn pool_api(&self) -> GString {
        GString::from(&crab_core::pool_game::api_source())
    }

    /// 레벨의 지도와 처음 모습 (JSON).
    #[func]
    fn pool_level(&self, level: i64, seed: i64) -> GString {
        GString::from(&crab_core::pool_game::level_json(level.max(0) as usize, seed as u64).to_string())
    }

    /// 꽃게 AI 코드를 컴파일하고 실행하기 시작한다. 결과는 poll_run 으로 받는다.
    #[func]
    fn start_pool_run(&self, code: GString, level: i64, seed: i64) -> i64 {
        let code = code.to_string();
        self.run_slot
            .start(move || crab_core::pool_game::run_code(&code, level.max(0) as usize, seed as u64)) as i64
    }

    /// 꽃게 AI 코드의 컴파일 에러만 확인하기 시작한다. 결과는 poll_check 로 받는다.
    #[func]
    fn start_pool_check(&self, code: GString) -> i64 {
        let code = code.to_string();
        self.check_slot
            .start(move || json!({ "diagnostics": crab_core::pool_game::check_code(&code) })) as i64
    }

    /// 코드 줄 수 (빈 줄·주석 제외). 기록에 쓴다.
    #[func]
    fn code_lines(&self, code: GString) -> i64 {
        crab_core::pool_game::code_lines(&code.to_string()) as i64
    }

    /// rustfmt 로 코드를 정리한다 (JSON: ok, code 또는 error). 금방 끝나므로 바로 돌려준다.
    #[func]
    fn format(&self, code: GString) -> GString {
        GString::from(&crab_core::game::format_code(&code.to_string()).to_string())
    }

    /// 임시 폴더를 지운다. 게임을 끝낼 때 부른다.
    #[func]
    fn cleanup(&self) {
        crab_core::game::cleanup();
    }
}
