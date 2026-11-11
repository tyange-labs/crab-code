// 플레이어 코드에서 부를 수 있는 함수들.
// 게임 본체에는 포함되지 않고, 플레이어 코드와 함께 rustc 로 컴파일된다.
// 행동은 "@@" 로 시작하는 줄로 표준 출력에 기록되고, 게임이 그 줄을 읽어 재생한다.

use crate::sim::{self, Action, World};
pub use crate::sim::{Color, Sign};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static WORLD: Mutex<Option<World>> = Mutex::new(None);
static STEPS: AtomicUsize = AtomicUsize::new(0);
const STEP_LIMIT: usize = 400;

fn env_num(name: &str) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn with<T>(f: impl FnOnce(&mut World) -> T) -> T {
    let mut guard = WORLD.lock().unwrap_or_else(|e| e.into_inner());
    let world = guard.get_or_insert_with(|| {
        sim::build_level(env_num("CRAB_LEVEL") as usize, env_num("CRAB_SEED"))
    });
    f(world)
}

fn act(action: Action) {
    if STEPS.fetch_add(1, Ordering::SeqCst) >= STEP_LIMIT {
        println!("@@X");
        std::process::exit(0);
    }
    println!("@@{}", action.encode());
    if with(|w| w.apply(action)).is_err() {
        // 게임이 같은 행동을 재생하다가 같은 이유로 멈추고 이유를 보여 준다.
        std::process::exit(0);
    }
}

/// 보석. Copy 도 Clone 도 없어서 주인은 언제나 하나뿐이다.
#[derive(Debug)]
pub struct Gem {
    id: usize,
}

/// 앞으로 한 칸 이동한다.
pub fn move_forward() {
    act(Action::Forward);
}

/// 제자리에서 왼쪽으로 90도 돈다.
pub fn turn_left() {
    act(Action::Left);
}

/// 제자리에서 오른쪽으로 90도 돈다.
pub fn turn_right() {
    act(Action::Right);
}

/// 발밑의 보석을 줍는다. 주운 보석의 소유권을 돌려준다.
pub fn collect() -> Gem {
    let id = with(|w| w.gem_here()).unwrap_or(usize::MAX);
    act(Action::Collect);
    Gem { id }
}

/// 발밑 받침대에 보석을 올린다. 보석의 소유권을 가져간다.
pub fn deposit(gem: Gem) {
    act(Action::Deposit(gem.id));
}

/// 보석의 색을 확인한다. 빌려서 보기만 한다.
pub fn color_of(gem: &Gem) -> Color {
    with(|w| w.gems[gem.id].color)
}

/// 보석에 광을 낸다. 보석을 바꾸므로 가변 참조가 필요하다.
pub fn polish(gem: &mut Gem) {
    act(Action::Polish(gem.id));
}

/// 바로 앞의 문에 비밀번호를 입력한다.
pub fn unlock(code: u32) {
    act(Action::Unlock(code));
}

/// 앞 칸으로 갈 수 있으면 true.
pub fn front_is_clear() -> bool {
    with(|w| w.front_is_clear())
}

/// 발밑에 보석이 있으면 true.
pub fn on_gem() -> bool {
    with(|w| w.gem_here().is_some())
}

/// 발밑의 표지판을 읽는다. 표지판이 없으면 Sign::Straight.
pub fn read_sign() -> Sign {
    with(|w| w.read_sign())
}

/// 발밑의 숫자 타일을 읽는다. 숫자가 없으면 None.
pub fn read_number() -> Option<u32> {
    with(|w| w.read_number())
}

/// inspect 가 돌려주는 보석 정보.
#[derive(Debug, Clone, Copy)]
pub struct GemInfo {
    pub color: Color,
    pub weight: u32,
    pub shiny: bool,
}

/// 보석을 자세히 살펴본다. 빌려서 보기만 한다.
pub fn inspect(gem: &Gem) -> GemInfo {
    with(|w| {
        let g = &w.gems[gem.id];
        GemInfo {
            color: g.color,
            weight: g.weight,
            shiny: g.shiny,
        }
    })
}

/// 다이얼에 적힌 숫자를 읽는다.
pub fn read_dial() -> u32 {
    with(|w| w.dial.unwrap_or(0))
}

/// 두루마리에 적힌 숫자 목록을 읽는다.
pub fn read_scroll() -> Vec<u32> {
    with(|w| w.scroll.clone())
}

/// 쪽지에 적힌 글을 읽는다.
pub fn read_note() -> String {
    with(|w| w.note.clone().unwrap_or_default())
}

/// 쪽지 두 장을 한꺼번에 읽는다.
pub fn read_two_notes() -> (String, String) {
    with(|w| (w.note.clone().unwrap_or_default(), w.note2.clone().unwrap_or_default()))
}

/// 보석까지의 거리: (앞으로 몇 칸, 오른쪽으로 몇 칸).
pub fn gem_offset() -> (u32, u32) {
    with(|w| w.gem_offset())
}
