// 레벨 모듈의 타입(Action·Dir·Tile·Flood)과 엔진의 타입을 서로 바꾼다.
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
            crate::level2::Action::Pick => w::Action::Pick,
            crate::level2::Action::Drop => w::Action::Drop,
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
            w::Tile::Soil => crate::level2::Tile::Soil,
            w::Tile::Crop => crate::level2::Tile::Crop,
            w::Tile::Barn => crate::level2::Tile::Barn,
            w::Tile::Stone => crate::level2::Tile::Soil,
            w::Tile::Paddy => crate::level2::Tile::Soil,
            w::Tile::Water => crate::level2::Tile::Soil,
            w::Tile::Goblin => crate::level2::Tile::Soil,
        }
    }
}

// ── level3 ──
impl From<crate::level3::Action> for w::Action {
    fn from(a: crate::level3::Action) -> w::Action {
        match a {
            crate::level3::Action::Wait => w::Action::Wait,
            crate::level3::Action::Move(d) => w::Action::Move(d.into()),
            crate::level3::Action::Pick => w::Action::Pick,
            crate::level3::Action::Drop => w::Action::Drop,
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
            w::Tile::Soil => crate::level3::Tile::Soil,
            w::Tile::Crop => crate::level3::Tile::Crop,
            w::Tile::Barn => crate::level3::Tile::Barn,
            w::Tile::Stone => crate::level3::Tile::Soil,
            w::Tile::Paddy => crate::level3::Tile::Soil,
            w::Tile::Water => crate::level3::Tile::Soil,
            w::Tile::Goblin => crate::level3::Tile::Soil,
        }
    }
}

// ── level4 ──
impl From<crate::level4::Action> for w::Action {
    fn from(a: crate::level4::Action) -> w::Action {
        match a {
            crate::level4::Action::Wait => w::Action::Wait,
            crate::level4::Action::Move(d) => w::Action::Move(d.into()),
            crate::level4::Action::Pick => w::Action::Pick,
            crate::level4::Action::Drop => w::Action::Drop,
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
            w::Tile::Soil => crate::level4::Tile::Soil,
            w::Tile::Crop => crate::level4::Tile::Crop,
            w::Tile::Barn => crate::level4::Tile::Barn,
            w::Tile::Stone => crate::level4::Tile::Stone,
            w::Tile::Paddy => crate::level4::Tile::Soil,
            w::Tile::Water => crate::level4::Tile::Stone,
            w::Tile::Goblin => crate::level4::Tile::Soil,
        }
    }
}

// ── level5 ──
impl From<crate::level5::Action> for w::Action {
    fn from(a: crate::level5::Action) -> w::Action {
        match a {
            crate::level5::Action::Wait => w::Action::Wait,
            crate::level5::Action::Move(d) => w::Action::Move(d.into()),
            crate::level5::Action::Pick => w::Action::Pick,
            crate::level5::Action::Drop => w::Action::Drop,
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
            w::Tile::Soil => crate::level5::Tile::Soil,
            w::Tile::Crop => crate::level5::Tile::Crop,
            w::Tile::Barn => crate::level5::Tile::Barn,
            w::Tile::Stone => crate::level5::Tile::Stone,
            w::Tile::Paddy => crate::level5::Tile::Paddy,
            w::Tile::Water => crate::level5::Tile::Water,
            w::Tile::Goblin => crate::level5::Tile::Soil,
        }
    }
}
impl From<w::Flood> for crate::level5::Flood {
    fn from(t: w::Flood) -> crate::level5::Flood {
        match t {
            w::Flood::Low { turns_left } => crate::level5::Flood::Low { turns_left },
            w::Flood::High { turns_left } => crate::level5::Flood::High { turns_left },
            w::Flood::Calm => crate::level5::Flood::Calm,
        }
    }
}

// ── level6 ──
impl From<crate::level6::Action> for w::Action {
    fn from(a: crate::level6::Action) -> w::Action {
        match a {
            crate::level6::Action::Wait => w::Action::Wait,
            crate::level6::Action::Move(d) => w::Action::Move(d.into()),
            crate::level6::Action::Pick => w::Action::Pick,
            crate::level6::Action::Drop => w::Action::Drop,
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
            w::Tile::Soil => crate::level6::Tile::Soil,
            w::Tile::Crop => crate::level6::Tile::Crop,
            w::Tile::Barn => crate::level6::Tile::Barn,
            w::Tile::Stone => crate::level6::Tile::Stone,
            w::Tile::Paddy => crate::level6::Tile::Paddy,
            w::Tile::Water => crate::level6::Tile::Water,
            w::Tile::Goblin => crate::level6::Tile::Goblin,
        }
    }
}
impl From<w::Flood> for crate::level6::Flood {
    fn from(t: w::Flood) -> crate::level6::Flood {
        match t {
            w::Flood::Low { turns_left } => crate::level6::Flood::Low { turns_left },
            w::Flood::High { turns_left } => crate::level6::Flood::High { turns_left },
            w::Flood::Calm => crate::level6::Flood::Calm,
        }
    }
}
