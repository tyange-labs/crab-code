//! # 퍼즐 39 · 클로저 — 변수를 붙잡는 함수
//!
//! 이 레벨에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use puzzle::level39::*;` 로 가져와요.
//! 새로 생긴 것에는 (새로!) 표시가 있어요.

/// 보석. `Copy` 도 `Clone` 도 아니라서 주인은 언제나 하나뿐이에요.
#[derive(Debug)]
pub struct Gem {
    pub(crate) id: usize,
}

/// 보석의 색.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Red,
    Blue,
}

/// 바닥 표지판.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sign {
    /// 표지판이 없어요.
    Straight,
    Left,
    Right,
    /// 도착!
    Finish,
}

/// `inspect` 가 돌려주는 보석 정보.
#[derive(Debug, Clone, Copy)]
pub struct GemInfo {
    pub color: Color,
    pub weight: u32,
    pub shiny: bool,
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

/// 발밑 받침대에 보석을 올려요. 보석의 소유권을 가져가요 (그 뒤로는 gem 을 못 써요).
pub fn deposit(gem: Gem) {
    crate::engine::deposit(gem.id)
}

/// 보석의 색을 확인해요. 빌려서 보기만 해요.
pub fn color_of(gem: &Gem) -> Color {
    crate::engine::color_of(gem.id).into()
}

/// 보석에 광을 내요. 보석을 바꾸므로 가변 레퍼런스가 필요해요.
pub fn polish(gem: &mut Gem) {
    crate::engine::polish(gem.id)
}

/// 보석을 자세히 살펴봐요. 빌려서 보기만 해요.
pub fn inspect(gem: &Gem) -> GemInfo {
    let (color, weight, shiny) = crate::engine::inspect(gem.id);
    GemInfo { color: color.into(), weight, shiny }
}

/// 바로 앞의 문에 비밀번호를 입력해요.
pub fn unlock(code: u32) {
    crate::engine::unlock(code)
}

/// 앞 칸으로 갈 수 있으면 true.
pub fn front_is_clear() -> bool {
    crate::engine::front_is_clear()
}

/// 발밑에 보석이 있으면 true.
pub fn on_gem() -> bool {
    crate::engine::on_gem()
}

/// 발밑의 표지판을 읽어요. 표지판이 없으면 Sign::Straight.
pub fn read_sign() -> Sign {
    crate::engine::read_sign().into()
}

/// 발밑의 숫자 타일을 읽어요. 숫자가 없으면 None.
pub fn read_number() -> Option<u32> {
    crate::engine::read_number()
}

/// 다이얼에 적힌 숫자를 읽어요.
pub fn read_dial() -> u32 {
    crate::engine::read_dial()
}

/// 두루마리에 적힌 숫자 목록을 읽어요.
pub fn read_scroll() -> Vec<u32> {
    crate::engine::read_scroll()
}

/// 쪽지에 적힌 글을 읽어요.
pub fn read_note() -> String {
    crate::engine::read_note()
}

/// 쪽지 두 장을 한꺼번에 읽어요.
pub fn read_two_notes() -> (String, String) {
    crate::engine::read_two_notes()
}

/// 보석까지의 거리: (앞으로 몇 칸, 오른쪽으로 몇 칸).
pub fn gem_offset() -> (u32, u32) {
    crate::engine::gem_offset()
}
