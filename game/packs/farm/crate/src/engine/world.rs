// 게임 엔진: 수위·고블린·열매가 있는 틱 시뮬레이션과 밭 지도.
//
// 플레이어는 이 파일을 몰라도 된다. 플레이어가 쓰는 것은 레벨마다의 모듈(level1.rs …)에 있고,
// 거기 있는 Action·View 등은 bridge.rs 에서 이 파일의 것으로 바뀐다.
// 플레이어 프로그램 안에서 돈다. 게임 창에는 매 틱의 모습을 출력해서 보낸다 (run.rs).
// 플레이어 컴퓨터에서 인터넷 없이 바로 빌드되도록 std 외에는 아무것도 쓰지 않는다.

use std::collections::VecDeque;

/// 방향. 농부는 몸을 돌리지 않고 어느 쪽으로든 걸어요한다.
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

/// 농부가 보는 칸.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// 흙. 걸어갈 수 있다.
    Soil,
    /// 돌. 못 지나간다.
    Stone,
    /// 깊은 물 (홍수에 잠긴 논 포함). 못 들어간다.
    Water,
    /// 물이 빠진 논. 걸어갈 수 있지만 홍수가 들어오면 휩쓸린다.
    Paddy,
    /// 열매가 놓인 칸.
    Crop,
    /// 농부 창고. 열매를 여기 내려놓으면 된다.
    Barn,
    /// 고블린이 있는 칸.
    Goblin,
    /// 빈 이랑. 씨앗을 심을 수 있다.
    Plot,
    /// 씨앗을 심은 이랑. 물을 줘야 자란다.
    Seed,
    /// 물을 줘서 자라는 중인 싹.
    Sprout,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가.
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Soil | Tile::Paddy | Tile::Crop | Tile::Barn | Tile::Plot | Tile::Seed | Tile::Sprout)
    }
}

/// 농부가 한 틱에 할 수 있는 일.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있는다.
    Wait,
    /// 그쪽으로 한 칸 걸어요한다.
    Move(Dir),
    /// 발밑의 열매를 집는다.
    Harvest,
    /// 들고 있는 열매를 내려놓는다. 창고 위라면 배달 완료.
    Store,
    /// 발밑 이랑에 씨앗을 심는다.
    Plant,
    /// 발밑 이랑의 씨앗에 물을 준다.
    Water,
}

impl Action {
    pub fn encode(&self) -> String {
        match self {
            Action::Wait => "W".into(),
            Action::Move(d) => format!("M{}", d.code()),
            Action::Harvest => "P".into(),
            Action::Store => "D".into(),
            Action::Plant => "S".into(),
            Action::Water => "A".into(),
        }
    }

    pub fn decode(s: &str) -> Option<Action> {
        let mut chars = s.trim().chars();
        Some(match chars.next()? {
            'W' => Action::Wait,
            'M' => Action::Move(Dir::from_code(chars.next()?)?),
            'P' => Action::Harvest,
            'D' => Action::Store,
            'S' => Action::Plant,
            'A' => Action::Water,
            _ => return None,
        })
    }
}

/// 수위. 홍수가 없는 밭에서는 Calm.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flood {
    /// 물이 빠져 있다. turns_left 틱 뒤에 홍수가 들어온다.
    Low { turns_left: u32 },
    /// 물이 차 있다. turns_left 틱 뒤에 빠진다.
    High { turns_left: u32 },
    /// 수위가 없는 밭.
    Calm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ground {
    Soil,
    Stone,
    Water,
    Paddy,
    Barn,
    Plot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mishap {
    None,
    /// 돌나 물에 막혔다.
    Bump,
    /// 홍수에 휩쓸려 창고로 돌아갔다.
    Swept,
    /// 고블린에게 쫓겨 창고로 도망쳤다.
    Caught,
}

#[derive(Clone, Debug)]
pub struct FarmerState {
    pub x: i32,
    pub y: i32,
    /// 들고 있는 열매 번호.
    pub carrying: Option<usize>,
    /// 마지막으로 움직인 방향 (그림용).
    pub facing: Dir,
    pub mishap: Mishap,
    /// 고블린에 놀라 창고에 숨어 있는 남은 틱. 그동안은 행동하지 못한다.
    pub hiding: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CropState {
    Ground,
    Carried,
    Delivered,
}

#[derive(Clone, Debug)]
pub struct Crop {
    pub x: i32,
    pub y: i32,
    pub home: (i32, i32),
    pub state: CropState,
}

#[derive(Clone, Debug)]
pub struct Goblin {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
}

/// 이랑 한 칸의 상태.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlotStage {
    /// 비어 있다.
    Empty,
    /// 씨앗을 심었다. 물을 줘야 자란다.
    Seeded,
    /// 물을 줬다. left 틱 뒤에 다 자란다.
    Growing { left: u32 },
}

#[derive(Clone, Debug)]
pub struct Plot {
    pub x: i32,
    pub y: i32,
    pub stage: PlotStage,
}

/// 물을 준 뒤 다 자랄 때까지 걸리는 틱.
pub const GROW_TICKS: u32 = 4;

/// 농부가 한 틱에 보는 것. 플레이어 코드의 tick() 이 &View 로 빌려 받는다.
#[derive(Clone, Debug)]
pub struct View {
    id: usize,
    tick: u32,
    pos: (i32, i32),
    here: Tile,
    around: [Tile; 4],
    danger: [bool; 4],
    carrying: bool,
    crop_dir: Option<Dir>,
    barn_dir: Option<Dir>,
    plot_dir: Option<Dir>,
    flood: Flood,
    delivered: u32,
    goal: u32,
}

impl View {
    /// 이 농부의 번호 (0부터).
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

    /// 발밑 칸: Soil, Paddy, Crop, Barn 중 하나.
    pub fn here(&self) -> Tile {
        self.here
    }

    /// 그쪽 바로 옆 칸.
    pub fn look(&self, dir: Dir) -> Tile {
        self.around[dir_index(dir)]
    }

    /// 그쪽으로 한 칸 가면 다음 틱에 고블린에게 잡히는가.
    pub fn danger(&self, dir: Dir) -> bool {
        self.danger[dir_index(dir)]
    }

    /// 열매를 들고 있는가.
    pub fn carrying(&self) -> bool {
        self.carrying
    }

    /// 가장 가까운 열매로 가는 첫걸음. 냄새가 닿지 않으면 None.
    pub fn crop_dir(&self) -> Option<Dir> {
        self.crop_dir
    }

    /// 창고로 가는 첫걸음. 이미 창고 위라면 None.
    pub fn barn_dir(&self) -> Option<Dir> {
        self.barn_dir
    }

    /// 손이 필요한 이랑(빈 이랑이나 물을 안 준 씨앗)으로 가는 첫걸음. 없으면 None.
    pub fn plot_dir(&self) -> Option<Dir> {
        self.plot_dir
    }

    /// 지금 수위.
    pub fn flood(&self) -> Flood {
        self.flood
    }

    /// 지금까지 창고에 배달한 열매 수 (농부 모두 합쳐서).
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
    /// 농부 수 (모두 창고에서 시작).
    farmers: usize,
    /// 목표 배달 수.
    pub goal: u32,
    /// 이 틱 안에 끝내야 한다.
    pub max_ticks: u32,
    /// 열매 냄새가 닿는 거리 (걸음 수).
    smell: u32,
    /// (주기, 그중 물이 차 있는 틱 수)
    flood: Option<(u32, u32)>,
    /// '?' 칸들 중 열매를 놓을 개수.
    random_crop: usize,
    /// 열매 칸에 닿기만 하면 성공 (배달하지 않아도 된다).
    reach: bool,
    /// 배달하면 '?' 칸 어딘가에 새 열매가 생긴다.
    respawn: bool,
}

// 지도 기호
//   #  돌       .  흙       N  창고 (농부가 여기서 시작)
//   f  열매       ?  열매가 놓일 수 있는 자리
//   ~  논 (홍수가 들어오면 잠긴다)      w  깊은 물
//   p  이랑 (씨앗을 심고 물을 주면 열매가 자란다)
//   g  가로로 왕복하는 고블린             G  세로로 왕복하는 고블린
pub const SPECS: &[Spec] = &[
    Spec {
        id: "tick",
        map: &["##########", "#N.....f.#", "##########"],
        farmers: 1,
        goal: 1,
        max_ticks: 20,
        smell: u32::MAX,
        flood: None,
        random_crop: 0,
        respawn: false,
        reach: true,
    },
    Spec {
        id: "carry",
        map: &["##########", "#N.....f.#", "##########"],
        farmers: 1,
        goal: 1,
        max_ticks: 30,
        smell: u32::MAX,
        flood: None,
        random_crop: 0,
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
        farmers: 1,
        goal: 3,
        max_ticks: 90,
        smell: u32::MAX,
        flood: None,
        random_crop: 3,
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
        farmers: 1,
        goal: 1,
        max_ticks: 150,
        smell: 1,
        flood: None,
        random_crop: 0,
        respawn: false,
        reach: false,
    },
    Spec {
        id: "flood",
        map: &[
            "#############",
            "#N..~~~~~..f#",
            "#...~~~~~...#",
            "#...~~~~~..f#",
            "#############",
        ],
        farmers: 1,
        goal: 2,
        max_ticks: 100,
        smell: u32::MAX,
        flood: Some((12, 5)),
        random_crop: 0,
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
        farmers: 3,
        goal: 10,
        max_ticks: 90,
        smell: u32::MAX,
        flood: None,
        random_crop: 3,
        respawn: true,
        reach: false,
    },
    Spec {
        id: "sow",
        map: &[
            "#############",
            "#N..........#",
            "#...........#",
            "#..p..p..p..#",
            "#...........#",
            "#############",
        ],
        farmers: 1,
        goal: 3,
        max_ticks: 70,
        smell: u32::MAX,
        flood: None,
        random_crop: 0,
        respawn: false,
        reach: false,
    },
];

pub const LEVEL_COUNT: usize = SPECS.len();

#[derive(Clone, Debug)]
pub struct Field {
    pub w: i32,
    pub h: i32,
    pub ground: Vec<Ground>,
    pub barn: (i32, i32),
    pub farmers: Vec<FarmerState>,
    pub crops: Vec<Crop>,
    pub plots: Vec<Plot>,
    pub goblins: Vec<Goblin>,
    pub tick: u32,
    pub delivered: u32,
    pub goal: u32,
    pub max_ticks: u32,
    pub smell: u32,
    pub flood: Option<(u32, u32)>,
    pub respawn: bool,
    pub reach: bool,
    pub spots: Vec<(i32, i32)>,
    rng: u64,
    /// 지난 틱에 일어난 일: (농부 번호, 설명)
    pub events: Vec<(usize, String)>,
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn build(level: usize, seed: u64) -> Field {
    let spec = &SPECS[level.min(SPECS.len() - 1)];
    let h = spec.map.len() as i32;
    let w = spec.map[0].chars().count() as i32;
    let mut field = Field {
        w,
        h,
        ground: vec![Ground::Soil; (w * h) as usize],
        barn: (0, 0),
        farmers: Vec::new(),
        crops: Vec::new(),
        plots: Vec::new(),
        goblins: Vec::new(),
        tick: 0,
        delivered: 0,
        goal: spec.goal,
        max_ticks: spec.max_ticks,
        smell: spec.smell,
        flood: spec.flood,
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
                '#' => field.ground[i] = Ground::Stone,
                'w' => field.ground[i] = Ground::Water,
                '~' => field.ground[i] = Ground::Paddy,
                'p' => {
                    field.ground[i] = Ground::Plot;
                    field.plots.push(Plot { x, y, stage: PlotStage::Empty });
                }
                'N' => {
                    field.ground[i] = Ground::Barn;
                    field.barn = (x, y);
                }
                'f' => field.crops.push(Crop { x, y, home: (x, y), state: CropState::Ground }),
                '?' => field.spots.push((x, y)),
                'g' => field.goblins.push(Goblin { x, y, dir: Dir::East }),
                'G' => field.goblins.push(Goblin { x, y, dir: Dir::South }),
                _ => {}
            }
        }
    }
    for _ in 0..spec.farmers {
        field.farmers.push(FarmerState {
            x: field.barn.0,
            y: field.barn.1,
            carrying: None,
            facing: Dir::East,
            mishap: Mishap::None,
            hiding: 0,
        });
    }
    for _ in 0..spec.random_crop {
        field.spawn_crop();
    }
    field
}

impl Field {
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            None
        } else {
            Some((y * self.w + x) as usize)
        }
    }

    pub fn ground_at(&self, x: i32, y: i32) -> Ground {
        self.idx(x, y).map(|i| self.ground[i]).unwrap_or(Ground::Stone)
    }

    /// 홍수가 들어와 있는가.
    pub fn flooded(&self) -> bool {
        matches!(self.flood_now(), Flood::High { .. })
    }

    pub fn flood_now(&self) -> Flood {
        match self.flood {
            None => Flood::Calm,
            Some((period, high)) => {
                let phase = self.tick % period;
                let low = period - high;
                if phase < low {
                    Flood::Low { turns_left: low - phase }
                } else {
                    Flood::High { turns_left: period - phase }
                }
            }
        }
    }

    fn spawn_crop(&mut self) {
        let free: Vec<(i32, i32)> = self
            .spots
            .iter()
            .copied()
            .filter(|&(x, y)| !self.crops.iter().any(|f| f.state == CropState::Ground && (f.x, f.y) == (x, y)))
            .collect();
        if free.is_empty() {
            return;
        }
        let (x, y) = free[(splitmix(&mut self.rng) % free.len() as u64) as usize];
        self.crops.push(Crop { x, y, home: (x, y), state: CropState::Ground });
    }

    fn plot_at(&self, x: i32, y: i32) -> Option<&Plot> {
        self.plots.iter().find(|p| p.x == x && p.y == y)
    }

    /// 손이 필요한 이랑인가: 열매가 안 열렸고, 비었거나 물을 안 준 씨앗.
    fn plot_needs_work(&self, x: i32, y: i32) -> bool {
        self.crop_on_ground(x, y).is_none()
            && matches!(self.plot_at(x, y).map(|p| p.stage), Some(PlotStage::Empty) | Some(PlotStage::Seeded))
    }

    fn crop_on_ground(&self, x: i32, y: i32) -> Option<usize> {
        self.crops.iter().position(|f| f.state == CropState::Ground && f.x == x && f.y == y)
    }

    /// 농부가 걸어 다닐 수 있는 땅 (수위는 따지지 않는다).
    fn walkable(&self, x: i32, y: i32) -> bool {
        matches!(self.ground_at(x, y), Ground::Soil | Ground::Paddy | Ground::Barn | Ground::Plot)
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        if self.goblins.iter().any(|g| g.x == x && g.y == y) {
            return Tile::Goblin;
        }
        self.ground_tile(x, y)
    }

    fn ground_tile(&self, x: i32, y: i32) -> Tile {
        match self.ground_at(x, y) {
            Ground::Stone => Tile::Stone,
            Ground::Water => Tile::Water,
            Ground::Paddy if self.flooded() => Tile::Water,
            Ground::Barn => Tile::Barn,
            g => {
                if self.crop_on_ground(x, y).is_some() {
                    Tile::Crop
                } else if g == Ground::Plot {
                    match self.plot_at(x, y).map(|p| p.stage) {
                        Some(PlotStage::Seeded) => Tile::Seed,
                        Some(PlotStage::Growing { .. }) => Tile::Sprout,
                        _ => Tile::Plot,
                    }
                } else if g == Ground::Paddy {
                    Tile::Paddy
                } else {
                    Tile::Soil
                }
            }
        }
    }

    /// from 에서 target 중 가장 가까운 곳으로 가는 첫걸음과 거리.
    /// 돌·깊은 물은 돌아간다. 논은 수위와 상관없이 길로 친다. (농부끼리는 서로 지나갈 수 있다.)
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

    /// 고블린이 다음 틱에 갈 칸과 방향: 앞이 막히면 돌아선다.
    fn goblin_next(&self, g: &Goblin) -> (i32, i32, Dir) {
        let ok = |d: Dir| {
            let (dx, dy) = d.delta();
            matches!(self.ground_at(g.x + dx, g.y + dy), Ground::Soil | Ground::Paddy | Ground::Plot)
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

    /// 그 칸에 서 있으면 다음 틱에 고블린에게 잡히는가.
    fn dangerous(&self, x: i32, y: i32) -> bool {
        self.goblins.iter().any(|g| {
            let (nx, ny, _) = self.goblin_next(g);
            (nx, ny) == (x, y)
        })
    }

    pub fn view(&self, i: usize) -> View {
        let c = &self.farmers[i];
        let mut around = [Tile::Stone; 4];
        let mut danger = [false; 4];
        for d in Dir::ALL {
            let (dx, dy) = d.delta();
            around[dir_index(d)] = self.tile_at(c.x + dx, c.y + dy);
            danger[dir_index(d)] = self.dangerous(c.x + dx, c.y + dy);
        }
        let crop_dir = if c.carrying.is_some() {
            None
        } else {
            self.first_step((c.x, c.y), |x, y| self.crop_on_ground(x, y).is_some())
                .filter(|&(_, dist)| dist <= self.smell)
                .map(|(d, _)| d)
        };
        let barn = self.barn;
        View {
            id: i,
            tick: self.tick,
            pos: (c.x, c.y),
            here: self.ground_tile(c.x, c.y),
            around,
            danger,
            carrying: c.carrying.is_some(),
            crop_dir,
            barn_dir: self.first_step((c.x, c.y), |x, y| (x, y) == barn).map(|(d, _)| d),
            plot_dir: self.first_step((c.x, c.y), |x, y| self.plot_needs_work(x, y)).map(|(d, _)| d),
            flood: self.flood_now(),
            delivered: self.delivered,
            goal: self.goal,
        }
    }

    pub fn is_won(&self) -> bool {
        if self.reach {
            return self.farmers.iter().any(|c| self.crop_on_ground(c.x, c.y).is_some());
        }
        self.delivered >= self.goal
    }

    pub fn is_done(&self) -> bool {
        self.is_won() || self.tick >= self.max_ticks
    }

    fn send_home(&mut self, i: usize, why: Mishap, text: &str) {
        if let Some(f) = self.farmers[i].carrying.take() {
            let home = self.crops[f].home;
            self.crops[f].state = CropState::Ground;
            self.crops[f].x = home.0;
            self.crops[f].y = home.1;
        }
        self.farmers[i].x = self.barn.0;
        self.farmers[i].y = self.barn.1;
        self.farmers[i].mishap = why;
        self.events.push((i, text.to_string()));
    }

    /// 농부마다 고른 행동으로 한 틱을 진행한다.
    pub fn step(&mut self, actions: &[Action]) {
        self.events.clear();
        for i in 0..self.farmers.len() {
            self.farmers[i].mishap = Mishap::None;
            if self.farmers[i].hiding > 0 {
                self.farmers[i].hiding -= 1;
                continue;
            }
            let action = actions.get(i).copied().unwrap_or(Action::Wait);
            let (x, y) = (self.farmers[i].x, self.farmers[i].y);
            match action {
                Action::Wait => {}
                Action::Move(d) => {
                    let (dx, dy) = d.delta();
                    let (nx, ny) = (x + dx, y + dy);
                    self.farmers[i].facing = d;
                    let deep = self.ground_at(nx, ny) == Ground::Water
                        || (self.ground_at(nx, ny) == Ground::Paddy && self.flooded());
                    if !self.walkable(nx, ny) || deep {
                        self.farmers[i].mishap = Mishap::Bump;
                        self.events.push((i, "쿵! 그쪽은 못 가요.".into()));
                    } else {
                        self.farmers[i].x = nx;
                        self.farmers[i].y = ny;
                        if let Some(f) = self.farmers[i].carrying {
                            self.crops[f].x = nx;
                            self.crops[f].y = ny;
                        }
                    }
                }
                Action::Harvest => {
                    if self.farmers[i].carrying.is_some() {
                        self.events.push((i, "이미 열매를 들고 있어요.".into()));
                    } else if let Some(f) = self.crop_on_ground(x, y) {
                        self.crops[f].state = CropState::Carried;
                        self.farmers[i].carrying = Some(f);
                    } else {
                        self.events.push((i, "여기에는 주울 열매가 없어요.".into()));
                    }
                }
                Action::Plant => match self.plots.iter().position(|p| p.x == x && p.y == y) {
                    None => self.events.push((i, "여기는 이랑이 아니에요.".into())),
                    Some(k) if self.crop_on_ground(x, y).is_some() || self.plots[k].stage != PlotStage::Empty => {
                        self.events.push((i, "이 이랑에는 벌써 씨앗이나 열매가 있어요.".into()))
                    }
                    Some(k) => {
                        self.plots[k].stage = PlotStage::Seeded;
                        self.events.push((i, "씨앗을 심었어요.".into()));
                    }
                },
                Action::Water => match self.plots.iter().position(|p| p.x == x && p.y == y) {
                    Some(k) if self.plots[k].stage == PlotStage::Seeded => {
                        self.plots[k].stage = PlotStage::Growing { left: GROW_TICKS };
                        self.events.push((i, format!("물을 줬어요. {GROW_TICKS}틱 뒤에 다 자라요.")));
                    }
                    _ => self.events.push((i, "물을 줄 씨앗이 없어요.".into())),
                },
                Action::Store => match self.farmers[i].carrying.take() {
                    None => self.events.push((i, "내려놓을 열매가 없어요.".into())),
                    Some(f) => {
                        if (x, y) == self.barn {
                            self.crops[f].state = CropState::Delivered;
                            self.delivered += 1;
                            self.events.push((i, format!("배달 완료! ({}/{})", self.delivered, self.goal)));
                            if self.respawn {
                                self.spawn_crop();
                            }
                        } else {
                            self.crops[f].state = CropState::Ground;
                            self.crops[f].x = x;
                            self.crops[f].y = y;
                        }
                    }
                },
            }
        }

        // 고블린: 한 칸씩 왕복한다. 돌·물·창고에 닿으면 돌아선다.
        for g in 0..self.goblins.len() {
            let (x, y, dir) = self.goblin_next(&self.goblins[g]);
            self.goblins[g] = Goblin { x, y, dir };
        }

        self.tick += 1;

        // 물을 준 싹은 자라서, 다 자라면 그 이랑에 열매가 열린다.
        for k in 0..self.plots.len() {
            if let PlotStage::Growing { left } = self.plots[k].stage {
                if left <= 1 {
                    let (x, y) = (self.plots[k].x, self.plots[k].y);
                    self.plots[k].stage = PlotStage::Empty;
                    self.crops.push(Crop { x, y, home: (x, y), state: CropState::Ground });
                    self.events.push((0, format!("({x}, {y}) 이랑에 열매가 열렸어요!")));
                } else {
                    self.plots[k].stage = PlotStage::Growing { left: left - 1 };
                }
            }
        }

        for i in 0..self.farmers.len() {
            let (x, y) = (self.farmers[i].x, self.farmers[i].y);
            if self.flooded() && self.ground_at(x, y) == Ground::Paddy {
                self.send_home(i, Mishap::Swept, "홍수에 휩쓸려 창고로 떠밀려 왔어요!");
            } else if self.goblins.iter().any(|g| g.x == x && g.y == y) {
                self.send_home(i, Mishap::Caught, "고블린다! 열매를 떨어뜨리고 창고로 도망쳤어요. (12틱 동안 숨어 있어요)");
                self.farmers[i].hiding = 12;
            }
        }
    }
}
