// 게임팩 검증: 레벨마다 정답은 통과하고 시작 코드는 통과하지 못하는지 실제로 빌드해서 확인한다.
//
//   cargo run --release --bin verify-pack -- game/packs/tidepool [시드 수]
//   cargo run --release --bin verify-pack                         # game/packs 아래 전부

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(4);
    let dirs: Vec<std::path::PathBuf> = match args.first() {
        Some(dir) => vec![dir.into()],
        None => crab_host::pack::list(std::path::Path::new("game/packs")).into_iter().map(|p| p.dir).collect(),
    };
    let mut ok = true;
    for dir in dirs {
        ok &= crab_host::verify::verify_pack(&dir, seeds);
    }
    std::process::exit(if ok { 0 } else { 1 });
}
