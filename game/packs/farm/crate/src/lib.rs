//! # 해바라기 농장 (게임팩)
//!
//! 농부를 조종하지 않아요. `Farmer` 트레이트를 구현해 농부의 두뇌를 만들면,
//! 게임이 매 틱 농부마다 `tick` 을 불러 할 일(`Action`)을 물어요.
//!
//! 밭마다 쓸 수 있는 것이 달라요. 처음엔 걷기만 할 수 있고, 밭을 지날 때마다 하나씩 늘어나요.
//! 레벨 파일 맨 위의 `use farm::level1::*;` 가 그 밭에서 쓸 수 있는 것을 가져와요.
//! 이름 위에서 F12(정의로 이동)를 누르면 그 레벨의 파일(level1.rs …)로 가요.
//!
//! 이 크레이트는 플레이어 프로그램과 함께 컴파일돼요. 인터넷 없이 바로 빌드되도록 std 만 써요.

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;

mod engine;

/// 게임 창의 미리 보기 (게임이 부른다).
#[doc(hidden)]
pub fn __preview(level_id: &str, seed: u64) {
    engine::preview(level_id, seed);
}
