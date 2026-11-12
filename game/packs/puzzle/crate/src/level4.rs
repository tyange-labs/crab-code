//! # 퍼즐 4 · 나만의 함수 — fn
//!
//! 이 레벨에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use puzzle::level4::*;` 로 가져와요.
//! 새로 생긴 것에는 (새로!) 표시가 있어요.

/// 보석. `Copy` 도 `Clone` 도 아니라서 주인은 언제나 하나뿐이에요.
#[derive(Debug)]
pub struct Gem {
    pub(crate) id: usize,
}

/// 앞으로 한 칸 이동해요. 벽이나 잠긴 문에 부딪히면 프로그램이 멈춰요.
pub fn move_forward() {
    crate::engine::move_forward()
}

/// 제자리에서 왼쪽으로 90도 돌아요.
pub fn turn_left() {
    crate::engine::turn_left()
}

/// 제자리에서 오른쪽으로 90도 돌아요.
pub fn turn_right() {
    crate::engine::turn_right()
}

/// 발밑의 보석을 주워요. 주운 보석의 소유권을 돌려줘요.
pub fn collect() -> Gem {
    Gem { id: crate::engine::collect() }
}
