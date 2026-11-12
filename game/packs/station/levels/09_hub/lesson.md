# 허브 가동 — 일을 enum 으로 나누기

이번에는 로봇이 **세 명**이고 슬롯이 열두 개예요. 같은 코드가 세 번 돌지만 `self` 는 로봇마다 따로예요. 그러니 `self` 에 **지금 하는 일**을 적어 두면, 로봇마다 자기 일을 이어 갈 수 있어요.

**새로 생긴 것**은 없어요. 지금까지 배운 것(`slot_dir()`, `core_dir()`, `reactor_dir()`, 튜플 `match`)으로 짜요. 달라지는 건 **짜는 방법**이에요.

일을 `enum` 으로 이름 붙여요.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,    // 슬롯 돌보기: 설치하고, 충전하고, 생긴 코어 집기
    Deliver, // 집은 코어를 반응로로
}

#[derive(Debug)]
struct Echo {
    job: Job,
}
```

그리고 `tick` 에서 (지금 일, 발밑, 들고 있는지) 를 **튜플로 묶어 `match`** 해요. 일이 바뀔 때는 `self.job = …` 으로 적어 둬요.

```rust
match (self.job, view.here(), view.carrying()) {
    (Job::Tend, Tile::Core, false) => {
        self.job = Job::Deliver; // 집으면 이제 배달할 차례
        return Action::Grab;
    }
    (Job::Deliver, Tile::Reactor, true) => {
        self.job = Job::Tend;
        return Action::Insert;
    }
    _ => {}
}
```

`Debug` 를 붙여 두었으니 게임 창에서 로봇을 누르면 `self` 의 `job` 이 틱마다 어떻게 바뀌는지 볼 수 있어요.

슬롯은 `Slot` →(설치)→ `Module` →(충전)→ `Booting` →(4틱)→ `Core` 순서로 변하는 걸 잊지 마세요.

목표: 170틱 안에 코어 9개를 반응로에 넣으세요.
