// 게임 창이 쓰는 미리 보기: 플레이어 코드가 컴파일되지 않아도 레벨의 처음 모습을 보여 준다.
//   preview <레벨 id> <시드>
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let level = args.get(1).cloned().unwrap_or_default();
    let seed = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    farm::__preview(&level, seed);
}
