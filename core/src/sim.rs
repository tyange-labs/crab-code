// 월드 시뮬레이션.
//
// 이 파일은 두 곳에서 쓰인다.
//   1. 게임 본체: 화면에 그릴 월드 상태를 관리한다.
//   2. 플레이어 프로그램: 플레이어 코드와 함께 컴파일되어 센서 값(앞이 비었나? 등)을 계산한다.
// 양쪽이 같은 코드를 쓰기 때문에, 플레이어 프로그램이 출력한 행동 목록을 게임이 그대로
// 다시 재생하면 똑같은 결과가 나온다. 그래서 std 외에는 아무것도 쓰면 안 된다.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Red,
    Blue,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sign {
    Straight,
    Left,
    Right,
    Finish,
}

#[derive(Clone, Debug)]
pub struct GemState {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub weight: u32,
    pub shiny: bool,
    pub held: bool,
    pub placed: bool,
}

#[derive(Clone, Debug)]
pub struct Pedestal {
    pub x: i32,
    pub y: i32,
    pub want: Option<Color>,
    pub filled: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Door {
    pub x: i32,
    pub y: i32,
    pub code: u32,
    pub open: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Forward,
    Left,
    Right,
    Collect,
    Deposit(usize),
    Polish(usize),
    Unlock(u32),
}

impl Action {
    pub fn encode(&self) -> String {
        match self {
            Action::Forward => "F".to_string(),
            Action::Left => "L".to_string(),
            Action::Right => "R".to_string(),
            Action::Collect => "C".to_string(),
            Action::Deposit(id) => format!("D {id}"),
            Action::Polish(id) => format!("P {id}"),
            Action::Unlock(code) => format!("U {code}"),
        }
    }

    pub fn decode(s: &str) -> Option<Action> {
        let mut parts = s.split_whitespace();
        let head = parts.next()?;
        let arg = parts.next();
        Some(match head {
            "F" => Action::Forward,
            "L" => Action::Left,
            "R" => Action::Right,
            "C" => Action::Collect,
            "D" => Action::Deposit(arg?.parse().ok()?),
            "P" => Action::Polish(arg?.parse().ok()?),
            "U" => Action::Unlock(arg?.parse().ok()?),
            _ => return None,
        })
    }
}

/// 방향: 0 = 동, 1 = 남, 2 = 서, 3 = 북
pub const DX: [i32; 4] = [1, 0, -1, 0];
pub const DY: [i32; 4] = [0, 1, 0, -1];

#[derive(Clone, Debug)]
pub struct World {
    pub w: i32,
    pub h: i32,
    pub walls: Vec<bool>,
    pub x: i32,
    pub y: i32,
    pub dir: i32,
    pub gems: Vec<GemState>,
    pub pedestals: Vec<Pedestal>,
    pub signs: Vec<(i32, i32, Sign)>,
    pub numbers: Vec<(i32, i32, u32)>,
    pub door: Option<Door>,
    pub flag: Option<(i32, i32)>,
    /// 다이얼: 레벨마다 주어지는 숫자 하나.
    pub dial: Option<u32>,
    /// 두루마리: 숫자 목록.
    pub scroll: Vec<u32>,
    /// 쪽지: 글자들.
    pub note: Option<String>,
    pub note2: Option<String>,
    /// 보석 위에 무게를 표시할지.
    pub show_weights: bool,
}

/// 실행할 때마다 달라지는 값을 만드는 작은 난수 생성기.
pub struct Rng(u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// lo 이상 hi 이하.
    pub fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + (self.next() % (hi - lo + 1) as u64) as u32
    }

    fn numbers(&mut self, len: u32, lo: u32, hi: u32) -> Vec<u32> {
        (0..len).map(|_| self.range(lo, hi)).collect()
    }

    /// 서로 다른 수 len 개.
    fn distinct(&mut self, len: usize, lo: u32, hi: u32) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::new();
        while out.len() < len {
            let n = self.range(lo, hi);
            if !out.contains(&n) {
                out.push(n);
            }
        }
        out
    }

    fn pick<'a>(&mut self, items: &[&'a str]) -> &'a str {
        items[(self.next() % items.len() as u64) as usize]
    }
}

pub struct Spec {
    pub id: &'static str,
    map: &'static [&'static str],
    setup: fn(&mut World, &mut Rng),
}

// 지도 기호
//   #  벽            .  바닥          > v < ^  꽃게 시작 위치와 방향
//   r b  빨강/파랑 보석     d  흐릿한(광 안 난) 빨강 보석
//   ?  실행할 때마다 색이 바뀌는 보석      %  있을 수도 없을 수도 있는 보석
//   *  이 기호가 있는 칸들 중 딱 한 곳에만 보석이 놓인다
//   R B  색이 정해진 받침대   P  아무 색 받침대
//   L T  왼쪽/오른쪽 표지판   F  깃발          D  문
//   1-9  숫자 타일            n  실행할 때마다 바뀌는 숫자 타일 (1~9)
const DOOR_ROOM: &[&str] = &["#####", "#>.D#", "#####"];
const WEIGH_ROW: &[&str] = &["########", "#>rrr.D#", "########"];
const COLOR_ROW: &[&str] = &["##########", "#>?????.D#", "##########"];
const VALUE_ROW: &[&str] = &["##########", "#>?n?n?.D#", "##########"];
const BEND: &[&str] = &[
    "########", "#>...###", "####.###", "####.###", "####.###", "####..r#", "########",
];

pub const SPECS: &[Spec] = &[
    // 2장 러스트 둘러보기
    Spec { id: "call", map: &["#######", "#>..r.#", "#######"], setup: none },
    Spec { id: "turn", map: &["######", "#>..##", "###.##", "###r##", "######"], setup: none },
    Spec { id: "for", map: &["##########", "#>.r.r.r.#", "##########"], setup: none },
    Spec {
        id: "fn",
        map: &["#######", "#####.#", "####.r#", "###.r##", "##.r###", "#>r####", "#######"],
        setup: none,
    },
    Spec { id: "param", map: BEND, setup: none },
    Spec { id: "return", map: DOOR_ROOM, setup: dial_square },
    // 3장 기본 타입
    Spec { id: "mut", map: &["#########", "#>rrrr.D#", "#########"], setup: none },
    Spec { id: "int", map: DOOR_ROOM, setup: dial_times100 },
    Spec {
        id: "tuple",
        map: &["########", "#>*****#", "#******#", "#******#", "#******#", "########"],
        setup: none,
    },
    Spec {
        id: "array",
        map: &["########", "#>.....#", "##r###.#", "##.###.#", "##.....#", "########"],
        setup: none,
    },
    Spec { id: "vec", map: DOOR_ROOM, setup: scroll_ends },
    Spec { id: "string", map: DOOR_ROOM, setup: note_len },
    // 4장 소유와 이동
    Spec { id: "move", map: &["########", "#>rr.PP#", "########"], setup: none },
    Spec { id: "bag", map: &["##########", "#>rrr.PPP#", "##########"], setup: none },
    Spec { id: "clone", map: DOOR_ROOM, setup: note_len },
    // 5장 레퍼런스
    Spec { id: "borrow", map: &["#######", "#R.?.B#", "###^###", "#######"], setup: none },
    Spec { id: "mutborrow", map: &["#######", "#>d..P#", "#######"], setup: none },
    Spec { id: "shared", map: &["########", "#>rr.PP#", "########"], setup: none },
    Spec { id: "lifetime", map: DOOR_ROOM, setup: two_notes_longer },
    // 6장 표현식
    Spec { id: "while", map: &["############", "#>%%%r%%%%%#", "############"], setup: none },
    Spec { id: "ifexpr", map: &["############", "#>%%%r%%%.D#", "############"], setup: none },
    Spec {
        id: "match",
        map: &["#########", "#>..T####", "####.####", "####L..F#", "#########"],
        setup: none,
    },
    Spec { id: "loop", map: &["#########", "#>......#", "#########", "#########"], setup: loop_len },
    // 7장 오류 처리
    Spec { id: "option", map: &["##########", "#>3.5.4.D#", "##########"], setup: none },
    Spec { id: "panic", map: DOOR_ROOM, setup: scroll_last },
    Spec { id: "result", map: DOOR_ROOM, setup: note_parse },
    Spec { id: "question", map: DOOR_ROOM, setup: two_notes_sum },
    // 9장 스트럭트
    Spec { id: "fields", map: WEIGH_ROW, setup: weights_total },
    Spec { id: "struct", map: COLOR_ROW, setup: color_count },
    Spec { id: "impl", map: WEIGH_ROW, setup: weights_total },
    Spec { id: "derive", map: &["#######", "#>..r.#", "#######"], setup: none },
    // 10장 이늄과 패턴
    Spec {
        id: "enumdata",
        map: &["########", "###...r#", "###.####", "###.####", "#>..####", "########"],
        setup: none,
    },
    Spec { id: "pattern", map: &["###########", "#>nnnnnn.D#", "###########"], setup: evens },
    // 11장 트레이트와 제네릭
    Spec { id: "trait", map: VALUE_ROW, setup: value_total },
    Spec { id: "generic", map: VALUE_ROW, setup: value_total },
    // 12·13장 연산자 오버로딩과 유틸리티 트레이트
    Spec { id: "add", map: COLOR_ROW, setup: score },
    Spec {
        id: "copy",
        map: &["######", "#>...#", "####.#", "####.#", "####r#", "######"],
        setup: none,
    },
    Spec { id: "from", map: &["########", "#>???.D#", "########"], setup: digits },
    // 14장 클로저
    Spec { id: "closure", map: &["#########", "#>rrrr.D#", "#########"], setup: none },
    Spec {
        id: "callback",
        map: &["######", "#>..r#", "#.##.#", "#.##.#", "#r..r#", "######"],
        setup: none,
    },
    // 15장 이터레이터
    Spec { id: "sum", map: DOOR_ROOM, setup: scroll_sum },
    Spec { id: "filter", map: DOOR_ROOM, setup: scroll_big },
    Spec { id: "enumerate", map: &["#########", "#>......#", "#########"], setup: scroll_max_gem },
    Spec { id: "fold", map: DOOR_ROOM, setup: dial_factorial },
    // 16장 컬렉션
    Spec { id: "hashmap", map: DOOR_ROOM, setup: scroll_mode },
    Spec { id: "hashset", map: DOOR_ROOM, setup: scroll_distinct },
    Spec { id: "sort", map: DOOR_ROOM, setup: scroll_median },
    // 17장 문자열과 텍스트
    Spec { id: "chars", map: DOOR_ROOM, setup: note_a_count },
    Spec { id: "split", map: DOOR_ROOM, setup: note_numbers },
    Spec {
        id: "commands",
        map: &[
            "#########", "#.......#", "#.......#", "#.......#", "#...>...#", "#.......#",
            "#.......#", "#.......#", "#########",
        ],
        setup: commands,
    },
    Spec {
        id: "final",
        map: &[
            "#########",
            "#..b....#",
            "#.#####.#",
            "#.#F..#.#",
            "#.###.#b#",
            "#..r..#.#",
            "#######.#",
            "#>..r...#",
            "#########",
        ],
        setup: none,
    },
];

pub const LEVEL_COUNT: usize = SPECS.len();

fn set_code(w: &mut World, code: u32) {
    if let Some(door) = &mut w.door {
        door.code = code;
    }
}

fn none(_: &mut World, _: &mut Rng) {}

fn dial_square(w: &mut World, r: &mut Rng) {
    let d = r.range(2, 9);
    w.dial = Some(d);
    set_code(w, d * d);
}

fn dial_times100(w: &mut World, r: &mut Rng) {
    let d = r.range(3, 9);
    w.dial = Some(d);
    set_code(w, d * 100);
}

fn dial_factorial(w: &mut World, r: &mut Rng) {
    let d = r.range(4, 8);
    w.dial = Some(d);
    set_code(w, (1..=d).product());
}

fn scroll_ends(w: &mut World, r: &mut Rng) {
    let len = r.range(4, 6);
    w.scroll = r.numbers(len, 1, 50);
    set_code(w, w.scroll[0] + w.scroll[w.scroll.len() - 1]);
}

fn scroll_last(w: &mut World, r: &mut Rng) {
    let len = r.range(4, 6);
    w.scroll = r.numbers(len, 1, 50);
    set_code(w, w.scroll[w.scroll.len() - 1]);
}

fn scroll_sum(w: &mut World, r: &mut Rng) {
    let len = r.range(5, 7);
    w.scroll = r.numbers(len, 1, 30);
    set_code(w, w.scroll.iter().sum());
}

fn scroll_big(w: &mut World, r: &mut Rng) {
    w.scroll = r.numbers(7, 1, 99);
    // 전부 50보다 크면 "그냥 다 세기" 가 통해 버린다.
    w.scroll[0] = r.range(1, 50);
    set_code(w, w.scroll.iter().filter(|n| **n > 50).count() as u32);
}

fn scroll_max_gem(w: &mut World, r: &mut Rng) {
    let mut v = r.distinct(6, 1, 99);
    let mut idx = (0..v.len()).max_by_key(|i| v[*i]).unwrap();
    if idx == 0 {
        // 가장 큰 수가 맨 앞이면 "0칸" 이 정답이 되어 버리니 자리를 바꾼다.
        let to = r.range(1, 5) as usize;
        v.swap(0, to);
        idx = to;
    }
    w.scroll = v;
    w.gems.push(GemState {
        x: w.x + 1 + idx as i32,
        y: w.y,
        color: Color::Blue,
        weight: 5,
        shiny: true,
        held: false,
        placed: false,
    });
}

fn scroll_mode(w: &mut World, r: &mut Rng) {
    // 한 수는 4번, 나머지는 많아야 2번 나온다.
    let vals = r.distinct(4, 1, 9);
    let mut v = vec![vals[0], vals[0], vals[0], vals[0], vals[1], vals[1], vals[2], vals[2], vals[3]];
    for i in (1..v.len()).rev() {
        let j = (r.next() % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
    w.scroll = v;
    set_code(w, vals[0]);
}

fn scroll_distinct(w: &mut World, r: &mut Rng) {
    w.scroll = r.numbers(8, 1, 5);
    let mut seen: Vec<u32> = Vec::new();
    for n in &w.scroll {
        if !seen.contains(n) {
            seen.push(*n);
        }
    }
    set_code(w, seen.len() as u32);
}

fn scroll_median(w: &mut World, r: &mut Rng) {
    let mut v = r.distinct(5, 1, 99);
    // 이미 정렬된 채로 나오지 않게 한다.
    if v.windows(2).all(|p| p[0] < p[1]) {
        v.swap(0, 4);
    }
    w.scroll = v.clone();
    v.sort();
    set_code(w, v[2]);
}

const WORDS: &[&str] = &["crab", "ferris", "borrow", "cargo", "lifetime", "trait", "ownership", "rustacean"];

fn note_len(w: &mut World, r: &mut Rng) {
    let word = r.pick(WORDS);
    w.note = Some(word.to_string());
    set_code(w, word.len() as u32);
}

fn two_notes_longer(w: &mut World, r: &mut Rng) {
    let a = r.pick(WORDS);
    let mut b = r.pick(WORDS);
    while b.len() == a.len() {
        b = r.pick(WORDS);
    }
    w.note = Some(a.to_string());
    w.note2 = Some(b.to_string());
    set_code(w, a.len().max(b.len()) as u32);
}

fn note_parse(w: &mut World, r: &mut Rng) {
    if r.next() % 2 == 0 {
        let n = r.range(10, 99);
        w.note = Some(n.to_string());
        set_code(w, n);
    } else {
        w.note = Some(r.pick(&["4x2", "seven", "1o1", "9?"]).to_string());
        set_code(w, 0);
    }
}

fn two_notes_sum(w: &mut World, r: &mut Rng) {
    let a = r.range(10, 60);
    let b = r.range(10, 60);
    w.note = Some(a.to_string());
    if r.next() % 3 == 0 {
        w.note2 = Some(r.pick(&["2O", "x7", "five"]).to_string());
        set_code(w, 0);
    } else {
        w.note2 = Some(b.to_string());
        set_code(w, a + b);
    }
}

fn note_a_count(w: &mut World, r: &mut Rng) {
    let word = r.pick(&["banana", "avocado", "papaya", "alpaca", "panama", "cargo", "abracadabra"]);
    w.note = Some(word.to_string());
    set_code(w, word.chars().filter(|c| *c == 'a').count() as u32);
}

fn note_numbers(w: &mut World, r: &mut Rng) {
    let len = r.range(3, 5);
    let v = r.numbers(len, 1, 40);
    let words: Vec<String> = v.iter().map(|n| n.to_string()).collect();
    w.note = Some(words.join(" "));
    set_code(w, v.iter().sum());
}

fn weights_total(w: &mut World, _: &mut Rng) {
    w.show_weights = true;
    set_code(w, w.gems.iter().map(|g| g.weight).sum());
}

fn color_count(w: &mut World, _: &mut Rng) {
    let red = w.gems.iter().filter(|g| g.color == Color::Red).count() as u32;
    let blue = w.gems.len() as u32 - red;
    set_code(w, red * 10 + blue);
}

fn evens(w: &mut World, _: &mut Rng) {
    // 홀수가 적어도 하나는 있게 해서 "그냥 다 더하기" 가 통하지 않게 한다.
    if let Some(first) = w.numbers.first_mut() {
        first.2 |= 1;
    }
    set_code(w, w.numbers.iter().map(|n| n.2).filter(|n| n % 2 == 0).sum());
}

fn value_total(w: &mut World, _: &mut Rng) {
    w.show_weights = true;
    let gems: u32 = w.gems.iter().map(|g| g.weight * 10).sum();
    let nums: u32 = w.numbers.iter().map(|n| n.2).sum();
    set_code(w, gems + nums);
}

fn score(w: &mut World, _: &mut Rng) {
    w.show_weights = true;
    let red: u32 = w.gems.iter().filter(|g| g.color == Color::Red).map(|g| g.weight).sum();
    let blue: u32 = w.gems.iter().filter(|g| g.color == Color::Blue).map(|g| g.weight).sum();
    set_code(w, red * 100 + blue);
}

fn digits(w: &mut World, _: &mut Rng) {
    let mut code = 0;
    for g in &w.gems {
        code = code * 10 + if g.color == Color::Red { 1 } else { 2 };
    }
    set_code(w, code);
}

fn loop_len(w: &mut World, r: &mut Rng) {
    // 복도 길이를 바꾸고, 복도 끝 칸의 오른쪽(아래)에 문을 둔다.
    let len = r.range(3, 6) as i32;
    let end = w.x + len;
    for x in end + 1..w.w {
        w.walls[(w.y * w.w + x) as usize] = true;
    }
    let (dx, dy) = (end, w.y + 1);
    w.walls[(dy * w.w + dx) as usize] = false;
    w.door = Some(Door {
        x: dx,
        y: dy,
        code: len as u32,
        open: false,
    });
}

fn commands(w: &mut World, r: &mut Rng) {
    let (sx, sy) = (w.x, w.y);
    let (mut x, mut y, mut d) = (sx, sy, w.dir);
    let mut note = String::new();
    let mut steps = 0;
    while steps < 11 || (x, y) == (sx, sy) || !note.ends_with('F') {
        let (fx, fy) = (x + DX[d as usize], y + DY[d as usize]);
        if !w.is_wall(fx, fy) && r.next() % 3 != 0 {
            note.push('F');
            x = fx;
            y = fy;
        } else if r.next() % 2 == 0 {
            note.push('L');
            d = (d + 3) % 4;
        } else {
            note.push('R');
            d = (d + 1) % 4;
        }
        steps += 1;
    }
    w.note = Some(note);
    w.flag = Some((x, y));
    w.signs.push((x, y, Sign::Finish));
}

fn mix(seed: u64, x: i32, y: i32) -> u64 {
    let mut z = seed
        .wrapping_add((x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add((y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F))
        .wrapping_add(0x1234_5678_9ABC_DEF1);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn build_level(idx: usize, seed: u64) -> World {
    let spec = &SPECS[idx.min(SPECS.len() - 1)];
    let map = spec.map;
    let h = map.len() as i32;
    let w = map[0].chars().count() as i32;
    let mut world = World {
        w,
        h,
        walls: vec![false; (w * h) as usize],
        x: 0,
        y: 0,
        dir: 0,
        gems: Vec::new(),
        pedestals: Vec::new(),
        signs: Vec::new(),
        numbers: Vec::new(),
        door: None,
        flag: None,
        dial: None,
        scroll: Vec::new(),
        note: None,
        note2: None,
        show_weights: false,
    };
    let gem = |x: i32, y: i32, color: Color, shiny: bool| GemState {
        x,
        y,
        color,
        weight: 1 + (mix(seed ^ 0x77, x, y) % 9) as u32,
        shiny,
        held: false,
        placed: false,
    };
    let pedestal = |x: i32, y: i32, want: Option<Color>| Pedestal {
        x,
        y,
        want,
        filled: None,
    };
    let mut candidates: Vec<(i32, i32)> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let random_color = if mix(seed, x, y) % 2 == 0 {
                Color::Red
            } else {
                Color::Blue
            };
            match ch {
                '#' => world.walls[(y * w + x) as usize] = true,
                '>' | 'v' | '<' | '^' => {
                    world.x = x;
                    world.y = y;
                    world.dir = match ch {
                        '>' => 0,
                        'v' => 1,
                        '<' => 2,
                        _ => 3,
                    };
                }
                'r' => world.gems.push(gem(x, y, Color::Red, true)),
                'b' => world.gems.push(gem(x, y, Color::Blue, true)),
                'd' => world.gems.push(gem(x, y, Color::Red, false)),
                '?' => world.gems.push(gem(x, y, random_color, true)),
                '%' => {
                    if mix(seed ^ 0xABCD, x, y) % 5 < 2 {
                        world.gems.push(gem(x, y, random_color, true));
                    }
                }
                '*' => candidates.push((x, y)),
                'R' => world.pedestals.push(pedestal(x, y, Some(Color::Red))),
                'B' => world.pedestals.push(pedestal(x, y, Some(Color::Blue))),
                'P' => world.pedestals.push(pedestal(x, y, None)),
                'L' => world.signs.push((x, y, Sign::Left)),
                'T' => world.signs.push((x, y, Sign::Right)),
                'F' => {
                    world.flag = Some((x, y));
                    world.signs.push((x, y, Sign::Finish));
                }
                'D' => {
                    world.door = Some(Door {
                        x,
                        y,
                        code: 0,
                        open: false,
                    })
                }
                '1'..='9' => world.numbers.push((x, y, ch as u32 - '0' as u32)),
                'n' => world.numbers.push((x, y, 1 + (mix(seed ^ 0x55, x, y) % 9) as u32)),
                _ => {}
            }
        }
    }
    if !candidates.is_empty() {
        let (x, y) = candidates[(mix(seed, 3, 9) % candidates.len() as u64) as usize];
        world.gems.push(gem(x, y, Color::Blue, true));
    }
    // 문의 기본 비밀번호: 숫자 타일이 있으면 그 합, 없으면 보석 개수.
    let code = if world.numbers.is_empty() {
        world.gems.len() as u32
    } else {
        world.numbers.iter().map(|n| n.2).sum()
    };
    set_code(&mut world, code);
    let mut rng = Rng(seed ^ 0x5EED_5EED);
    (spec.setup)(&mut world, &mut rng);
    world
}

impl World {
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            return true;
        }
        self.walls[(y * self.w + x) as usize]
    }

    fn blocked(&self, x: i32, y: i32) -> bool {
        if self.is_wall(x, y) {
            return true;
        }
        match &self.door {
            Some(d) => d.x == x && d.y == y && !d.open,
            None => false,
        }
    }

    pub fn front(&self) -> (i32, i32) {
        (
            self.x + DX[self.dir as usize],
            self.y + DY[self.dir as usize],
        )
    }

    pub fn front_is_clear(&self) -> bool {
        let (fx, fy) = self.front();
        !self.blocked(fx, fy)
    }

    pub fn gem_here(&self) -> Option<usize> {
        self.gems
            .iter()
            .position(|g| !g.held && !g.placed && g.x == self.x && g.y == self.y)
    }

    /// 아직 줍지 않은 첫 보석까지의 거리: (앞으로 몇 칸, 오른쪽으로 몇 칸).
    pub fn gem_offset(&self) -> (u32, u32) {
        let Some(g) = self.gems.iter().find(|g| !g.held && !g.placed) else {
            return (0, 0);
        };
        let (dx, dy) = (g.x - self.x, g.y - self.y);
        let d = self.dir as usize;
        let r = (d + 1) % 4;
        let ahead = dx * DX[d] + dy * DY[d];
        let right = dx * DX[r] + dy * DY[r];
        (ahead.max(0) as u32, right.max(0) as u32)
    }

    pub fn read_sign(&self) -> Sign {
        self.signs
            .iter()
            .find(|s| s.0 == self.x && s.1 == self.y)
            .map(|s| s.2)
            .unwrap_or(Sign::Straight)
    }

    pub fn read_number(&self) -> Option<u32> {
        self.numbers
            .iter()
            .find(|n| n.0 == self.x && n.1 == self.y)
            .map(|n| n.2)
    }

    pub fn apply(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Forward => {
                let (fx, fy) = self.front();
                if let Some(d) = &self.door {
                    if d.x == fx && d.y == fy && !d.open {
                        return Err("쿵! 문이 잠겨 있어요. unlock(비밀번호) 로 먼저 여세요.".into());
                    }
                }
                if self.is_wall(fx, fy) {
                    return Err("쿵! 벽에 부딪혔어요.".into());
                }
                self.x = fx;
                self.y = fy;
            }
            Action::Left => self.dir = (self.dir + 3) % 4,
            Action::Right => self.dir = (self.dir + 1) % 4,
            Action::Collect => match self.gem_here() {
                Some(id) => self.gems[id].held = true,
                None => return Err("여기에는 주울 보석이 없어요.".into()),
            },
            Action::Deposit(id) => {
                let (x, y) = (self.x, self.y);
                let Some(pi) = self.pedestals.iter().position(|p| p.x == x && p.y == y) else {
                    return Err("받침대 위에 서 있을 때만 보석을 올릴 수 있어요.".into());
                };
                let Some(gem) = self.gems.get(id) else {
                    return Err("그런 보석은 없어요.".into());
                };
                if self.pedestals[pi].filled.is_some() {
                    return Err("이 받침대에는 이미 보석이 있어요.".into());
                }
                if let Some(want) = self.pedestals[pi].want {
                    if want != gem.color {
                        return Err("받침대와 보석의 색이 달라요!".into());
                    }
                }
                if !gem.shiny {
                    return Err("흐릿한 보석은 올릴 수 없어요. polish 로 광을 내세요.".into());
                }
                self.pedestals[pi].filled = Some(id);
                let gem = &mut self.gems[id];
                gem.held = false;
                gem.placed = true;
                gem.x = x;
                gem.y = y;
            }
            Action::Polish(id) => match self.gems.get_mut(id) {
                Some(gem) => gem.shiny = true,
                None => return Err("그런 보석은 없어요.".into()),
            },
            Action::Unlock(code) => {
                let (fx, fy) = self.front();
                match &mut self.door {
                    Some(d) if d.x == fx && d.y == fy => {
                        if d.code == code {
                            d.open = true;
                        } else {
                            return Err(format!("삐빅! 비밀번호 {code} 은(는) 틀렸어요."));
                        }
                    }
                    _ => return Err("바로 앞에 문이 없어요.".into()),
                }
            }
        }
        Ok(())
    }

    pub fn is_won(&self) -> bool {
        self.todo_hint().is_none()
    }

    /// 아직 남은 목표가 있으면 그 설명을 돌려준다.
    pub fn todo_hint(&self) -> Option<String> {
        let left = self.gems.iter().filter(|g| !g.held && !g.placed).count();
        if left > 0 {
            return Some(format!("아직 줍지 않은 보석이 {left}개 남았어요."));
        }
        // 받침대가 있는 레벨에서는 모든 보석을 받침대에 올려야 한다.
        let unplaced = self.gems.iter().filter(|g| !g.placed).count();
        if !self.pedestals.is_empty() && unplaced > 0 {
            return Some(format!("받침대에 올리지 않은 보석이 {unplaced}개 남았어요."));
        }
        if let Some(d) = &self.door {
            if !d.open {
                return Some("문이 아직 잠겨 있어요.".into());
            }
        }
        if let Some((fx, fy)) = self.flag {
            if (self.x, self.y) != (fx, fy) {
                return Some("깃발까지 가야 해요.".into());
            }
        }
        None
    }
}
