// 레벨 모듈의 타입(Action·Dir·Tile·Tide)과 엔진의 타입을 서로 바꾼다.
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
            w::Tile::Sand => crate::level2::Tile::Sand,
            w::Tile::Food => crate::level2::Tile::Food,
            w::Tile::Nest => crate::level2::Tile::Nest,
            w::Tile::Rock => crate::level2::Tile::Sand,
            w::Tile::Flat => crate::level2::Tile::Sand,
            w::Tile::Water => crate::level2::Tile::Sand,
            w::Tile::Gull => crate::level2::Tile::Sand,
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
            w::Tile::Sand => crate::level3::Tile::Sand,
            w::Tile::Food => crate::level3::Tile::Food,
            w::Tile::Nest => crate::level3::Tile::Nest,
            w::Tile::Rock => crate::level3::Tile::Sand,
            w::Tile::Flat => crate::level3::Tile::Sand,
            w::Tile::Water => crate::level3::Tile::Sand,
            w::Tile::Gull => crate::level3::Tile::Sand,
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
            w::Tile::Sand => crate::level4::Tile::Sand,
            w::Tile::Food => crate::level4::Tile::Food,
            w::Tile::Nest => crate::level4::Tile::Nest,
            w::Tile::Rock => crate::level4::Tile::Rock,
            w::Tile::Flat => crate::level4::Tile::Sand,
            w::Tile::Water => crate::level4::Tile::Rock,
            w::Tile::Gull => crate::level4::Tile::Sand,
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
            w::Tile::Sand => crate::level5::Tile::Sand,
            w::Tile::Food => crate::level5::Tile::Food,
            w::Tile::Nest => crate::level5::Tile::Nest,
            w::Tile::Rock => crate::level5::Tile::Rock,
            w::Tile::Flat => crate::level5::Tile::Flat,
            w::Tile::Water => crate::level5::Tile::Water,
            w::Tile::Gull => crate::level5::Tile::Sand,
        }
    }
}
impl From<w::Tide> for crate::level5::Tide {
    fn from(t: w::Tide) -> crate::level5::Tide {
        match t {
            w::Tide::Low { turns_left } => crate::level5::Tide::Low { turns_left },
            w::Tide::High { turns_left } => crate::level5::Tide::High { turns_left },
            w::Tide::Calm => crate::level5::Tide::Calm,
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
            w::Tile::Sand => crate::level6::Tile::Sand,
            w::Tile::Food => crate::level6::Tile::Food,
            w::Tile::Nest => crate::level6::Tile::Nest,
            w::Tile::Rock => crate::level6::Tile::Rock,
            w::Tile::Flat => crate::level6::Tile::Flat,
            w::Tile::Water => crate::level6::Tile::Water,
            w::Tile::Gull => crate::level6::Tile::Gull,
        }
    }
}
impl From<w::Tide> for crate::level6::Tide {
    fn from(t: w::Tide) -> crate::level6::Tide {
        match t {
            w::Tide::Low { turns_left } => crate::level6::Tide::Low { turns_left },
            w::Tide::High { turns_left } => crate::level6::Tide::High { turns_left },
            w::Tide::Calm => crate::level6::Tide::Calm,
        }
    }
}
