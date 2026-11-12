//! Crab Code 의 게임 본체(호스트).
//!
//! 어떤 게임팩이든 똑같이 다룬다: 게임팩을 읽고, 작업 폴더(cargo 워크스페이스 + git 저장소)를 준비하고,
//! 플레이어가 IDE 에서 파일을 저장하면 cargo 로 빌드·실행해 게임팩이 출력한 프레임을 모은다.
//! 그 세계가 어떻게 생겼는지는 모른다. 그리는 일은 게임팩의 화면(Godot)이 맡는다.

pub mod git;
pub mod pack;
pub mod run;
pub mod verify;
pub mod watch;
pub mod workspace;
