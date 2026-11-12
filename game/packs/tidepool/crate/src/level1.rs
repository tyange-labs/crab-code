//! # 조수 웅덩이 1 · 꽃게의 두뇌, tick
//!
//! 이 웅덩이에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use tidepool::level1::*;` 로 가져와요.
//! 새로 생긴 것에는 (새로!) 표시가 있어요.

use std::fmt::Debug;

/// 꽃게의 두뇌. 게임이 꽃게마다 `new` 로 하나 만들고, 매 틱 `tick` 을 불러 할 일을 물어요.
///
/// 타임라인에서 꽃게의 상태를 보여 주려고 `Debug` 가 필요해요. 구조체에 `#[derive(Debug)]` 를 붙여요.
pub trait Crab: Debug {
    /// 꽃게를 하나 만들어요. `id` 는 0부터 차례로 붙는 번호예요.
    fn new(id: usize) -> Self
    where
        Self: Sized;

    /// 이번 틱에 할 일을 골라 돌려줘요. `view` 는 이 꽃게가 지금 보는 것이에요 (빌려 받아요).
    fn tick(&mut self, view: &View) -> Action;
}

/// 이 웅덩이를 `C` 꽃게로 플레이해요. `main` 에서 `play::<Ferris>();` 처럼 불러요.
pub fn play<C: Crab>() {
    crate::engine::run("tick", C::new, |crab: &mut C, view| crab.tick(&View { inner: view }).into());
}

/// 꽃게가 한 틱에 할 수 있는 일. `tick` 이 이 중 하나를 돌려주면 게임이 그대로 해 줘요.
///
/// 함수가 아니라 **값**이에요. `Action::Wait` 는 "가만히 있겠다"는 뜻의 값이고,
/// `Action::Move(Dir::East)` 는 방향 값을 하나 품은 값이에요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있어요.
    Wait,
    /// 그쪽으로 한 칸 옆걸음질해요.
    Move(Dir),
}

/// 방향. 꽃게는 몸을 돌리지 않고 어느 쪽으로든 옆걸음질해요.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

/// 꽃게가 한 틱에 보는 것. `tick` 이 `&View` 로 빌려 받아요.
pub struct View {
    /// 게임 엔진이 채워 줘요.
    pub(crate) inner: crate::engine::world::View,
}

impl View {
    /// 지금 몇 번째 틱인지 (0부터).
    pub fn tick(&self) -> u32 {
        self.inner.tick()
    }
}
