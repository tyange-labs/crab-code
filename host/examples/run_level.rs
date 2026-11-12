// 게임 창과 똑같이 레벨 하나를 빌드·실행해서 결과를 요약해 찍는다 (디버깅용).
//   cargo run -p crab-host --example run_level -- <작업 폴더> <팩> <레벨> [시드]
use crab_host::{pack, run};
use std::path::Path;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p = pack::find(Path::new("game/packs"), &a[2]).expect("팩");
    let out = run::build_and_run(Path::new(&a[1]), &p, &a[3], a.get(4).and_then(|s| s.parse().ok()).unwrap_or(1));
    println!("compiled={} won={} message={}", out["compiled"], out["won"], out["message"]);
    println!("frames={} map={}", out["frames"].as_array().map(|f| f.len()).unwrap_or(0), out["map"]);
    println!("output:\n{}", out["output"].as_str().unwrap_or(""));
    for l in out["logs"].as_array().into_iter().flatten().take(10) {
        println!("log: {}", l);
    }
}
