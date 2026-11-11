// 레벨 검증: 모든 레슨의 정답이 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.
//
//   cargo run --release --bin verify

fn main() {
    std::process::exit(if crab_core::game::verify() { 0 } else { 1 });
}
