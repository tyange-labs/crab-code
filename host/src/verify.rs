// 게임팩 검증: 임시 작업 폴더를 만들어, 레벨마다 정답은 여러 시드에서 통과하고
// 시작 코드는 통과하지 못하는지 실제로 cargo 로 빌드해서 확인한다.

use crate::pack::{self, Pack};
use crate::{run, workspace};
use std::path::Path;

pub fn verify_pack(pack_dir: &Path, seeds: u64) -> bool {
    let pack: Pack = match pack::load(pack_dir) {
        Ok(p) => p,
        Err(e) => {
            println!("FAIL  게임팩을 읽지 못함: {e}");
            return false;
        }
    };
    let root = std::env::temp_dir().join(format!("crab-verify-{}-{}", pack.id, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    // 검증용 폴더에는 git 기록이 필요 없다.
    std::env::set_var("CRAB_NO_GIT", "1");
    if let Err(e) = workspace::ensure(&root, &pack, None) {
        println!("FAIL  작업 폴더를 만들지 못함: {e}");
        return false;
    }
    println!("── {} ({}개 레벨)", pack.name(), pack.levels.len());
    let mut all_ok = true;
    for level in &pack.levels {
        let file = root.join(pack.level_file(level));
        let mut notes = Vec::new();
        let mut worst = 0;
        if level.solution.trim().is_empty() {
            notes.push("정답 코드(solution.rs)가 없음".to_string());
        } else {
            let _ = std::fs::write(&file, &level.solution);
            for seed in 1..=seeds {
                let out = run::build_and_run(&root, &pack, &level.id, seed);
                if !out["compiled"].as_bool().unwrap_or(false) {
                    notes.push(format!("정답 컴파일 실패\n{}", out["output"].as_str().unwrap_or("")));
                    break;
                }
                if !out["won"].as_bool().unwrap_or(false) {
                    notes.push(format!("정답 실패 (seed {seed}): {}", out["message"].as_str().unwrap_or("")));
                    break;
                }
                worst = worst.max(out["stats"]["ticks"].as_u64().unwrap_or(0));
            }
        }
        let _ = std::fs::write(&file, &level.start);
        let out = run::build_and_run(&root, &pack, &level.id, 1);
        if out["won"].as_bool().unwrap_or(false) {
            notes.push("시작 코드가 그대로 통과해 버림".into());
        }
        let compiled = out["compiled"].as_bool().unwrap_or(false);
        match &level.start_error {
            // 일부러 컴파일 에러를 내는 레벨: 바로 그 에러여야 한다 (다른 에러로 막히면 레슨이 엉뚱해진다).
            Some(code) => {
                let output = out["output"].as_str().unwrap_or("");
                if compiled || !output.contains(&format!("error[{code}]")) {
                    let got: Vec<&str> = output.lines().filter(|l| l.starts_with("error")).take(2).collect();
                    notes.push(format!("시작 코드가 {code} 에러를 내지 않음 (나온 것: {})", if compiled { "컴파일 성공".to_string() } else { got.join(" / ") }));
                }
            }
            None if !compiled => notes.push(format!(
                "시작 코드가 컴파일되지 않음 (일부러라면 start_error.txt 에 에러 코드를 적어 주세요)\n{}",
                out["output"].as_str().unwrap_or("")
            )),
            None => {}
        }
        if notes.is_empty() {
            println!("ok    {:<10} {}  (정답 최대 {}틱, {}줄)", level.id, level.title, worst, run::code_lines(&level.solution));
        } else {
            all_ok = false;
            println!("FAIL  {:<10} {}", level.id, level.title);
            for n in notes {
                println!("      {n}");
            }
        }
    }
    let _ = std::fs::remove_dir_all(&root);
    all_ok
}
