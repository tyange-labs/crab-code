// 게임 엔진: 물때·갈매기·먹이가 있는 틱 시뮬레이션과 웅덩이 지도.
//
// 플레이어는 이 파일을 몰라도 된다. 플레이어가 쓰는 것은 레벨마다의 모듈(level1.rs …)에 있고,
// 거기 있는 Action·View 등은 bridge.rs 에서 이 파일의 것으로 바뀐다.
// 플레이어 프로그램 안에서 돈다. 게임 창에는 매 틱의 모습을 출력해서 보낸다 (run.rs).
// 플레이어 컴퓨터에서 인터넷 없이 바로 빌드되도록 std 외에는 아무것도 쓰지 않는다.

use std::collections::VecDeque;

/// 방향. 꽃게는 몸을 돌리지 않고 어느 쪽으로든 옆걸음질한다.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub const ALL: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

    /// (x 변화, y 변화). y 는 아래로 갈수록 커진다.
    pub fn delta(self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        }
    }

    /// 시계 방향으로 90도.
    pub fn turn_right(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    /// 반시계 방향으로 90도.
    pub fn turn_left(self) -> Dir {
        self.turn_right().turn_right().turn_right()
    }

    /// 반대쪽.
    pub fn opposite(self) -> Dir {
        self.turn_right().turn_right()
    }
}

impl Dir {
    fn code(self) -> char {
        match self {
            Dir::North => 'N',
            Dir::East => 'E',
            Dir::South => 'S',
            Dir::West => 'W',
        }
    }

    fn from_code(c: char) -> Option<Dir> {
        Some(match c {
            'N' => Dir::North,
            'E' => Dir::East,
            'S' => Dir::South,
            'W' => Dir::West,
            _ => return None,
        })
    }
}

/// 꽃게가 보는 칸.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// 모래. 걸어갈 수 있다.
    Sand,
    /// 바위. 못 지나간다.
    Rock,
    /// 깊은 물 (밀물에 잠긴 갯벌 포함). 못 들어간다.
    Water,
    /// 물이 빠진 갯벌. 걸어갈 수 있지만 밀물이 들어오면 휩쓸린다.
    Flat,
    /// 먹이가 놓인 칸.
    Food,
    /// 꽃게 굴. 먹이를 여기 내려놓으면 된다.
    Nest,
    /// 갈매기가 있는 칸.
    Gull,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가.
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Sand | Tile::Flat | Tile::Food | Tile::Nest)
    }
}

/// 꽃게가 한 틱에 할 수 있는 일.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있는다.
    Wait,
    /// 그쪽으로 한 칸 옆걸음질한다.
    Move(Dir),
    /// 발밑의 먹이를 집는다.
    Pick,
    /// 들고 있는 먹이를 내려놓는다. 굴 위라면 배달 완료.
    Drop,
}

impl Action {
    pub fn encode(&self) -> String {
        match self {
            Action::Wait => "W".into(),
            Action::Move(d) => format!("M{}", d.code()),
            Action::Pick => "P".into(),
            Action::Drop => "D".into(),
        }
    }

    pub fn decode(s: &str) -> Option<Action> {
        let mut chars = s.trim().chars();
        Some(match chars.next()? {
            'W' => Action::Wait,
            'M' => Action::Move(Dir::from_code(chars.next()?)?),
            'P' => Action::Pick,
            'D' => Action::Drop,
            _ => return None,
        })
    }
}

/// 물때. 밀물이 없는 웅덩이에서는 Calm.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tide {
    /// 물이 빠져 있다. turns_left 틱 뒤에 밀물이 들어온다.
    Low { turns_left: u32 },
    /// 물이 차 있다. turns_left 틱 뒤에 빠진다.
    High { turns_left: u32 },
    /// 물때가 없는 웅덩이.
    Calm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ground {
    Sand,
    Rock,
    Water,
    Flat,
    Nest,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mishap {
    None,
    /// 바위나 물에 막혔다.
    Bump,
    /// 밀물에 휩쓸려 굴로 돌아갔다.
    Swept,
    /// 갈매기에게 쫓겨 굴로 도망쳤다.
    Caught,
}

#[derive(Clone, Debug)]
pub struct CrabState {
    pub x: i32,
    pub y: i32,
    /// 들고 있는 먹이 번호.
    pub carrying: Option<usize>,
    /// 마지막으로 움직인 방향 (그림용).
    pub facing: Dir,
    pub mishap: Mishap,
    /// 갈매기에 놀라 굴에 숨어 있는 남은 틱. 그동안은 행동하지 못한다.
    pub hiding: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FoodState {
    Ground,
    Carried,
    Delivered,
}

#[derive(Clone, Debug)]
pub struct Food {
    pub x: i32,
    pub y: i32,
    pub home: (i32, i32),
    pub state: FoodState,
}

#[derive(Clone, Debug)]
pub struct Gull {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
}

/// 꽃게가 한 틱에 보는 것. 플레이어 코드의 tick() 이 &View 로 빌려 받는다.
#[derive(Clone, Debug)]
pub struct View {
    id: usize,
    tick: u32,
    pos: (i32, i32),
    here: Tile,
    around: [Tile; 4],
    danger: [bool; 4],
    carrying: bool,
    food_dir: Option<Dir>,
    nest_dir: Option<Dir>,
    tide: Tide,
    delivered: u32,
    goal: u32,
}

impl View {
    /// 이 꽃게의 번호 (0부터).
    pub fn id(&self) -> usize {
        self.id
    }

    /// 지금 몇 번째 틱인지 (0부터).
    pub fn tick(&self) -> u32 {
        self.tick
    }

    /// 지금 위치 (x, y). 왼쪽 위가 (0, 0).
    pub fn pos(&self) -> (i32, i32) {
        self.pos
    }

    /// 발밑 칸: Sand, Flat, Food, Nest 중 하나.
    pub fn here(&self) -> Tile {
        self.here
    }

    /// 그쪽 바로 옆 칸.
    pub fn look(&self, dir: Dir) -> Tile {
        self.around[dir_index(dir)]
    }

    /// 그쪽으로 한 칸 가면 다음 틱에 갈매기에게 잡히는가.
    pub fn danger(&self, dir: Dir) -> bool {
        self.danger[dir_index(dir)]
    }

    /// 먹이를 들고 있는가.
    pub fn carrying(&self) -> bool {
        self.carrying
    }

    /// 가장 가까운 먹이로 가는 첫걸음. 냄새가 닿지 않으면 None.
    pub fn food_dir(&self) -> Option<Dir> {
        self.food_dir
    }

    /// 굴로 가는 첫걸음. 이미 굴 위라면 None.
    pub fn nest_dir(&self) -> Option<Dir> {
        self.nest_dir
    }

    /// 지금 물때.
    pub fn tide(&self) -> Tide {
        self.tide
    }

    /// 지금까지 굴에 배달한 먹이 수 (꽃게 모두 합쳐서).
    pub fn delivered(&self) -> u32 {
        self.delivered
    }

    /// 목표 배달 수.
    pub fn goal(&self) -> u32 {
        self.goal
    }
}

fn dir_index(d: Dir) -> usize {
    match d {
        Dir::North => 0,
        Dir::East => 1,
        Dir::South => 2,
        Dir::West => 3,
    }
}

pub struct Spec {
    pub id: &'static str,
    map: &'static [&'static str],
    /// 꽃게 수 (모두 굴에서 시작).
    crabs: usize,
    /// 목표 배달 수.
    pub goal: u32,
    /// 이 틱 안에 끝내야 한다.
    pub max_ticks: u32,
    /// 먹이 냄새가 닿는 거리 (걸음 수).
    smell: u32,
    /// (주기, 그중 물이 차 있는 틱 수)
    tide: Option<(u32, u32)>,
    /// '?' 칸들 중 먹이를 놓을 개수.
    random_food: usize,
    /// 먹이 칸에 닿기만 하면 성공 (배달하지 않아도 된다).
    reach: bool,
    /// 배달하면 '?' 칸 어딘가에 새 먹이가 생긴다.
    respawn: bool,
}

// 지도 기호
//   #  바위       .  모래       N  굴 (꽃게가 여기서 시작)
//   f  먹이       ?  먹이가 놓일 수 있는 자리
//   ~  갯벌 (밀물이 들어오면 잠긴다)      w  깊은 물
//   g  가로로 왕복하는 갈매기             G  세로로 왕복하는 갈매기
pub const SPECS: &[Spec] = &[
    Spec {
        id: "tick",
        map: &["##########", "#N.....f.#", "##########"],
        crabs: 1,
        goal: 1,
        max_ticks: 20,
        smell: u32::MAX,
        tide: None,
        random_food: 0,
        respawn: false,
        reach: true,
    },
    Spec {
        id: "carry",
        map: &["##########", "#N.....f.#", "##########"],
        crabs: 1,
        goal: 1,
        max_ticks: 30,
        smell: u32::MAX,
        tide: None,
        random_food: 0,
        respawn: false,
        reach: false,
    },
    Spec {
        id: "smell",
        map: &[
            "###########",
            "#?...?...?#",
            "#.##...##.#",
            "#?..N....?#",
            "#.##...##.#",
            "#?...?...?#",
            "###########",
        ],
        crabs: 1,
        goal: 3,
        max_ticks: 90,
        smell: u32::MAX,
        tide: None,
        random_food: 3,
        respawn: false,
        reach: false,
    },
    Spec {
        id: "maze",
        map: &[
            "###########",
            "#N..#.....#",
            "###.#.###.#",
            "#...#...#.#",
            "#.#####.#.#",
            "#.......#f#",
            "###########",
        ],
        crabs: 1,
        goal: 1,
        max_ticks: 150,
        smell: 1,
        tide: None,
        random_food: 0,
        respawn: false,
        reach: false,
    },
    Spec {
        id: "tide",
        map: &[
            "#############",
            "#N..~~~~~..f#",
            "#...~~~~~...#",
            "#...~~~~~..f#",
            "#############",
        ],
        crabs: 1,
        goal: 2,
        max_ticks: 100,
        smell: u32::MAX,
        tide: Some((12, 5)),
        random_food: 0,
        respawn: false,
        reach: false,
    },
    Spec {
        id: "colony",
        map: &[
            "#############",
            "#?....?....?#",
            "#g.....g....#",
            "#...........#",
            "#.....N.....#",
            "#...........#",
            "#....g.....g#",
            "#?....?....?#",
            "#############",
        ],
        crabs: 3,
        goal: 10,
        max_ticks: 90,
        smell: u32::MAX,
        tide: None,
        random_food: 3,
        respawn: true,
        reach: false,
    },
];

pub const LEVEL_COUNT: usize = SPECS.len();

#[derive(Clone, Debug)]
pub struct Pool {
    pub w: i32,
    pub h: i32,
    pub ground: Vec<Ground>,
    pub nest: (i32, i32),
    pub crabs: Vec<CrabState>,
    pub foods: Vec<Food>,
    pub gulls: Vec<Gull>,
    pub tick: u32,
    pub delivered: u32,
    pub goal: u32,
    pub max_ticks: u32,
    pub smell: u32,
    pub tide: Option<(u32, u32)>,
    pub respawn: bool,
    pub reach: bool,
    pub spots: Vec<(i32, i32)>,
    rng: u64,
    /// 지난 틱에 일어난 일: (꽃게 번호, 설명)
    pub events: Vec<(usize, String)>,
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn build(level: usize, seed: u64) -> Pool {
    let spec = &SPECS[level.min(SPECS.len() - 1)];
    let h = spec.map.len() as i32;
    let w = spec.map[0].chars().count() as i32;
    let mut pool = Pool {
        w,
        h,
        ground: vec![Ground::Sand; (w * h) as usize],
        nest: (0, 0),
        crabs: Vec::new(),
        foods: Vec::new(),
        gulls: Vec::new(),
        tick: 0,
        delivered: 0,
        goal: spec.goal,
        max_ticks: spec.max_ticks,
        smell: spec.smell,
        tide: spec.tide,
        respawn: spec.respawn,
        reach: spec.reach,
        spots: Vec::new(),
        rng: seed ^ 0xC0FF_EE15_BEEF,
        events: Vec::new(),
    };
    for (y, row) in spec.map.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let i = (y * w + x) as usize;
            match ch {
                '#' => pool.ground[i] = Ground::Rock,
                'w' => pool.ground[i] = Ground::Water,
                '~' => pool.ground[i] = Ground::Flat,
                'N' => {
                    pool.ground[i] = Ground::Nest;
                    pool.nest = (x, y);
                }
                'f' => pool.foods.push(Food { x, y, home: (x, y), state: FoodState::Ground }),
                '?' => pool.spots.push((x, y)),
                'g' => pool.gulls.push(Gull { x, y, dir: Dir::East }),
                'G' => pool.gulls.push(Gull { x, y, dir: Dir::South }),
                _ => {}
            }
        }
    }
    for _ in 0..spec.crabs {
        pool.crabs.push(CrabState {
            x: pool.nest.0,
            y: pool.nest.1,
            carrying: None,
            facing: Dir::East,
            mishap: Mishap::None,
            hiding: 0,
        });
    }
    for _ in 0..spec.random_food {
        pool.spawn_food();
    }
    pool
}

impl Pool {
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            None
        } else {
            Some((y * self.w + x) as usize)
        }
    }

    pub fn ground_at(&self, x: i32, y: i32) -> Ground {
        self.idx(x, y).map(|i| self.ground[i]).unwrap_or(Ground::Rock)
    }

    /// 밀물이 들어와 있는가.
    pub fn flooded(&self) -> bool {
        matches!(self.tide_now(), Tide::High { .. })
    }

    pub fn tide_now(&self) -> Tide {
        match self.tide {
            None => Tide::Calm,
            Some((period, high)) => {
                let phase = self.tick % period;
                let low = period - high;
                if phase < low {
                    Tide::Low { turns_left: low - phase }
                } else {
                    Tide::High { turns_left: period - phase }
                }
            }
        }
    }

    fn spawn_food(&mut self) {
        let free: Vec<(i32, i32)> = self
            .spots
            .iter()
            .copied()
            .filter(|&(x, y)| !self.foods.iter().any(|f| f.state == FoodState::Ground && (f.x, f.y) == (x, y)))
            .collect();
        if free.is_empty() {
            return;
        }
        let (x, y) = free[(splitmix(&mut self.rng) % free.len() as u64) as usize];
        self.foods.push(Food { x, y, home: (x, y), state: FoodState::Ground });
    }

    fn food_on_ground(&self, x: i32, y: i32) -> Option<usize> {
        self.foods.iter().position(|f| f.state == FoodState::Ground && f.x == x && f.y == y)
    }

    /// 꽃게가 걸어 다닐 수 있는 땅 (물때는 따지지 않는다).
    fn walkable(&self, x: i32, y: i32) -> bool {
        matches!(self.ground_at(x, y), Ground::Sand | Ground::Flat | Ground::Nest)
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        if self.gulls.iter().any(|g| g.x == x && g.y == y) {
            return Tile::Gull;
        }
        self.ground_tile(x, y)
    }

    fn ground_tile(&self, x: i32, y: i32) -> Tile {
        match self.ground_at(x, y) {
            Ground::Rock => Tile::Rock,
            Ground::Water => Tile::Water,
            Ground::Flat if self.flooded() => Tile::Water,
            Ground::Nest => Tile::Nest,
            g => {
                if self.food_on_ground(x, y).is_some() {
                    Tile::Food
                } else if g == Ground::Flat {
                    Tile::Flat
                } else {
                    Tile::Sand
                }
            }
        }
    }

    /// from 에서 target 중 가장 가까운 곳으로 가는 첫걸음과 거리.
    /// 바위·깊은 물은 돌아간다. 갯벌은 물때와 상관없이 길로 친다. (꽃게끼리는 서로 지나갈 수 있다.)
    fn first_step(&self, from: (i32, i32), is_target: impl Fn(i32, i32) -> bool) -> Option<(Dir, u32)> {
        if is_target(from.0, from.1) {
            return None;
        }
        let mut seen = vec![false; self.ground.len()];
        let mut queue = VecDeque::new();
        seen[self.idx(from.0, from.1)?] = true;
        for d in Dir::ALL {
            let (dx, dy) = d.delta();
            let (x, y) = (from.0 + dx, from.1 + dy);
            if self.walkable(x, y) && !seen[self.idx(x, y)?] {
                seen[self.idx(x, y)?] = true;
                queue.push_back((x, y, d, 1u32));
            }
        }
        while let Some((x, y, first, dist)) = queue.pop_front() {
            if is_target(x, y) {
                return Some((first, dist));
            }
            for d in Dir::ALL {
                let (dx, dy) = d.delta();
                let (nx, ny) = (x + dx, y + dy);
                if let Some(i) = self.idx(nx, ny) {
                    if self.walkable(nx, ny) && !seen[i] {
                        seen[i] = true;
                        queue.push_back((nx, ny, first, dist + 1));
                    }
                }
            }
        }
        None
    }

    /// 갈매기가 다음 틱에 갈 칸과 방향: 앞이 막히면 돌아선다.
    fn gull_next(&self, g: &Gull) -> (i32, i32, Dir) {
        let ok = |d: Dir| {
            let (dx, dy) = d.delta();
            matches!(self.ground_at(g.x + dx, g.y + dy), Ground::Sand | Ground::Flat)
        };
        let dir = if ok(g.dir) {
            g.dir
        } else if ok(g.dir.opposite()) {
            g.dir.opposite()
        } else {
            return (g.x, g.y, g.dir);
        };
        let (dx, dy) = dir.delta();
        (g.x + dx, g.y + dy, dir)
    }

    /// 그 칸에 서 있으면 다음 틱에 갈매기에게 잡히는가.
    fn dangerous(&self, x: i32, y: i32) -> bool {
        self.gulls.iter().any(|g| {
            let (nx, ny, _) = self.gull_next(g);
            (nx, ny) == (x, y)
        })
    }

    pub fn view(&self, i: usize) -> View {
        let c = &self.crabs[i];
        let mut around = [Tile::Rock; 4];
        let mut danger = [false; 4];
        for d in Dir::ALL {
            let (dx, dy) = d.delta();
            around[dir_index(d)] = self.tile_at(c.x + dx, c.y + dy);
            danger[dir_index(d)] = self.dangerous(c.x + dx, c.y + dy);
        }
        let food_dir = if c.carrying.is_some() {
            None
        } else {
            self.first_step((c.x, c.y), |x, y| self.food_on_ground(x, y).is_some())
                .filter(|&(_, dist)| dist <= self.smell)
                .map(|(d, _)| d)
        };
        let nest = self.nest;
        View {
            id: i,
            tick: self.tick,
            pos: (c.x, c.y),
            here: self.ground_tile(c.x, c.y),
            around,
            danger,
            carrying: c.carrying.is_some(),
            food_dir,
            nest_dir: self.first_step((c.x, c.y), |x, y| (x, y) == nest).map(|(d, _)| d),
            tide: self.tide_now(),
            delivered: self.delivered,
            goal: self.goal,
        }
    }

    pub fn is_won(&self) -> bool {
        if self.reach {
            return self.crabs.iter().any(|c| self.food_on_ground(c.x, c.y).is_some());
        }
        self.delivered >= self.goal
    }

    pub fn is_done(&self) -> bool {
        self.is_won() || self.tick >= self.max_ticks
    }

    fn send_home(&mut self, i: usize, why: Mishap, text: &str) {
        if let Some(f) = self.crabs[i].carrying.take() {
            let home = self.foods[f].home;
            self.foods[f].state = FoodState::Ground;
            self.foods[f].x = home.0;
            self.foods[f].y = home.1;
        }
        self.crabs[i].x = self.nest.0;
        self.crabs[i].y = self.nest.1;
        self.crabs[i].mishap = why;
        self.events.push((i, text.to_string()));
    }

    /// 꽃게마다 고른 행동으로 한 틱을 진행한다.
    pub fn step(&mut self, actions: &[Action]) {
        self.events.clear();
        for i in 0..self.crabs.len() {
            self.crabs[i].mishap = Mishap::None;
            if self.crabs[i].hiding > 0 {
                self.crabs[i].hiding -= 1;
                continue;
            }
            let action = actions.get(i).copied().unwrap_or(Action::Wait);
            let (x, y) = (self.crabs[i].x, self.crabs[i].y);
            match action {
                Action::Wait => {}
                Action::Move(d) => {
                    let (dx, dy) = d.delta();
                    let (nx, ny) = (x + dx, y + dy);
                    self.crabs[i].facing = d;
                    let deep = self.ground_at(nx, ny) == Ground::Water
                        || (self.ground_at(nx, ny) == Ground::Flat && self.flooded());
                    if !self.walkable(nx, ny) || deep {
                        self.crabs[i].mishap = Mishap::Bump;
                        self.events.push((i, "쿵! 그쪽은 못 가요.".into()));
                    } else {
                        self.crabs[i].x = nx;
                        self.crabs[i].y = ny;
                        if let Some(f) = self.crabs[i].carrying {
                            self.foods[f].x = nx;
                            self.foods[f].y = ny;
                        }
                    }
                }
                Action::Pick => {
                    if self.crabs[i].carrying.is_some() {
                        self.events.push((i, "이미 먹이를 들고 있어요.".into()));
                    } else if let Some(f) = self.food_on_ground(x, y) {
                        self.foods[f].state = FoodState::Carried;
                        self.crabs[i].carrying = Some(f);
                    } else {
                        self.events.push((i, "여기에는 주울 먹이가 없어요.".into()));
                    }
                }
                Action::Drop => match self.crabs[i].carrying.take() {
                    None => self.events.push((i, "내려놓을 먹이가 없어요.".into())),
                    Some(f) => {
                        if (x, y) == self.nest {
                            self.foods[f].state = FoodState::Delivered;
                            self.delivered += 1;
                            self.events.push((i, format!("배달 완료! ({}/{})", self.delivered, self.goal)));
                            if self.respawn {
                                self.spawn_food();
                            }
                        } else {
                            self.foods[f].state = FoodState::Ground;
                            self.foods[f].x = x;
                            self.foods[f].y = y;
                        }
                    }
                },
            }
        }

        // 갈매기: 한 칸씩 왕복한다. 바위·물·굴에 닿으면 돌아선다.
        for g in 0..self.gulls.len() {
            let (x, y, dir) = self.gull_next(&self.gulls[g]);
            self.gulls[g] = Gull { x, y, dir };
        }

        self.tick += 1;

        for i in 0..self.crabs.len() {
            let (x, y) = (self.crabs[i].x, self.crabs[i].y);
            if self.flooded() && self.ground_at(x, y) == Ground::Flat {
                self.send_home(i, Mishap::Swept, "밀물에 휩쓸려 굴로 떠밀려 왔어요!");
            } else if self.gulls.iter().any(|g| g.x == x && g.y == y) {
                self.send_home(i, Mishap::Caught, "갈매기다! 먹이를 떨어뜨리고 굴로 도망쳤어요. (12틱 동안 숨어 있어요)");
                self.crabs[i].hiding = 12;
            }
        }
    }
}
