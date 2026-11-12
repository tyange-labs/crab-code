// 레벨 모듈의 타입(Action·Dir·Tile·Pressure)과 엔진의 타입을 서로 바꾼다.
// 레벨마다 쓸 수 있는 변형이 달라서, 그 레벨에 없는 엔진의 칸은 가장 가까운 것으로 보여 준다.

use super::world as w;

// ── level1 ──
impl From<crate::level1::Action> for w::Action {
    fn from(a: crate::level1::Action) -> w::Action {
        match a {
            crate::level1::Action::Wait => w::Action::Wait,
            crate::level1::Action::Move(d) => w::Action::Move(d.into()),
        }
    }
}
impl From<crate::level1::Dir> for w::Dir {
    fn from(d: crate::level1::Dir) -> w::Dir {
        match d {
            crate::level1::Dir::North => w::Dir::North,
            crate::level1::Dir::East => w::Dir::East,
            crate::level1::Dir::South => w::Dir::South,
            crate::level1::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level1::Dir {
    fn from(d: w::Dir) -> crate::level1::Dir {
        match d {
            w::Dir::North => crate::level1::Dir::North,
            w::Dir::East => crate::level1::Dir::East,
            w::Dir::South => crate::level1::Dir::South,
            w::Dir::West => crate::level1::Dir::West,
        }
    }
}

// ── level2 ──
impl From<crate::level2::Action> for w::Action {
    fn from(a: crate::level2::Action) -> w::Action {
        match a {
            crate::level2::Action::Wait => w::Action::Wait,
            crate::level2::Action::Move(d) => w::Action::Move(d.into()),
            crate::level2::Action::Grab => w::Action::Grab,
            crate::level2::Action::Insert => w::Action::Insert,
        }
    }
}
impl From<crate::level2::Dir> for w::Dir {
    fn from(d: crate::level2::Dir) -> w::Dir {
        match d {
            crate::level2::Dir::North => w::Dir::North,
            crate::level2::Dir::East => w::Dir::East,
            crate::level2::Dir::South => w::Dir::South,
            crate::level2::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level2::Dir {
    fn from(d: w::Dir) -> crate::level2::Dir {
        match d {
            w::Dir::North => crate::level2::Dir::North,
            w::Dir::East => crate::level2::Dir::East,
            w::Dir::South => crate::level2::Dir::South,
            w::Dir::West => crate::level2::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level2::Tile {
    fn from(t: w::Tile) -> crate::level2::Tile {
        match t {
            w::Tile::Deck => crate::level2::Tile::Deck,
            w::Tile::Core => crate::level2::Tile::Core,
            w::Tile::Reactor => crate::level2::Tile::Reactor,
            w::Tile::Wall => crate::level2::Tile::Deck,
            w::Tile::Duct => crate::level2::Tile::Deck,
            w::Tile::Void => crate::level2::Tile::Deck,
            w::Tile::Sentry => crate::level2::Tile::Deck,
            w::Tile::Slot | w::Tile::Module | w::Tile::Booting => crate::level2::Tile::Deck,
        }
    }
}

// ── level3 ──
impl From<crate::level3::Action> for w::Action {
    fn from(a: crate::level3::Action) -> w::Action {
        match a {
            crate::level3::Action::Wait => w::Action::Wait,
            crate::level3::Action::Move(d) => w::Action::Move(d.into()),
            crate::level3::Action::Grab => w::Action::Grab,
            crate::level3::Action::Insert => w::Action::Insert,
        }
    }
}
impl From<crate::level3::Dir> for w::Dir {
    fn from(d: crate::level3::Dir) -> w::Dir {
        match d {
            crate::level3::Dir::North => w::Dir::North,
            crate::level3::Dir::East => w::Dir::East,
            crate::level3::Dir::South => w::Dir::South,
            crate::level3::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level3::Dir {
    fn from(d: w::Dir) -> crate::level3::Dir {
        match d {
            w::Dir::North => crate::level3::Dir::North,
            w::Dir::East => crate::level3::Dir::East,
            w::Dir::South => crate::level3::Dir::South,
            w::Dir::West => crate::level3::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level3::Tile {
    fn from(t: w::Tile) -> crate::level3::Tile {
        match t {
            w::Tile::Deck => crate::level3::Tile::Deck,
            w::Tile::Core => crate::level3::Tile::Core,
            w::Tile::Reactor => crate::level3::Tile::Reactor,
            w::Tile::Wall => crate::level3::Tile::Deck,
            w::Tile::Duct => crate::level3::Tile::Deck,
            w::Tile::Void => crate::level3::Tile::Deck,
            w::Tile::Sentry => crate::level3::Tile::Deck,
            w::Tile::Slot | w::Tile::Module | w::Tile::Booting => crate::level3::Tile::Deck,
        }
    }
}

// ── level4 ──
impl From<crate::level4::Action> for w::Action {
    fn from(a: crate::level4::Action) -> w::Action {
        match a {
            crate::level4::Action::Wait => w::Action::Wait,
            crate::level4::Action::Move(d) => w::Action::Move(d.into()),
            crate::level4::Action::Grab => w::Action::Grab,
            crate::level4::Action::Insert => w::Action::Insert,
        }
    }
}
impl From<crate::level4::Dir> for w::Dir {
    fn from(d: crate::level4::Dir) -> w::Dir {
        match d {
            crate::level4::Dir::North => w::Dir::North,
            crate::level4::Dir::East => w::Dir::East,
            crate::level4::Dir::South => w::Dir::South,
            crate::level4::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level4::Dir {
    fn from(d: w::Dir) -> crate::level4::Dir {
        match d {
            w::Dir::North => crate::level4::Dir::North,
            w::Dir::East => crate::level4::Dir::East,
            w::Dir::South => crate::level4::Dir::South,
            w::Dir::West => crate::level4::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level4::Tile {
    fn from(t: w::Tile) -> crate::level4::Tile {
        match t {
            w::Tile::Deck => crate::level4::Tile::Deck,
            w::Tile::Core => crate::level4::Tile::Core,
            w::Tile::Reactor => crate::level4::Tile::Reactor,
            w::Tile::Wall => crate::level4::Tile::Wall,
            w::Tile::Duct => crate::level4::Tile::Deck,
            w::Tile::Void => crate::level4::Tile::Wall,
            w::Tile::Sentry => crate::level4::Tile::Deck,
            w::Tile::Slot | w::Tile::Module | w::Tile::Booting => crate::level4::Tile::Deck,
        }
    }
}

// ── level5 ──
impl From<crate::level5::Action> for w::Action {
    fn from(a: crate::level5::Action) -> w::Action {
        match a {
            crate::level5::Action::Wait => w::Action::Wait,
            crate::level5::Action::Move(d) => w::Action::Move(d.into()),
            crate::level5::Action::Grab => w::Action::Grab,
            crate::level5::Action::Insert => w::Action::Insert,
        }
    }
}
impl From<crate::level5::Dir> for w::Dir {
    fn from(d: crate::level5::Dir) -> w::Dir {
        match d {
            crate::level5::Dir::North => w::Dir::North,
            crate::level5::Dir::East => w::Dir::East,
            crate::level5::Dir::South => w::Dir::South,
            crate::level5::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level5::Dir {
    fn from(d: w::Dir) -> crate::level5::Dir {
        match d {
            w::Dir::North => crate::level5::Dir::North,
            w::Dir::East => crate::level5::Dir::East,
            w::Dir::South => crate::level5::Dir::South,
            w::Dir::West => crate::level5::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level5::Tile {
    fn from(t: w::Tile) -> crate::level5::Tile {
        match t {
            w::Tile::Deck => crate::level5::Tile::Deck,
            w::Tile::Core => crate::level5::Tile::Core,
            w::Tile::Reactor => crate::level5::Tile::Reactor,
            w::Tile::Wall => crate::level5::Tile::Wall,
            w::Tile::Duct => crate::level5::Tile::Duct,
            w::Tile::Void => crate::level5::Tile::Void,
            w::Tile::Sentry => crate::level5::Tile::Deck,
            w::Tile::Slot | w::Tile::Module | w::Tile::Booting => crate::level5::Tile::Deck,
        }
    }
}
impl From<w::Pressure> for crate::level5::Pressure {
    fn from(t: w::Pressure) -> crate::level5::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level5::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level5::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level5::Pressure::Stable,
        }
    }
}

// ── level6 ──
impl From<crate::level6::Action> for w::Action {
    fn from(a: crate::level6::Action) -> w::Action {
        match a {
            crate::level6::Action::Wait => w::Action::Wait,
            crate::level6::Action::Move(d) => w::Action::Move(d.into()),
            crate::level6::Action::Grab => w::Action::Grab,
            crate::level6::Action::Insert => w::Action::Insert,
        }
    }
}
impl From<crate::level6::Dir> for w::Dir {
    fn from(d: crate::level6::Dir) -> w::Dir {
        match d {
            crate::level6::Dir::North => w::Dir::North,
            crate::level6::Dir::East => w::Dir::East,
            crate::level6::Dir::South => w::Dir::South,
            crate::level6::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level6::Dir {
    fn from(d: w::Dir) -> crate::level6::Dir {
        match d {
            w::Dir::North => crate::level6::Dir::North,
            w::Dir::East => crate::level6::Dir::East,
            w::Dir::South => crate::level6::Dir::South,
            w::Dir::West => crate::level6::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level6::Tile {
    fn from(t: w::Tile) -> crate::level6::Tile {
        match t {
            w::Tile::Deck => crate::level6::Tile::Deck,
            w::Tile::Core => crate::level6::Tile::Core,
            w::Tile::Reactor => crate::level6::Tile::Reactor,
            w::Tile::Wall => crate::level6::Tile::Wall,
            w::Tile::Duct => crate::level6::Tile::Duct,
            w::Tile::Void => crate::level6::Tile::Void,
            w::Tile::Sentry => crate::level6::Tile::Sentry,
            w::Tile::Slot | w::Tile::Module | w::Tile::Booting => crate::level6::Tile::Deck,
        }
    }
}
impl From<w::Pressure> for crate::level6::Pressure {
    fn from(t: w::Pressure) -> crate::level6::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level6::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level6::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level6::Pressure::Stable,
        }
    }
}

// ── level7 ──
impl From<crate::level7::Action> for w::Action {
    fn from(a: crate::level7::Action) -> w::Action {
        match a {
            crate::level7::Action::Wait => w::Action::Wait,
            crate::level7::Action::Move(d) => w::Action::Move(d.into()),
            crate::level7::Action::Grab => w::Action::Grab,
            crate::level7::Action::Insert => w::Action::Insert,
            crate::level7::Action::Install => w::Action::Install,
            crate::level7::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level7::Dir> for w::Dir {
    fn from(d: crate::level7::Dir) -> w::Dir {
        match d {
            crate::level7::Dir::North => w::Dir::North,
            crate::level7::Dir::East => w::Dir::East,
            crate::level7::Dir::South => w::Dir::South,
            crate::level7::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level7::Dir {
    fn from(d: w::Dir) -> crate::level7::Dir {
        match d {
            w::Dir::North => crate::level7::Dir::North,
            w::Dir::East => crate::level7::Dir::East,
            w::Dir::South => crate::level7::Dir::South,
            w::Dir::West => crate::level7::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level7::Tile {
    fn from(t: w::Tile) -> crate::level7::Tile {
        match t {
            w::Tile::Deck => crate::level7::Tile::Deck,
            w::Tile::Core => crate::level7::Tile::Core,
            w::Tile::Reactor => crate::level7::Tile::Reactor,
            w::Tile::Wall => crate::level7::Tile::Wall,
            w::Tile::Duct => crate::level7::Tile::Duct,
            w::Tile::Void => crate::level7::Tile::Void,
            w::Tile::Sentry => crate::level7::Tile::Sentry,
            w::Tile::Slot => crate::level7::Tile::Slot,
            w::Tile::Module => crate::level7::Tile::Module,
            w::Tile::Booting => crate::level7::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level7::Pressure {
    fn from(t: w::Pressure) -> crate::level7::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level7::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level7::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level7::Pressure::Stable,
        }
    }
}

// ── level8 ──
impl From<crate::level8::Action> for w::Action {
    fn from(a: crate::level8::Action) -> w::Action {
        match a {
            crate::level8::Action::Wait => w::Action::Wait,
            crate::level8::Action::Move(d) => w::Action::Move(d.into()),
            crate::level8::Action::Grab => w::Action::Grab,
            crate::level8::Action::Insert => w::Action::Insert,
            crate::level8::Action::Install => w::Action::Install,
            crate::level8::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level8::Dir> for w::Dir {
    fn from(d: crate::level8::Dir) -> w::Dir {
        match d {
            crate::level8::Dir::North => w::Dir::North,
            crate::level8::Dir::East => w::Dir::East,
            crate::level8::Dir::South => w::Dir::South,
            crate::level8::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level8::Dir {
    fn from(d: w::Dir) -> crate::level8::Dir {
        match d {
            w::Dir::North => crate::level8::Dir::North,
            w::Dir::East => crate::level8::Dir::East,
            w::Dir::South => crate::level8::Dir::South,
            w::Dir::West => crate::level8::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level8::Tile {
    fn from(t: w::Tile) -> crate::level8::Tile {
        match t {
            w::Tile::Deck => crate::level8::Tile::Deck,
            w::Tile::Core => crate::level8::Tile::Core,
            w::Tile::Reactor => crate::level8::Tile::Reactor,
            w::Tile::Wall => crate::level8::Tile::Wall,
            w::Tile::Duct => crate::level8::Tile::Duct,
            w::Tile::Void => crate::level8::Tile::Void,
            w::Tile::Sentry => crate::level8::Tile::Sentry,
            w::Tile::Slot => crate::level8::Tile::Slot,
            w::Tile::Module => crate::level8::Tile::Module,
            w::Tile::Booting => crate::level8::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level8::Pressure {
    fn from(t: w::Pressure) -> crate::level8::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level8::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level8::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level8::Pressure::Stable,
        }
    }
}

// ── level9 ──
impl From<crate::level9::Action> for w::Action {
    fn from(a: crate::level9::Action) -> w::Action {
        match a {
            crate::level9::Action::Wait => w::Action::Wait,
            crate::level9::Action::Move(d) => w::Action::Move(d.into()),
            crate::level9::Action::Grab => w::Action::Grab,
            crate::level9::Action::Insert => w::Action::Insert,
            crate::level9::Action::Install => w::Action::Install,
            crate::level9::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level9::Dir> for w::Dir {
    fn from(d: crate::level9::Dir) -> w::Dir {
        match d {
            crate::level9::Dir::North => w::Dir::North,
            crate::level9::Dir::East => w::Dir::East,
            crate::level9::Dir::South => w::Dir::South,
            crate::level9::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level9::Dir {
    fn from(d: w::Dir) -> crate::level9::Dir {
        match d {
            w::Dir::North => crate::level9::Dir::North,
            w::Dir::East => crate::level9::Dir::East,
            w::Dir::South => crate::level9::Dir::South,
            w::Dir::West => crate::level9::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level9::Tile {
    fn from(t: w::Tile) -> crate::level9::Tile {
        match t {
            w::Tile::Deck => crate::level9::Tile::Deck,
            w::Tile::Core => crate::level9::Tile::Core,
            w::Tile::Reactor => crate::level9::Tile::Reactor,
            w::Tile::Wall => crate::level9::Tile::Wall,
            w::Tile::Duct => crate::level9::Tile::Duct,
            w::Tile::Void => crate::level9::Tile::Void,
            w::Tile::Sentry => crate::level9::Tile::Sentry,
            w::Tile::Slot => crate::level9::Tile::Slot,
            w::Tile::Module => crate::level9::Tile::Module,
            w::Tile::Booting => crate::level9::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level9::Pressure {
    fn from(t: w::Pressure) -> crate::level9::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level9::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level9::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level9::Pressure::Stable,
        }
    }
}

// ── level10 ──
impl From<crate::level10::Action> for w::Action {
    fn from(a: crate::level10::Action) -> w::Action {
        match a {
            crate::level10::Action::Wait => w::Action::Wait,
            crate::level10::Action::Move(d) => w::Action::Move(d.into()),
            crate::level10::Action::Grab => w::Action::Grab,
            crate::level10::Action::Insert => w::Action::Insert,
            crate::level10::Action::Install => w::Action::Install,
            crate::level10::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level10::Dir> for w::Dir {
    fn from(d: crate::level10::Dir) -> w::Dir {
        match d {
            crate::level10::Dir::North => w::Dir::North,
            crate::level10::Dir::East => w::Dir::East,
            crate::level10::Dir::South => w::Dir::South,
            crate::level10::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level10::Dir {
    fn from(d: w::Dir) -> crate::level10::Dir {
        match d {
            w::Dir::North => crate::level10::Dir::North,
            w::Dir::East => crate::level10::Dir::East,
            w::Dir::South => crate::level10::Dir::South,
            w::Dir::West => crate::level10::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level10::Tile {
    fn from(t: w::Tile) -> crate::level10::Tile {
        match t {
            w::Tile::Deck => crate::level10::Tile::Deck,
            w::Tile::Core => crate::level10::Tile::Core,
            w::Tile::Reactor => crate::level10::Tile::Reactor,
            w::Tile::Wall => crate::level10::Tile::Wall,
            w::Tile::Duct => crate::level10::Tile::Duct,
            w::Tile::Void => crate::level10::Tile::Void,
            w::Tile::Sentry => crate::level10::Tile::Sentry,
            w::Tile::Slot => crate::level10::Tile::Slot,
            w::Tile::Module => crate::level10::Tile::Module,
            w::Tile::Booting => crate::level10::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level10::Pressure {
    fn from(t: w::Pressure) -> crate::level10::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level10::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level10::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level10::Pressure::Stable,
        }
    }
}

// ── level11 ──
impl From<crate::level11::Action> for w::Action {
    fn from(a: crate::level11::Action) -> w::Action {
        match a {
            crate::level11::Action::Wait => w::Action::Wait,
            crate::level11::Action::Move(d) => w::Action::Move(d.into()),
            crate::level11::Action::Grab => w::Action::Grab,
            crate::level11::Action::Insert => w::Action::Insert,
            crate::level11::Action::Install => w::Action::Install,
            crate::level11::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level11::Dir> for w::Dir {
    fn from(d: crate::level11::Dir) -> w::Dir {
        match d {
            crate::level11::Dir::North => w::Dir::North,
            crate::level11::Dir::East => w::Dir::East,
            crate::level11::Dir::South => w::Dir::South,
            crate::level11::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level11::Dir {
    fn from(d: w::Dir) -> crate::level11::Dir {
        match d {
            w::Dir::North => crate::level11::Dir::North,
            w::Dir::East => crate::level11::Dir::East,
            w::Dir::South => crate::level11::Dir::South,
            w::Dir::West => crate::level11::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level11::Tile {
    fn from(t: w::Tile) -> crate::level11::Tile {
        match t {
            w::Tile::Deck => crate::level11::Tile::Deck,
            w::Tile::Core => crate::level11::Tile::Core,
            w::Tile::Reactor => crate::level11::Tile::Reactor,
            w::Tile::Wall => crate::level11::Tile::Wall,
            w::Tile::Duct => crate::level11::Tile::Duct,
            w::Tile::Void => crate::level11::Tile::Void,
            w::Tile::Sentry => crate::level11::Tile::Sentry,
            w::Tile::Slot => crate::level11::Tile::Slot,
            w::Tile::Module => crate::level11::Tile::Module,
            w::Tile::Booting => crate::level11::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level11::Pressure {
    fn from(t: w::Pressure) -> crate::level11::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level11::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level11::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level11::Pressure::Stable,
        }
    }
}

// ── level12 ──
impl From<crate::level12::Action> for w::Action {
    fn from(a: crate::level12::Action) -> w::Action {
        match a {
            crate::level12::Action::Wait => w::Action::Wait,
            crate::level12::Action::Move(d) => w::Action::Move(d.into()),
            crate::level12::Action::Grab => w::Action::Grab,
            crate::level12::Action::Insert => w::Action::Insert,
            crate::level12::Action::Install => w::Action::Install,
            crate::level12::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level12::Dir> for w::Dir {
    fn from(d: crate::level12::Dir) -> w::Dir {
        match d {
            crate::level12::Dir::North => w::Dir::North,
            crate::level12::Dir::East => w::Dir::East,
            crate::level12::Dir::South => w::Dir::South,
            crate::level12::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level12::Dir {
    fn from(d: w::Dir) -> crate::level12::Dir {
        match d {
            w::Dir::North => crate::level12::Dir::North,
            w::Dir::East => crate::level12::Dir::East,
            w::Dir::South => crate::level12::Dir::South,
            w::Dir::West => crate::level12::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level12::Tile {
    fn from(t: w::Tile) -> crate::level12::Tile {
        match t {
            w::Tile::Deck => crate::level12::Tile::Deck,
            w::Tile::Core => crate::level12::Tile::Core,
            w::Tile::Reactor => crate::level12::Tile::Reactor,
            w::Tile::Wall => crate::level12::Tile::Wall,
            w::Tile::Duct => crate::level12::Tile::Duct,
            w::Tile::Void => crate::level12::Tile::Void,
            w::Tile::Sentry => crate::level12::Tile::Sentry,
            w::Tile::Slot => crate::level12::Tile::Slot,
            w::Tile::Module => crate::level12::Tile::Module,
            w::Tile::Booting => crate::level12::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level12::Pressure {
    fn from(t: w::Pressure) -> crate::level12::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level12::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level12::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level12::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level12::SightError {
    fn from(e: w::Sight) -> crate::level12::SightError {
        match e {
            w::Sight::OffMap => crate::level12::SightError::OffMap,
            w::Sight::TooFar => crate::level12::SightError::TooFar,
        }
    }
}

// ── level13 ──
impl From<crate::level13::Action> for w::Action {
    fn from(a: crate::level13::Action) -> w::Action {
        match a {
            crate::level13::Action::Wait => w::Action::Wait,
            crate::level13::Action::Move(d) => w::Action::Move(d.into()),
            crate::level13::Action::Grab => w::Action::Grab,
            crate::level13::Action::Insert => w::Action::Insert,
            crate::level13::Action::Install => w::Action::Install,
            crate::level13::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level13::Dir> for w::Dir {
    fn from(d: crate::level13::Dir) -> w::Dir {
        match d {
            crate::level13::Dir::North => w::Dir::North,
            crate::level13::Dir::East => w::Dir::East,
            crate::level13::Dir::South => w::Dir::South,
            crate::level13::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level13::Dir {
    fn from(d: w::Dir) -> crate::level13::Dir {
        match d {
            w::Dir::North => crate::level13::Dir::North,
            w::Dir::East => crate::level13::Dir::East,
            w::Dir::South => crate::level13::Dir::South,
            w::Dir::West => crate::level13::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level13::Tile {
    fn from(t: w::Tile) -> crate::level13::Tile {
        match t {
            w::Tile::Deck => crate::level13::Tile::Deck,
            w::Tile::Core => crate::level13::Tile::Core,
            w::Tile::Reactor => crate::level13::Tile::Reactor,
            w::Tile::Wall => crate::level13::Tile::Wall,
            w::Tile::Duct => crate::level13::Tile::Duct,
            w::Tile::Void => crate::level13::Tile::Void,
            w::Tile::Sentry => crate::level13::Tile::Sentry,
            w::Tile::Slot => crate::level13::Tile::Slot,
            w::Tile::Module => crate::level13::Tile::Module,
            w::Tile::Booting => crate::level13::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level13::Pressure {
    fn from(t: w::Pressure) -> crate::level13::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level13::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level13::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level13::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level13::SightError {
    fn from(e: w::Sight) -> crate::level13::SightError {
        match e {
            w::Sight::OffMap => crate::level13::SightError::OffMap,
            w::Sight::TooFar => crate::level13::SightError::TooFar,
        }
    }
}

// ── level14 ──
impl From<crate::level14::Action> for w::Action {
    fn from(a: crate::level14::Action) -> w::Action {
        match a {
            crate::level14::Action::Wait => w::Action::Wait,
            crate::level14::Action::Move(d) => w::Action::Move(d.into()),
            crate::level14::Action::Grab => w::Action::Grab,
            crate::level14::Action::Insert => w::Action::Insert,
            crate::level14::Action::Install => w::Action::Install,
            crate::level14::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level14::Dir> for w::Dir {
    fn from(d: crate::level14::Dir) -> w::Dir {
        match d {
            crate::level14::Dir::North => w::Dir::North,
            crate::level14::Dir::East => w::Dir::East,
            crate::level14::Dir::South => w::Dir::South,
            crate::level14::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level14::Dir {
    fn from(d: w::Dir) -> crate::level14::Dir {
        match d {
            w::Dir::North => crate::level14::Dir::North,
            w::Dir::East => crate::level14::Dir::East,
            w::Dir::South => crate::level14::Dir::South,
            w::Dir::West => crate::level14::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level14::Tile {
    fn from(t: w::Tile) -> crate::level14::Tile {
        match t {
            w::Tile::Deck => crate::level14::Tile::Deck,
            w::Tile::Core => crate::level14::Tile::Core,
            w::Tile::Reactor => crate::level14::Tile::Reactor,
            w::Tile::Wall => crate::level14::Tile::Wall,
            w::Tile::Duct => crate::level14::Tile::Duct,
            w::Tile::Void => crate::level14::Tile::Void,
            w::Tile::Sentry => crate::level14::Tile::Sentry,
            w::Tile::Slot => crate::level14::Tile::Slot,
            w::Tile::Module => crate::level14::Tile::Module,
            w::Tile::Booting => crate::level14::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level14::Pressure {
    fn from(t: w::Pressure) -> crate::level14::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level14::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level14::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level14::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level14::SightError {
    fn from(e: w::Sight) -> crate::level14::SightError {
        match e {
            w::Sight::OffMap => crate::level14::SightError::OffMap,
            w::Sight::TooFar => crate::level14::SightError::TooFar,
        }
    }
}

// ── level15 ──
impl From<crate::level15::Action> for w::Action {
    fn from(a: crate::level15::Action) -> w::Action {
        match a {
            crate::level15::Action::Wait => w::Action::Wait,
            crate::level15::Action::Move(d) => w::Action::Move(d.into()),
            crate::level15::Action::Grab => w::Action::Grab,
            crate::level15::Action::Insert => w::Action::Insert,
            crate::level15::Action::Install => w::Action::Install,
            crate::level15::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level15::Dir> for w::Dir {
    fn from(d: crate::level15::Dir) -> w::Dir {
        match d {
            crate::level15::Dir::North => w::Dir::North,
            crate::level15::Dir::East => w::Dir::East,
            crate::level15::Dir::South => w::Dir::South,
            crate::level15::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level15::Dir {
    fn from(d: w::Dir) -> crate::level15::Dir {
        match d {
            w::Dir::North => crate::level15::Dir::North,
            w::Dir::East => crate::level15::Dir::East,
            w::Dir::South => crate::level15::Dir::South,
            w::Dir::West => crate::level15::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level15::Tile {
    fn from(t: w::Tile) -> crate::level15::Tile {
        match t {
            w::Tile::Deck => crate::level15::Tile::Deck,
            w::Tile::Core => crate::level15::Tile::Core,
            w::Tile::Reactor => crate::level15::Tile::Reactor,
            w::Tile::Wall => crate::level15::Tile::Wall,
            w::Tile::Duct => crate::level15::Tile::Duct,
            w::Tile::Void => crate::level15::Tile::Void,
            w::Tile::Sentry => crate::level15::Tile::Sentry,
            w::Tile::Slot => crate::level15::Tile::Slot,
            w::Tile::Module => crate::level15::Tile::Module,
            w::Tile::Booting => crate::level15::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level15::Pressure {
    fn from(t: w::Pressure) -> crate::level15::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level15::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level15::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level15::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level15::SightError {
    fn from(e: w::Sight) -> crate::level15::SightError {
        match e {
            w::Sight::OffMap => crate::level15::SightError::OffMap,
            w::Sight::TooFar => crate::level15::SightError::TooFar,
        }
    }
}

// ── level16 ──
impl From<crate::level16::Action> for w::Action {
    fn from(a: crate::level16::Action) -> w::Action {
        match a {
            crate::level16::Action::Wait => w::Action::Wait,
            crate::level16::Action::Move(d) => w::Action::Move(d.into()),
            crate::level16::Action::Grab => w::Action::Grab,
            crate::level16::Action::Insert => w::Action::Insert,
            crate::level16::Action::Install => w::Action::Install,
            crate::level16::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level16::Dir> for w::Dir {
    fn from(d: crate::level16::Dir) -> w::Dir {
        match d {
            crate::level16::Dir::North => w::Dir::North,
            crate::level16::Dir::East => w::Dir::East,
            crate::level16::Dir::South => w::Dir::South,
            crate::level16::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level16::Dir {
    fn from(d: w::Dir) -> crate::level16::Dir {
        match d {
            w::Dir::North => crate::level16::Dir::North,
            w::Dir::East => crate::level16::Dir::East,
            w::Dir::South => crate::level16::Dir::South,
            w::Dir::West => crate::level16::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level16::Tile {
    fn from(t: w::Tile) -> crate::level16::Tile {
        match t {
            w::Tile::Deck => crate::level16::Tile::Deck,
            w::Tile::Core => crate::level16::Tile::Core,
            w::Tile::Reactor => crate::level16::Tile::Reactor,
            w::Tile::Wall => crate::level16::Tile::Wall,
            w::Tile::Duct => crate::level16::Tile::Duct,
            w::Tile::Void => crate::level16::Tile::Void,
            w::Tile::Sentry => crate::level16::Tile::Sentry,
            w::Tile::Slot => crate::level16::Tile::Slot,
            w::Tile::Module => crate::level16::Tile::Module,
            w::Tile::Booting => crate::level16::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level16::Pressure {
    fn from(t: w::Pressure) -> crate::level16::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level16::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level16::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level16::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level16::SightError {
    fn from(e: w::Sight) -> crate::level16::SightError {
        match e {
            w::Sight::OffMap => crate::level16::SightError::OffMap,
            w::Sight::TooFar => crate::level16::SightError::TooFar,
        }
    }
}

// ── level17 ──
impl From<crate::level17::Action> for w::Action {
    fn from(a: crate::level17::Action) -> w::Action {
        match a {
            crate::level17::Action::Wait => w::Action::Wait,
            crate::level17::Action::Move(d) => w::Action::Move(d.into()),
            crate::level17::Action::Grab => w::Action::Grab,
            crate::level17::Action::Insert => w::Action::Insert,
            crate::level17::Action::Install => w::Action::Install,
            crate::level17::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level17::Dir> for w::Dir {
    fn from(d: crate::level17::Dir) -> w::Dir {
        match d {
            crate::level17::Dir::North => w::Dir::North,
            crate::level17::Dir::East => w::Dir::East,
            crate::level17::Dir::South => w::Dir::South,
            crate::level17::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level17::Dir {
    fn from(d: w::Dir) -> crate::level17::Dir {
        match d {
            w::Dir::North => crate::level17::Dir::North,
            w::Dir::East => crate::level17::Dir::East,
            w::Dir::South => crate::level17::Dir::South,
            w::Dir::West => crate::level17::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level17::Tile {
    fn from(t: w::Tile) -> crate::level17::Tile {
        match t {
            w::Tile::Deck => crate::level17::Tile::Deck,
            w::Tile::Core => crate::level17::Tile::Core,
            w::Tile::Reactor => crate::level17::Tile::Reactor,
            w::Tile::Wall => crate::level17::Tile::Wall,
            w::Tile::Duct => crate::level17::Tile::Duct,
            w::Tile::Void => crate::level17::Tile::Void,
            w::Tile::Sentry => crate::level17::Tile::Sentry,
            w::Tile::Slot => crate::level17::Tile::Slot,
            w::Tile::Module => crate::level17::Tile::Module,
            w::Tile::Booting => crate::level17::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level17::Pressure {
    fn from(t: w::Pressure) -> crate::level17::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level17::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level17::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level17::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level17::SightError {
    fn from(e: w::Sight) -> crate::level17::SightError {
        match e {
            w::Sight::OffMap => crate::level17::SightError::OffMap,
            w::Sight::TooFar => crate::level17::SightError::TooFar,
        }
    }
}

// ── level18 ──
impl From<crate::level18::Action> for w::Action {
    fn from(a: crate::level18::Action) -> w::Action {
        match a {
            crate::level18::Action::Wait => w::Action::Wait,
            crate::level18::Action::Move(d) => w::Action::Move(d.into()),
            crate::level18::Action::Grab => w::Action::Grab,
            crate::level18::Action::Insert => w::Action::Insert,
            crate::level18::Action::Install => w::Action::Install,
            crate::level18::Action::Charge => w::Action::Charge,
        }
    }
}
impl From<crate::level18::Dir> for w::Dir {
    fn from(d: crate::level18::Dir) -> w::Dir {
        match d {
            crate::level18::Dir::North => w::Dir::North,
            crate::level18::Dir::East => w::Dir::East,
            crate::level18::Dir::South => w::Dir::South,
            crate::level18::Dir::West => w::Dir::West,
        }
    }
}
impl From<w::Dir> for crate::level18::Dir {
    fn from(d: w::Dir) -> crate::level18::Dir {
        match d {
            w::Dir::North => crate::level18::Dir::North,
            w::Dir::East => crate::level18::Dir::East,
            w::Dir::South => crate::level18::Dir::South,
            w::Dir::West => crate::level18::Dir::West,
        }
    }
}
impl From<w::Tile> for crate::level18::Tile {
    fn from(t: w::Tile) -> crate::level18::Tile {
        match t {
            w::Tile::Deck => crate::level18::Tile::Deck,
            w::Tile::Core => crate::level18::Tile::Core,
            w::Tile::Reactor => crate::level18::Tile::Reactor,
            w::Tile::Wall => crate::level18::Tile::Wall,
            w::Tile::Duct => crate::level18::Tile::Duct,
            w::Tile::Void => crate::level18::Tile::Void,
            w::Tile::Sentry => crate::level18::Tile::Sentry,
            w::Tile::Slot => crate::level18::Tile::Slot,
            w::Tile::Module => crate::level18::Tile::Module,
            w::Tile::Booting => crate::level18::Tile::Booting,
        }
    }
}
impl From<w::Pressure> for crate::level18::Pressure {
    fn from(t: w::Pressure) -> crate::level18::Pressure {
        match t {
            w::Pressure::Safe { turns_left } => crate::level18::Pressure::Safe { turns_left },
            w::Pressure::Vacuum { turns_left } => crate::level18::Pressure::Vacuum { turns_left },
            w::Pressure::Stable => crate::level18::Pressure::Stable,
        }
    }
}

impl From<w::Sight> for crate::level18::SightError {
    fn from(e: w::Sight) -> crate::level18::SightError {
        match e {
            w::Sight::OffMap => crate::level18::SightError::OffMap,
            w::Sight::TooFar => crate::level18::SightError::TooFar,
        }
    }
}
