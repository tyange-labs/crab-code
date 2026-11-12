// Crab Code 의 핵심: 레벨, 레슨, 플레이어 코드 컴파일·실행, 월드 시뮬레이션.
// 화면(Godot)과는 상관없이 돌아가며, godot-ext 가 이 크레이트를 Godot 에 연결한다.

pub mod game;
pub mod lessons;
#[allow(dead_code)] // 일부는 플레이어 프로그램 쪽에서만 쓴다.
pub mod pool;
pub mod pool_game;
pub mod pool_lessons;
pub mod runner;
#[allow(dead_code)] // 일부 함수는 플레이어 프로그램 쪽에서만 쓴다.
pub mod sim;
