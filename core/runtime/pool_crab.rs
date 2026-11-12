// 꽃게 AI 모드: 플레이어 코드가 쓰는 것들.
// 게임 본체에는 포함되지 않고, 플레이어 코드와 함께 rustc 로 컴파일된다.
//
// 플레이어는 Crab 트레이트를 구현한 Ferris 를 만든다. 이 파일의 run() 이
// 꽃게 수만큼 Ferris::new(번호) 를 만들고, 매 틱 꽃게마다 tick(&View) 을 불러 행동을 받는다.
// 행동과 꽃게 상태({:?})는 "@@" 로 시작하는 줄로 출력되고, 게임이 그 줄을 읽어 다시 재생한다.

use crate::pool;
pub use crate::pool::{Action, Dir, Tide, Tile, View};
use std::fmt::Debug;

//>> 참고
/// 꽃게의 두뇌. 게임이 꽃게마다 new 로 하나씩 만들고, 매 틱 tick 을 부른다.
/// 타임라인에서 상태를 보여 주려고 Debug 가 필요하다: 구조체에 #[derive(Debug)] 를 붙이자.
pub trait Crab: Debug {
    /// 꽃게를 하나 만든다. id 는 0부터 차례로 붙는 번호.
    fn new(id: usize) -> Self
    where
        Self: Sized;

    /// 이번 틱에 할 일을 고른다. view 는 이 꽃게가 지금 보는 것 (빌려 받는다).
    fn tick(&mut self, view: &View) -> Action;
}
//<< 참고

fn env_num(name: &str) -> u64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(0)
}

#[doc(hidden)]
pub fn run<C: Crab>() {
    let mut world = pool::build(env_num("CRAB_LEVEL") as usize, env_num("CRAB_SEED"));
    let mut crabs: Vec<C> = (0..world.crabs.len()).map(C::new).collect();
    while !world.is_done() {
        let mut actions = Vec::with_capacity(crabs.len());
        for (i, crab) in crabs.iter_mut().enumerate() {
            let view = world.view(i);
            println!("@@T {} {}", world.tick, i);
            let action = crab.tick(&view);
            println!("@@A {}", action.encode());
            let mut state = format!("{:?}", crab).replace('\n', " ");
            if state.chars().count() > 300 {
                state = state.chars().take(300).collect::<String>() + "…";
            }
            println!("@@S {}", state);
            actions.push(action);
        }
        world.step(&actions);
    }
    println!("@@E");
}
