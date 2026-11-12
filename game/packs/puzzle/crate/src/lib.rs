//! # 퍼즐 (Crab Code 게임팩)
//!
//! 꽃게 페리스를 함수 호출로 한 걸음씩 움직여요. 『프로그래밍 러스트』 2~17장 순서의 51개 레벨.
//!
//! 레벨마다 쓸 수 있는 함수가 달라요. 처음엔 `move_forward()` 와 `collect()` 뿐이고, 레벨을 지날 때마다 늘어나요.
//! 레벨 파일 맨 위의 `use puzzle::level7::*;` 가 그 레벨에서 쓸 수 있는 것을 가져와요.
//! 이름 위에서 F12(정의로 이동)를 누르면 그 레벨의 파일(level7.rs …)로 가요.
//!
//! 이 크레이트는 플레이어 프로그램과 함께 컴파일돼요. 인터넷 없이 바로 빌드되도록 std 만 써요.

// 앞쪽 레벨에서는 아직 보석 번호(Gem 의 id)를 읽는 함수가 열리지 않는다.
#![allow(dead_code)]

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;
pub mod level10;
pub mod level11;
pub mod level12;
pub mod level13;
pub mod level14;
pub mod level15;
pub mod level16;
pub mod level17;
pub mod level18;
pub mod level19;
pub mod level20;
pub mod level21;
pub mod level22;
pub mod level23;
pub mod level24;
pub mod level25;
pub mod level26;
pub mod level27;
pub mod level28;
pub mod level29;
pub mod level30;
pub mod level31;
pub mod level32;
pub mod level33;
pub mod level34;
pub mod level35;
pub mod level36;
pub mod level37;
pub mod level38;
pub mod level39;
pub mod level40;
pub mod level41;
pub mod level42;
pub mod level43;
pub mod level44;
pub mod level45;
pub mod level46;
pub mod level47;
pub mod level48;
pub mod level49;
pub mod level50;
pub mod level51;

mod engine;

/// 게임 창의 미리 보기 (게임이 부른다).
#[doc(hidden)]
pub fn __preview(level_id: &str, seed: u64) {
    engine::preview(level_id, seed);
}
