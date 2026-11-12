//! # 정거장 복구 7 · 설치와 충전
//!
//! 이 레벨에서 쓸 수 있는 것들이에요. 레벨 파일에서 `use station::level7::*;` 로 가져와요.
//! 새로 생긴 것에는 표시가 있어요.

use std::fmt::Debug;

/// 로봇의 두뇌. 게임이 로봇마다 `new` 로 하나 만들고, 매 틱 `tick` 을 불러 할 일을 물어요.
///
/// 타임라인에서 로봇의 상태를 보여 주려고 `Debug` 가 필요해요. 구조체에 `#[derive(Debug)]` 를 붙여요.
pub trait Bot: Debug {
    /// 로봇을 하나 만들어요. `id` 는 0부터 차례로 붙는 번호예요.
    fn new(id: usize) -> Self
    where
        Self: Sized;

    /// 이번 틱에 할 일을 골라 돌려줘요. `view` 는 이 로봇이 지금 보는 것이에요 (빌려 받아요).
    fn tick(&mut self, view: &View) -> Action;
}

/// 이 레벨을 `C` 로봇으로 플레이해요. `main` 에서 `play::<Echo>();` 처럼 불러요.
pub fn play<C: Bot>() {
    crate::engine::run("install", C::new, |bot: &mut C, view| bot.tick(&View { inner: view }).into());
}

/// 로봇이 한 틱에 할 수 있는 일. `tick` 이 이 중 하나를 돌려주면 게임이 그대로 해 줘요.
///
/// 함수가 아니라 **값**이에요. `Action::Wait` 는 "가만히 있겠다"는 뜻의 값이고,
/// `Action::Move(Dir::East)` 는 방향 값을 하나 품은 값이에요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있어요.
    Wait,
    /// 그쪽으로 한 칸 걸어요.
    Move(Dir),
    /// 발밑의 코어를 집어요.
    Grab,
    /// 들고 있는 코어를 내려놓아요. 반응로 위라면 장착 완료(배달)!
    Insert,
    /// 발밑 슬롯에 모듈을 설치해요. (새로!)
    Install,
    /// 발밑 슬롯의 모듈을 충전해요. 충전해야 켜져요. (새로!)
    Charge,
}

/// 방향. 로봇은 몸을 돌리지 않고 어느 쪽으로든 걸어요.
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

/// 로봇이 보는 칸.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// 갑판. 걸어갈 수 있어요.
    Deck,
    /// 코어가 놓인 칸.
    Core,
    /// 로봇 반응로. 코어를 여기 내려놓으면 돼요.
    Reactor,
    /// 벽. 못 지나가요.
    Wall,
    /// 기압이 정상인 통로. 걸어갈 수 있지만 진공이 되면 휩쓸려요.
    Duct,
    /// 허공 (진공이 된 통로도). 못 들어가요.
    Void,
    /// 경비 드론이 있는 칸.
    Sentry,
    /// 빈 슬롯. 모듈을 설치할 수 있어요. (새로!)
    Slot,
    /// 모듈을 설치한 슬롯. 충전해야 켜져요. (새로!)
    Module,
    /// 충전해서 부팅 중인 모듈. 몇 틱 기다리면 코어가 생겨요. (새로!)
    Booting,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가요?
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Deck | Tile::Core | Tile::Reactor | Tile::Duct | Tile::Slot | Tile::Module | Tile::Booting)
    }
}

/// 기압. 변형마다 숫자를 하나씩 품고 있어요.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pressure {
    /// 기압이 정상이에요. `turns_left` 틱 뒤에 진공이 돼요.
    Safe { turns_left: u32 },
    /// 진공이에요. `turns_left` 틱 뒤에 기압이 돌아와요.
    Vacuum { turns_left: u32 },
    /// 기압 변화가 없는 구역.
    Stable,
}

/// 로봇이 한 틱에 보는 것. `tick` 이 `&View` 로 빌려 받아요.
pub struct View {
    /// 게임 엔진이 채워 줘요.
    pub(crate) inner: crate::engine::world::View,
}

impl View {
    /// 지금 몇 번째 틱인지 (0부터).
    pub fn tick(&self) -> u32 {
        self.inner.tick()
    }

    /// 이 로봇의 번호 (0부터).
    pub fn id(&self) -> usize {
        self.inner.id()
    }

    /// 발밑 칸.
    pub fn here(&self) -> Tile {
        self.inner.here().into()
    }

    /// 코어를 들고 있나요?
    pub fn carrying(&self) -> bool {
        self.inner.carrying()
    }

    /// 가장 가까운 코어로 가는 첫걸음. 탐지 범위 밖이면 `None`.
    pub fn core_dir(&self) -> Option<Dir> {
        self.inner.core_dir().map(Into::into)
    }

    /// 반응로로 가는 첫걸음. 이미 반응로 위라면 `None`.
    pub fn reactor_dir(&self) -> Option<Dir> {
        self.inner.reactor_dir().map(Into::into)
    }

    /// 손이 필요한 슬롯(빈 슬롯이나 충전 안 한 모듈)으로 가는 첫걸음. 없으면 `None`. (새로!)
    pub fn slot_dir(&self) -> Option<Dir> {
        self.inner.slot_dir().map(Into::into)
    }

    /// 그쪽 바로 옆 칸.
    pub fn look(&self, dir: Dir) -> Tile {
        self.inner.look(dir.into()).into()
    }

    /// 지금 기압.
    pub fn pressure(&self) -> Pressure {
        self.inner.pressure().into()
    }

    /// 그쪽으로 한 칸 가면 다음 틱에 경비 드론에게 잡히나요?
    pub fn danger(&self, dir: Dir) -> bool {
        self.inner.danger(dir.into())
    }

    /// 지금까지 반응로에 배달한 코어 수 (로봇 모두 합쳐서).
    pub fn delivered(&self) -> u32 {
        self.inner.delivered()
    }

    /// 목표 배달 수.
    pub fn goal(&self) -> u32 {
        self.inner.goal()
    }
}
