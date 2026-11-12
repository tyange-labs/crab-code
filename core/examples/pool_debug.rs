// 꽃게 AI 레벨 하나를 정답(또는 시작 코드)으로 돌려 틱마다 지도를 글자로 찍는다.
//   cargo run --release --example pool_debug -- <레벨 번호 0부터> [start] [seed]
use crab_core::pool_game::run_code;
use crab_core::pool_lessons::POOL_LESSONS;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let level: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let start = args.get(2).map(|s| s == "start").unwrap_or(false);
    let seed: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    let lesson = &POOL_LESSONS[level];
    let out = run_code(if start { lesson.start } else { lesson.solution }, level, seed);
    let map = &out["map"];
    let (w, h) = (map["w"].as_i64().unwrap(), map["h"].as_i64().unwrap());
    let ground: Vec<char> = map["ground"].as_str().unwrap().chars().collect();
    let max: usize = std::env::var("FRAMES").ok().and_then(|s| s.parse().ok()).unwrap_or(30);
    for f in out["frames"].as_array().unwrap().iter().take(max) {
        let mut grid = ground.clone();
        for food in f["foods"].as_array().unwrap() {
            grid[(food["y"].as_i64().unwrap() * w + food["x"].as_i64().unwrap()) as usize] = 'f';
        }
        for g in f["gulls"].as_array().unwrap() {
            grid[(g["y"].as_i64().unwrap() * w + g["x"].as_i64().unwrap()) as usize] = 'G';
        }
        for (i, c) in f["crabs"].as_array().unwrap().iter().enumerate() {
            let ch = if c["carrying"].as_bool().unwrap() { (b'A' + i as u8) as char } else { (b'0' + i as u8) as char };
            grid[(c["y"].as_i64().unwrap() * w + c["x"].as_i64().unwrap()) as usize] = ch;
        }
        println!("tick {} delivered {} {:?}", f["tick"], f["delivered"], f["events"].as_array().unwrap().iter().map(|e| e["text"].as_str().unwrap()).collect::<Vec<_>>());
        for y in 0..h {
            let row: String = grid[(y * w) as usize..((y + 1) * w) as usize].iter().collect();
            println!("  {row}");
        }
    }
    println!("{} {}", out["message"], out["stats"]);
}
