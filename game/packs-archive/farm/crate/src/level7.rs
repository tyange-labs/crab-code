//! # 해바라기 농장 7 · 씨 뿌리고 물 주고
//!
//! 이 밭에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use farm::level7::*;` 로 가져와요.
//! 새로 생긴 것에는 표시가 있어요.

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
    crate::engine::run("sow", C::new, |farmer: &mut C, view| farmer.tick(&View { inner: view }).into());
}

/// 농부가 한 틱에 할 수 있는 일. `tick` 이 이 중 하나를 돌려주면 게임이 그대로 해 줘요.
///
/// 함수가 아니라 **값**이에요. `Action::Wait` 는 "가만히 있겠다"는 뜻의 값이고,
/// `Action::Move(Dir::East)` 는 방향 값을 하나 품은 값이에요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있어요.
    Wait,
    /// 그쪽으로 한 칸 걸어요.
    Move(Dir),
    /// 발밑의 열매를 집어요.
    Harvest,
    /// 들고 있는 열매를 내려놓아요. 창고 위라면 배달 완료!
    Store,
    /// 발밑 이랑에 씨앗을 심어요. (새로!)
    Plant,
    /// 발밑 이랑의 씨앗에 물을 줘요. 물을 줘야 자라요. (새로!)
    Water,
}

/// 방향. 농부는 몸을 돌리지 않고 어느 쪽으로든 걸어요.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    /// 시계 방향으로 90도 돈 방향.
    pub fn turn_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    /// 반시계 방향으로 90도 돈 방향.
    pub fn turn_left(self) -> Dir {
        self.turn_right().turn_right().turn_right()
    }

    /// 반대쪽.
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
    /// 돌. 못 지나가요.
    Stone,
    /// 물이 빠진 논. 걸어갈 수 있지만 홍수가 들어오면 휩쓸려요.
    Paddy,
    /// 깊은 물 (홍수에 잠긴 논도). 못 들어가요.
    Water,
    /// 고블린이 있는 칸.
    Goblin,
    /// 빈 이랑. 씨앗을 심을 수 있어요. (새로!)
    Plot,
    /// 씨앗을 심은 이랑. 물을 줘야 자라요. (새로!)
    Seed,
    /// 물을 줘서 자라는 중인 싹. 몇 틱 기다리면 열매가 열려요. (새로!)
    Sprout,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가요?
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Soil | Tile::Crop | Tile::Barn | Tile::Paddy | Tile::Plot | Tile::Seed | Tile::Sprout)
    }
}

/// 수위. 변형마다 숫자를 하나씩 품고 있어요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flood {
    /// 물이 빠져 있어요. `turns_left` 틱 뒤에 홍수가 들어와요.
    Low { turns_left: u32 },
    /// 물이 차 있어요. `turns_left` 틱 뒤에 빠져요.
    High { turns_left: u32 },
    /// 수위가 없는 밭.
    Calm,
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

    /// 이 농부의 번호 (0부터).
    pub fn id(&self) -> usize {
        self.inner.id()
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

    /// 손이 필요한 이랑(빈 이랑이나 물을 안 준 씨앗)으로 가는 첫걸음. 없으면 `None`. (새로!)
    pub fn plot_dir(&self) -> Option<Dir> {
        self.inner.plot_dir().map(Into::into)
    }

    /// 그쪽 바로 옆 칸.
    pub fn look(&self, dir: Dir) -> Tile {
        self.inner.look(dir.into()).into()
    }

    /// 지금 수위.
    pub fn flood(&self) -> Flood {
        self.inner.flood().into()
    }

    /// 그쪽으로 한 칸 가면 다음 틱에 고블린에게 잡히나요?
    pub fn danger(&self, dir: Dir) -> bool {
        self.inner.danger(dir.into())
    }

    /// 지금까지 창고에 배달한 열매 수 (농부 모두 합쳐서).
    pub fn delivered(&self) -> u32 {
        self.inner.delivered()
    }

    /// 목표 배달 수.
    pub fn goal(&self) -> u32 {
        self.inner.goal()
    }
}
