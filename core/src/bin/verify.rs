// 레벨 검증: 모든 레슨의 정답이 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.
//
//   cargo run --release --bin verify            # 전부
//   cargo run --release --bin verify -- puzzle  # 퍼즐 모드만
//   cargo run --release --bin verify -- pool    # 꽃게 AI 모드만

fn main() {
    let only = std::env::args().nth(1).unwrap_or_default();
    let mut ok = true;
    if only != "pool" {
        println!("── 퍼즐 모드");
        ok &= crab_core::game::verify();
    }
    if only != "puzzle" {
        println!("── 꽃게 AI 모드 (조수 웅덩이)");
        ok &= crab_core::pool_game::verify();
    }
    std::process::exit(if ok { 0 } else { 1 });
}
