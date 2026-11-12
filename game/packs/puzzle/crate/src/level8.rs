//! # 퍼즐 8 · 정수 타입과 오버플로
//!
//! 이 레벨에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use puzzle::level8::*;` 로 가져와요.
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

/// 바로 앞의 문에 비밀번호를 입력해요.
pub fn unlock(code: u32) {
    crate::engine::unlock(code)
}

/// 다이얼에 적힌 숫자를 읽어요.
pub fn read_dial() -> u32 {
    crate::engine::read_dial()
}
