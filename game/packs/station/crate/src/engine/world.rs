// 게임 엔진: 기압·경비 드론·코어가 있는 틱 시뮬레이션과 정거장 구역 지도.
//
// 플레이어는 이 파일을 몰라도 된다. 플레이어가 쓰는 것은 레벨마다의 모듈(level1.rs …)에 있고,
// 거기 있는 Action·View 등은 bridge.rs 에서 이 파일의 것으로 바뀐다.
// 플레이어 프로그램 안에서 돈다. 게임 창에는 매 틱의 모습을 출력해서 보낸다 (run.rs).
// 플레이어 컴퓨터에서 인터넷 없이 바로 빌드되도록 std 외에는 아무것도 쓰지 않는다.

use std::collections::VecDeque;

/// 방향. 로봇은 몸을 돌리지 않고 어느 쪽으로든 걸어 간다.
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

/// 로봇이 보는 칸.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// 갑판. 걸어갈 수 있다.
    Deck,
    /// 벽. 못 지나간다.
    Wall,
    /// 허공 (진공이 된 통로 포함). 못 들어간다.
    Void,
    /// 기압이 정상인 통로. 걸어갈 수 있지만 감압(진공)이 되면 휩쓸린다.
    Duct,
    /// 코어가 놓인 칸.
    Core,
    /// 로봇 반응로. 코어를 여기 내려놓으면 된다.
    Reactor,
    /// 경비 드론이 있는 칸.
    Sentry,
    /// 빈 슬롯. 모듈을 설치할 수 있다.
    Slot,
    /// 모듈을 설치한 슬롯. 충전해야 켜진다.
    Module,
    /// 충전해서 부팅 중인 모듈.
    Booting,
}

impl Tile {
    /// 지금 걸어 들어갈 수 있는 칸인가.
    pub fn is_open(self) -> bool {
        matches!(self, Tile::Deck | Tile::Duct | Tile::Core | Tile::Reactor | Tile::Slot | Tile::Module | Tile::Booting)
    }
}

/// 로봇이 한 틱에 할 수 있는 일.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// 가만히 있는다.
    Wait,
    /// 그쪽으로 한 칸 걸어요한다.
    Move(Dir),
    /// 발밑의 코어를 집는다.
    Grab,
    /// 들고 있는 코어를 내려놓는다. 반응로 위라면 장착 완료(배달).
    Insert,
    /// 발밑 슬롯에 모듈을 설치한다.
    Install,
    /// 발밑 슬롯의 모듈을 충전한다.
    Charge,
}

impl Action {
    pub fn encode(&self) -> String {
        match self {
            Action::Wait => "W".into(),
            Action::Move(d) => format!("M{}", d.code()),
            Action::Grab => "P".into(),
            Action::Insert => "D".into(),
            Action::Install => "S".into(),
            Action::Charge => "A".into(),
        }
    }

    pub fn decode(s: &str) -> Option<Action> {
        let mut chars = s.trim().chars();
        Some(match chars.next()? {
            'W' => Action::Wait,
            'M' => Action::Move(Dir::from_code(chars.next()?)?),
            'P' => Action::Grab,
            'D' => Action::Insert,
            'S' => Action::Install,
            'A' => Action::Charge,
            _ => return None,
        })
    }
}

/// 기압. 기압 변화가 없는 구역에서는 Stable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pressure {
    /// 기압이 정상이다. turns_left 틱 뒤에 진공이 된다.
    Safe { turns_left: u32 },
    /// 진공이다. turns_left 틱 뒤에 기압이 돌아온다.
    Vacuum { turns_left: u32 },
    /// 기압 변화가 없는 구역.
    Stable,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ground {
    Deck,
    Wall,
    Void,
    Duct,
    Reactor,
    Slot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mishap {
    None,
    /// 벽이나 허공에 막혔다.
    Bump,
    /// 진공에 휩쓸려 반응로로 돌아갔다.
    Swept,
    /// 경비 드론에게 쫓겨 반응로로 돌아갔다.
    Caught,
}

#[derive(Clone, Debug)]
pub struct BotState {
    pub x: i32,
    pub y: i32,
    /// 들고 있는 코어 번호.
    pub carrying: Option<usize>,
    /// 마지막으로 움직인 방향 (그림용).
    pub facing: Dir,
    pub mishap: Mishap,
    /// 경비 드론에게 쫓겨 반응로에 숨어 있는 남은 틱. 그동안은 행동하지 못한다.
    pub hiding: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CoreState {
    Ground,
    Carried,
    Delivered,
}

#[derive(Clone, Debug)]
pub struct Core {
    pub x: i32,
    pub y: i32,
    pub home: (i32, i32),
    pub state: CoreState,
}

#[derive(Clone, Debug)]
pub struct Sentry {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
}

/// 슬롯 한 칸의 상태.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SlotStage {
    /// 비어 있다.
    Empty,
    /// 모듈을 설치했다. 충전해야 켜진다.
    Installed,
    /// 충전했다. left 틱 뒤에 코어가 생긴다.
    Charging { left: u32 },
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub x: i32,
    pub y: i32,
    pub stage: SlotStage,
}

/// 멀리 볼 때 실패하는 까닭.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sight {
    /// 지도 밖이다.
    OffMap,
    /// 너무 멀다 (시야는 걸음 수로 sight 칸).
    TooFar,
}

/// 충전한 뒤 코어가 생길 때까지 걸리는 틱.
pub const CHARGE_TICKS: u32 = 4;

/// 로봇이 한 틱에 보는 것. 플레이어 코드의 tick() 이 &View 로 빌려 받는다.
#[derive(Clone, Debug)]
pub struct View {
    id: usize,
    tick: u32,
    pos: (i32, i32),
    here: Tile,
    around: [Tile; 4],
    danger: [bool; 4],
    carrying: bool,
    core_dir: Option<Dir>,
    reactor_dir: Option<Dir>,
    slot_dir: Option<Dir>,
    ripe: Vec<(i32, i32)>,
    radar: String,
    size: (i32, i32),
    sight: i32,
    grid: Vec<Tile>,
    pressure: Pressure,
    delivered: u32,
    goal: u32,
}

impl View {
    /// 이 로봇의 번호 (0부터).
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

    /// 발밑 칸: Deck, Duct, Core, Reactor 중 하나.
    pub fn here(&self) -> Tile {
        self.here
    }

    /// 그쪽 바로 옆 칸.
    pub fn look(&self, dir: Dir) -> Tile {
        self.around[dir_index(dir)]
    }

    /// 그쪽으로 한 칸 가면 다음 틱에 경비 드론에게 잡히는가.
    pub fn danger(&self, dir: Dir) -> bool {
        self.danger[dir_index(dir)]
    }

    /// 코어를 들고 있는가.
    pub fn carrying(&self) -> bool {
        self.carrying
    }

    /// 가장 가까운 코어로 가는 첫걸음. 탐지 범위 밖이면 None.
    pub fn core_dir(&self) -> Option<Dir> {
        self.core_dir
    }

    /// 반응로로 가는 첫걸음. 이미 반응로 위라면 None.
    pub fn reactor_dir(&self) -> Option<Dir> {
        self.reactor_dir
    }

    /// 레이더가 보내는 글. 코어마다 `C:x,y` 를 공백으로 이어 붙이고, 가끔 잡음이 섞인다.
    pub fn radar(&self) -> &str {
        &self.radar
    }

    /// 지도 크기 (가로, 세로).
    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    /// 볼 수 있는 거리 (걸음 수).
    pub fn sight(&self) -> i32 {
        self.sight
    }

    /// 아무 칸이나 들여다본다. 지도 밖이거나 너무 멀면 Err.
    pub fn tile_at(&self, x: i32, y: i32) -> Result<Tile, Sight> {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            return Err(Sight::OffMap);
        }
        if (x - self.pos.0).abs() + (y - self.pos.1).abs() > self.sight {
            return Err(Sight::TooFar);
        }
        Ok(self.grid[(y * self.size.0 + x) as usize])
    }

    /// 바닥에 놓인 코어 전부의 위치. 가까운 순서가 아니라 놓인 순서예요.
    pub fn ripe(&self) -> Vec<(i32, i32)> {
        self.ripe.clone()
    }

    /// 손이 필요한 슬롯(빈 슬롯이나 충전 안 한 모듈)으로 가는 첫걸음. 없으면 None.
    pub fn slot_dir(&self) -> Option<Dir> {
        self.slot_dir
    }

    /// 지금 기압.
    pub fn pressure(&self) -> Pressure {
        self.pressure
    }

    /// 지금까지 반응로에 배달한 코어 수 (로봇 모두 합쳐서).
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
    /// 로봇 수 (모두 반응로에서 시작).
    bots: usize,
    /// 목표 배달 수.
    pub goal: u32,
    /// 이 틱 안에 끝내야 한다.
    pub max_ticks: u32,
    /// 코어 탐지가 닿는 거리 (걸음 수).
    smell: u32,
    /// (주기, 그중 진공인 틱 수)
    pressure: Option<(u32, u32)>,
    /// '?' 칸들 중 코어를 놓을 개수.
    random_core: usize,
    /// 코어 칸에 닿기만 하면 성공 (배달하지 않아도 된다).
    reach: bool,
    /// 배달하면 '?' 칸 어딘가에 새 코어가 생긴다.
    respawn: bool,
    /// false 면 slot_dir() 이 늘 None (슬롯은 직접 찾아야 한다).
    slot_sense: bool,
    /// tile_at 으로 볼 수 있는 거리 (걸음 수).
    sight: i32,
}

// 지도 기호
//   #  벽       .  갑판       N  반응로 (로봇이 여기서 시작)
//   f  코어       ?  코어가 놓일 수 있는 자리
//   ~  통로 (진공이 되면 휩쓸린다)      w  허공
//   p  슬롯 (모듈을 설치하고 충전하면 코어가 생긴다)
//   g  가로로 왕복하는 경비 드론             G  세로로 왕복하는 경비 드론
pub const SPECS: &[Spec] = &[
    Spec {
        id: "tick",
        map: &["##########", "#N.....f.#", "##########"],
        bots: 1,
        goal: 1,
        max_ticks: 20,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: true,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "carry",
        map: &["##########", "#N.....f.#", "##########"],
        bots: 1,
        goal: 1,
        max_ticks: 30,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
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
        bots: 1,
        goal: 3,
        max_ticks: 90,
        smell: u32::MAX,
        pressure: None,
        random_core: 3,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
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
        bots: 1,
        goal: 1,
        max_ticks: 150,
        smell: 1,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "vacuum",
        map: &[
            "#############",
            "#N..~~~~~..f#",
            "#...~~~~~...#",
            "#...~~~~~..f#",
            "#############",
        ],
        bots: 1,
        goal: 2,
        max_ticks: 100,
        smell: u32::MAX,
        pressure: Some((12, 5)),
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "swarm",
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
        bots: 3,
        goal: 10,
        max_ticks: 90,
        smell: u32::MAX,
        pressure: None,
        random_core: 3,
        respawn: true,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "install",
        map: &[
            "#############",
            "#N..........#",
            "#...........#",
            "#..p..p..p..#",
            "#...........#",
            "#############",
        ],
        bots: 1,
        goal: 3,
        max_ticks: 70,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "salvage",
        map: &[
            "###############",
            "#N............#",
            "#..?.....?....#",
            "#......?......#",
            "#?.......?..?.#",
            "#....?........#",
            "#.?.......?...#",
            "###############",
        ],
        bots: 1,
        goal: 6,
        max_ticks: 140,
        smell: 0,
        pressure: None,
        random_core: 7,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "hub",
        map: &[
            "###############",
            "#N............#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "###############",
        ],
        bots: 3,
        goal: 9,
        max_ticks: 170,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "sentries",
        map: &[
            "###############",
            "#N............#",
            "#.....G.....G.#",
            "#.p.p.p.p.p.p.#",
            "#g............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "###############",
        ],
        bots: 3,
        goal: 9,
        max_ticks: 220,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "chores",
        map: &[
            "###############",
            "#N............#",
            "#.....G.....G.#",
            "#.p.p.p.p.p.p.#",
            "#g............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "###############",
        ],
        bots: 3,
        goal: 9,
        max_ticks: 220,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 7,
    },
    Spec {
        id: "scout",
        map: &[
            "###############",
            "#N............#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "###############",
        ],
        bots: 2,
        goal: 8,
        max_ticks: 220,
        smell: 0,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: false,
        sight: 7,
    },
    Spec {
        id: "fog",
        map: &[
            "#####################",
            "#N...................#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#####################",
        ],
        bots: 2,
        goal: 8,
        max_ticks: 160,
        smell: 0,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: false,
        sight: 3,
    },
    Spec {
        id: "relay",
        map: &[
            "###############",
            "#N............#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "#.p.p.p.p.p.p.#",
            "#.............#",
            "###############",
        ],
        bots: 4,
        goal: 12,
        max_ticks: 130,
        smell: u32::MAX,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 30,
    },
    Spec {
        id: "radar",
        map: &[
            "###############",
            "#N............#",
            "#..?.....?....#",
            "#......?......#",
            "#?.......?..?.#",
            "#....?........#",
            "#.?.......?...#",
            "###############",
        ],
        bots: 1,
        goal: 6,
        max_ticks: 160,
        smell: 0,
        pressure: None,
        random_core: 7,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 30,
    },
    Spec {
        id: "ledger",
        map: &[
            "#####################",
            "#N...................#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#..p..p..p..p..p..p..#",
            "#....................#",
            "#####################",
        ],
        bots: 2,
        goal: 8,
        max_ticks: 160,
        smell: 0,
        pressure: None,
        random_core: 0,
        respawn: false,
        reach: false,
        slot_sense: false,
        sight: 3,
    },
    Spec {
        id: "route",
        map: &[
            "#################",
            "#N....#.....#...#",
            "#.###.#.###.#.#.#",
            "#.?.#...#.?.#.#.#",
            "###.#####.###.#.#",
            "#.....#...#...#.#",
            "#.###.#.#.#.###.#",
            "#.#?..#.#.....#.#",
            "#.#####.#######.#",
            "#...?...?.....?.#",
            "#################",
        ],
        bots: 1,
        goal: 3,
        max_ticks: 260,
        smell: 0,
        pressure: None,
        random_core: 3,
        respawn: false,
        reach: false,
        slot_sense: true,
        sight: 40,
    },
];

pub const LEVEL_COUNT: usize = SPECS.len();

#[derive(Clone, Debug)]
pub struct Field {
    pub w: i32,
    pub h: i32,
    pub ground: Vec<Ground>,
    pub reactor: (i32, i32),
    pub bots: Vec<BotState>,
    pub cores: Vec<Core>,
    pub slots: Vec<Slot>,
    pub sentries: Vec<Sentry>,
    pub tick: u32,
    pub delivered: u32,
    pub goal: u32,
    pub max_ticks: u32,
    pub smell: u32,
    pub pressure: Option<(u32, u32)>,
    pub respawn: bool,
    pub reach: bool,
    pub slot_sense: bool,
    pub sight: i32,
    pub spots: Vec<(i32, i32)>,
    rng: u64,
    /// 지난 틱에 일어난 일: (로봇 번호, 설명)
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
        ground: vec![Ground::Deck; (w * h) as usize],
        reactor: (0, 0),
        bots: Vec::new(),
        cores: Vec::new(),
        slots: Vec::new(),
        sentries: Vec::new(),
        tick: 0,
        delivered: 0,
        goal: spec.goal,
        max_ticks: spec.max_ticks,
        smell: spec.smell,
        pressure: spec.pressure,
        respawn: spec.respawn,
        reach: spec.reach,
        slot_sense: spec.slot_sense,
        sight: spec.sight,
        spots: Vec::new(),
        rng: seed ^ 0xC0FF_EE15_BEEF,
        events: Vec::new(),
    };
    for (y, row) in spec.map.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let i = (y * w + x) as usize;
            match ch {
                '#' => field.ground[i] = Ground::Wall,
                'w' => field.ground[i] = Ground::Void,
                '~' => field.ground[i] = Ground::Duct,
                'p' => {
                    field.ground[i] = Ground::Slot;
                    field.slots.push(Slot { x, y, stage: SlotStage::Empty });
                }
                'N' => {
                    field.ground[i] = Ground::Reactor;
                    field.reactor = (x, y);
                }
                'f' => field.cores.push(Core { x, y, home: (x, y), state: CoreState::Ground }),
                '?' => field.spots.push((x, y)),
                'g' => field.sentries.push(Sentry { x, y, dir: Dir::East }),
                'G' => field.sentries.push(Sentry { x, y, dir: Dir::South }),
                _ => {}
            }
        }
    }
    for _ in 0..spec.bots {
        field.bots.push(BotState {
            x: field.reactor.0,
            y: field.reactor.1,
            carrying: None,
            facing: Dir::East,
            mishap: Mishap::None,
            hiding: 0,
        });
    }
    for _ in 0..spec.random_core {
        field.spawn_core();
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
        self.idx(x, y).map(|i| self.ground[i]).unwrap_or(Ground::Wall)
    }

    /// 진공 상태인가.
    pub fn vented(&self) -> bool {
        matches!(self.pressure_now(), Pressure::Vacuum { .. })
    }

    pub fn pressure_now(&self) -> Pressure {
        match self.pressure {
            None => Pressure::Stable,
            Some((period, high)) => {
                let phase = self.tick % period;
                let low = period - high;
                if phase < low {
                    Pressure::Safe { turns_left: low - phase }
                } else {
                    Pressure::Vacuum { turns_left: period - phase }
                }
            }
        }
    }

    fn spawn_core(&mut self) {
        let free: Vec<(i32, i32)> = self
            .spots
            .iter()
            .copied()
            .filter(|&(x, y)| !self.cores.iter().any(|f| f.state == CoreState::Ground && (f.x, f.y) == (x, y)))
            .collect();
        if free.is_empty() {
            return;
        }
        let (x, y) = free[(splitmix(&mut self.rng) % free.len() as u64) as usize];
        self.cores.push(Core { x, y, home: (x, y), state: CoreState::Ground });
    }

    fn radar_text(&self) -> String {
        let mut tokens: Vec<String> = self
            .cores
            .iter()
            .filter(|f| f.state == CoreState::Ground)
            .map(|f| format!("C:{},{}", f.x, f.y))
            .collect();
        // 잡음: 해독할 수 없는 조각이 가끔 섞인다 (틱으로 정해져서 늘 같다).
        if self.tick % 4 == 1 {
            tokens.insert(tokens.len() / 2, "C:?,?".to_string());
        }
        if self.tick % 5 == 2 {
            tokens.insert(0, "###".to_string());
        }
        tokens.join(" ")
    }

    fn slot_at(&self, x: i32, y: i32) -> Option<&Slot> {
        self.slots.iter().find(|p| p.x == x && p.y == y)
    }

    /// 손이 필요한 슬롯인가: 코어가 안 생겼고, 비었거나 충전 안 한 모듈.
    fn slot_needs_work(&self, x: i32, y: i32) -> bool {
        self.core_on_ground(x, y).is_none()
            && matches!(self.slot_at(x, y).map(|p| p.stage), Some(SlotStage::Empty) | Some(SlotStage::Installed))
    }

    fn core_on_ground(&self, x: i32, y: i32) -> Option<usize> {
        self.cores.iter().position(|f| f.state == CoreState::Ground && f.x == x && f.y == y)
    }

    /// 로봇이 걸어 다닐 수 있는 칸 (기압은 따지지 않는다).
    fn walkable(&self, x: i32, y: i32) -> bool {
        matches!(self.ground_at(x, y), Ground::Deck | Ground::Duct | Ground::Reactor | Ground::Slot)
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        if self.sentries.iter().any(|g| g.x == x && g.y == y) {
            return Tile::Sentry;
        }
        self.ground_tile(x, y)
    }

    fn ground_tile(&self, x: i32, y: i32) -> Tile {
        match self.ground_at(x, y) {
            Ground::Wall => Tile::Wall,
            Ground::Void => Tile::Void,
            Ground::Duct if self.vented() => Tile::Void,
            Ground::Reactor => Tile::Reactor,
            g => {
                if self.core_on_ground(x, y).is_some() {
                    Tile::Core
                } else if g == Ground::Slot {
                    match self.slot_at(x, y).map(|p| p.stage) {
                        Some(SlotStage::Installed) => Tile::Module,
                        Some(SlotStage::Charging { .. }) => Tile::Booting,
                        _ => Tile::Slot,
                    }
                } else if g == Ground::Duct {
                    Tile::Duct
                } else {
                    Tile::Deck
                }
            }
        }
    }

    /// from 에서 target 중 가장 가까운 곳으로 가는 첫걸음과 거리.
    /// 벽·허공은 돌아간다. 통로는 기압과 상관없이 길로 친다. (로봇끼리는 서로 지나갈 수 있다.)
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

    /// 경비 드론이 다음 틱에 갈 칸과 방향: 앞이 막히면 돌아선다.
    fn sentry_next(&self, g: &Sentry) -> (i32, i32, Dir) {
        let ok = |d: Dir| {
            let (dx, dy) = d.delta();
            matches!(self.ground_at(g.x + dx, g.y + dy), Ground::Deck | Ground::Duct | Ground::Slot)
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

    /// 그 칸에 서 있으면 다음 틱에 경비 드론에게 잡히는가.
    fn dangerous(&self, x: i32, y: i32) -> bool {
        self.sentries.iter().any(|g| {
            let (nx, ny, _) = self.sentry_next(g);
            (nx, ny) == (x, y)
        })
    }

    pub fn view(&self, i: usize) -> View {
        let c = &self.bots[i];
        let mut around = [Tile::Wall; 4];
        let mut danger = [false; 4];
        for d in Dir::ALL {
            let (dx, dy) = d.delta();
            around[dir_index(d)] = self.tile_at(c.x + dx, c.y + dy);
            danger[dir_index(d)] = self.dangerous(c.x + dx, c.y + dy);
        }
        let core_dir = if c.carrying.is_some() {
            None
        } else {
            self.first_step((c.x, c.y), |x, y| self.core_on_ground(x, y).is_some())
                .filter(|&(_, dist)| dist <= self.smell)
                .map(|(d, _)| d)
        };
        let reactor = self.reactor;
        View {
            id: i,
            tick: self.tick,
            pos: (c.x, c.y),
            here: self.ground_tile(c.x, c.y),
            around,
            danger,
            carrying: c.carrying.is_some(),
            core_dir,
            reactor_dir: self.first_step((c.x, c.y), |x, y| (x, y) == reactor).map(|(d, _)| d),
            ripe: self.cores.iter().filter(|f| f.state == CoreState::Ground).map(|f| (f.x, f.y)).collect(),
            slot_dir: if self.slot_sense {
                self.first_step((c.x, c.y), |x, y| self.slot_needs_work(x, y)).map(|(d, _)| d)
            } else {
                None
            },
            radar: self.radar_text(),
            size: (self.w, self.h),
            sight: self.sight,
            grid: (0..self.h).flat_map(|gy| (0..self.w).map(move |gx| (gx, gy))).map(|(gx, gy)| self.tile_at(gx, gy)).collect(),
            pressure: self.pressure_now(),
            delivered: self.delivered,
            goal: self.goal,
        }
    }

    pub fn is_won(&self) -> bool {
        if self.reach {
            return self.bots.iter().any(|c| self.core_on_ground(c.x, c.y).is_some());
        }
        self.delivered >= self.goal
    }

    pub fn is_done(&self) -> bool {
        self.is_won() || self.tick >= self.max_ticks
    }

    fn send_home(&mut self, i: usize, why: Mishap, text: &str) {
        if let Some(f) = self.bots[i].carrying.take() {
            let home = self.cores[f].home;
            self.cores[f].state = CoreState::Ground;
            self.cores[f].x = home.0;
            self.cores[f].y = home.1;
        }
        self.bots[i].x = self.reactor.0;
        self.bots[i].y = self.reactor.1;
        self.bots[i].mishap = why;
        self.events.push((i, text.to_string()));
    }

    /// 로봇마다 고른 행동으로 한 틱을 진행한다.
    pub fn step(&mut self, actions: &[Action]) {
        self.events.clear();
        for i in 0..self.bots.len() {
            self.bots[i].mishap = Mishap::None;
            if self.bots[i].hiding > 0 {
                self.bots[i].hiding -= 1;
                continue;
            }
            let action = actions.get(i).copied().unwrap_or(Action::Wait);
            let (x, y) = (self.bots[i].x, self.bots[i].y);
            match action {
                Action::Wait => {}
                Action::Move(d) => {
                    let (dx, dy) = d.delta();
                    let (nx, ny) = (x + dx, y + dy);
                    self.bots[i].facing = d;
                    let deep = self.ground_at(nx, ny) == Ground::Void
                        || (self.ground_at(nx, ny) == Ground::Duct && self.vented());
                    if !self.walkable(nx, ny) || deep {
                        self.bots[i].mishap = Mishap::Bump;
                        self.events.push((i, "쿵! 그쪽으로는 못 가요.".into()));
                    } else {
                        self.bots[i].x = nx;
                        self.bots[i].y = ny;
                        if let Some(f) = self.bots[i].carrying {
                            self.cores[f].x = nx;
                            self.cores[f].y = ny;
                        }
                    }
                }
                Action::Grab => {
                    if self.bots[i].carrying.is_some() {
                        self.events.push((i, "이미 코어를 들고 있어요.".into()));
                    } else if let Some(f) = self.core_on_ground(x, y) {
                        self.cores[f].state = CoreState::Carried;
                        self.bots[i].carrying = Some(f);
                    } else {
                        self.events.push((i, "여기에는 집을 코어가 없어요.".into()));
                    }
                }
                Action::Install => match self.slots.iter().position(|p| p.x == x && p.y == y) {
                    None => self.events.push((i, "여기는 슬롯이 아니에요.".into())),
                    Some(k) if self.core_on_ground(x, y).is_some() || self.slots[k].stage != SlotStage::Empty => {
                        self.events.push((i, "이 슬롯에는 이미 모듈이나 코어가 있어요.".into()))
                    }
                    Some(k) => {
                        self.slots[k].stage = SlotStage::Installed;
                        self.events.push((i, "모듈을 설치했어요.".into()));
                    }
                },
                Action::Charge => match self.slots.iter().position(|p| p.x == x && p.y == y) {
                    Some(k) if self.slots[k].stage == SlotStage::Installed => {
                        self.slots[k].stage = SlotStage::Charging { left: CHARGE_TICKS };
                        self.events.push((i, format!("충전을 시작했어요. 부팅까지 {CHARGE_TICKS}틱.")));
                    }
                    _ => self.events.push((i, "충전할 모듈이 없어요.".into())),
                },
                Action::Insert => match self.bots[i].carrying.take() {
                    None => self.events.push((i, "내려놓을 코어가 없어요.".into())),
                    Some(f) => {
                        if (x, y) == self.reactor {
                            self.cores[f].state = CoreState::Delivered;
                            self.delivered += 1;
                            self.events.push((i, format!("코어 장착 완료! ({}/{})", self.delivered, self.goal)));
                            if self.respawn {
                                self.spawn_core();
                            }
                        } else {
                            self.cores[f].state = CoreState::Ground;
                            self.cores[f].x = x;
                            self.cores[f].y = y;
                        }
                    }
                },
            }
        }

        // 경비 드론: 한 칸씩 왕복한다. 벽·허공·반응로에 닿으면 돌아선다.
        for g in 0..self.sentries.len() {
            let (x, y, dir) = self.sentry_next(&self.sentries[g]);
            self.sentries[g] = Sentry { x, y, dir };
        }

        self.tick += 1;

        // 충전한 모듈은 부팅을 마치면 그 슬롯에 코어가 생긴다.
        for k in 0..self.slots.len() {
            if let SlotStage::Charging { left } = self.slots[k].stage {
                if left <= 1 {
                    let (x, y) = (self.slots[k].x, self.slots[k].y);
                    self.slots[k].stage = SlotStage::Empty;
                    self.cores.push(Core { x, y, home: (x, y), state: CoreState::Ground });
                    self.events.push((0, format!("({x}, {y}) 슬롯에서 코어가 생겼어요!")));
                } else {
                    self.slots[k].stage = SlotStage::Charging { left: left - 1 };
                }
            }
        }

        for i in 0..self.bots.len() {
            let (x, y) = (self.bots[i].x, self.bots[i].y);
            if self.vented() && self.ground_at(x, y) == Ground::Duct {
                self.send_home(i, Mishap::Swept, "진공에 휩쓸려 반응로로 되돌아왔어요!");
            } else if self.sentries.iter().any(|g| g.x == x && g.y == y) {
                self.send_home(i, Mishap::Caught, "경비 드론이에요! 코어를 잃고 반응로로 쫓겨났어요. (12틱 동안 숨어 있어요)");
                self.bots[i].hiding = 12;
            }
        }
    }
}
