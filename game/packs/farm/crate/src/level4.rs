//! # 해바라기 농장 4 · 기억하는 농부
//!
//! 이 밭에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use farm::level4::*;` 로 가져와요.
//! 새로 생긴 것에는 (새로!) 표시가 있어요.

use std::fmt::Debug;

/// 농부의 두뇌. 게임이 농부마다 `new` 로 하나 만들고, 매 틱 `tick` 을 불러 할 일을 물어요.
///
/// 타임라인에서 농부의 상태를 보여 주려고 `Debug` 가 필요해요. 구조체에 `#[derive(Debug)]` 를 붙여요.
pub trait Farmer: Debug {
    /// 농부를 하나 만들어요. `id` 는 0부터 차례로 붙는 번호예요.
    fn new(id: usize) -> Self
    where
        Self: Sized;

    /// 이번 틱에 할 일을 골라 돌려줘요. `view` 는 이 농부가 지금 보는 것이에요 (빌려 받아요).
    fn tick(&mut self, view: &View) -> Action;
}

/// 이 밭을 `C` 농부로 플레이해요. `main` 에서 `play::<Pip>();` 처럼 불러요.
pub fn play<C: Farmer>() {
    crate::engine::run("maze", C::new, |farmer: &mut C, view| farmer.tick(&View { inner: view }).into());
}

/// 농부가 한 틱에 할 수 있는 일. `tick` 이 이 중 하나를 돌려주면 게임이 그대로 해 줘요.
///
/// 함수가 아니라 **값**이에요. `Action::Wait` 는 "가만히 있겠다"는 뜻의 값이고,
/// `Action::Move(Dir::East)` 는 방향 값을 하나 품은 값이에요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있어요.
    Wait,
    /// 그쪽으로 한 칸 걸어요해요.
    Move(Dir),
    /// 발밑의 열매를 집어요.
    Pick,
    /// 들고 있는 열매를 내려놓아요. 창고 위라면 배달 완료!
    Drop,
}

/// 방향. 농부는 몸을 돌리지 않고 어느 쪽으로든 걸어요해요.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    /// 시계 방향으로 90도 돈 방향. (새로!)
    pub fn turn_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    /// 반시계 방향으로 90도 돈 방향. (새로!)
    pub fn turn_left(self) -> Dir {
        self.turn_right().turn_right().turn_right()
    }

    /// 반대쪽. (새로!)
    pub fn opposite(self) -> Dir {
        self.turn_right().turn_right()
    }
}

/// 농부가 보는 칸.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// 흙. 걸어갈 수 있어요.
    Soil,
    /// 열매가 놓인 칸.
    Crop,
    /// 농부 창고. 열매를 여기 내려놓으면 돼요.
    Barn,
    /// 돌. 못 지나가요. (새로!)
    Stone,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가요? (새로!)
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Soil | Tile::Crop | Tile::Barn)
    }
}

/// 농부가 한 틱에 보는 것. `tick` 이 `&View` 로 빌려 받아요.
pub struct View {
    /// 게임 엔진이 채워 줘요.
    pub(crate) inner: crate::engine::world::View,
}

impl View {
    /// 지금 몇 번째 틱인지 (0부터).
    pub fn tick(&self) -> u32 {
        self.inner.tick()
    }

    /// 발밑 칸.
    pub fn here(&self) -> Tile {
        self.inner.here().into()
    }

    /// 열매를 들고 있나요?
    pub fn carrying(&self) -> bool {
        self.inner.carrying()
    }

    /// 가장 가까운 열매로 가는 첫걸음. 냄새가 닿지 않으면 `None`.
    pub fn crop_dir(&self) -> Option<Dir> {
        self.inner.crop_dir().map(Into::into)
    }

    /// 창고로 가는 첫걸음. 이미 창고 위라면 `None`.
    pub fn barn_dir(&self) -> Option<Dir> {
        self.inner.barn_dir().map(Into::into)
    }

    /// 그쪽 바로 옆 칸. (새로!)
    pub fn look(&self, dir: Dir) -> Tile {
        self.inner.look(dir.into()).into()
    }
}
