// 읽기 전용 crab.rs 탭에 들어갈 소스를 찍어 본다.
//   cargo run --example api_source
fn main() {
    print!("{}", crab_core::pool_game::api_source());
}
