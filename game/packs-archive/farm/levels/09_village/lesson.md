# 마을 농사 — 일을 enum 으로 나누기

이번에는 농부가 **세 명**이고 이랑이 열두 개예요. 같은 코드가 세 번 돌지만 `self` 는 농부마다 따로예요. 그러니 `self` 에 **지금 하는 일**을 적어 두면, 농부마다 자기 일을 이어 갈 수 있어요.

**새로 생긴 것**은 없어요. 지금까지 배운 것(`plot_dir()`, `crop_dir()`, `barn_dir()`, 튜플 `match`)으로 짜요. 달라지는 건 **짜는 방법**이에요.

일을 `enum` 으로 이름 붙여요.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Job {
    Tend,    // 이랑 돌보기: 심고, 물 주고, 열린 열매 거두기
    Deliver, // 거둔 열매를 창고로
}

#[derive(Debug)]
struct Pip {
    job: Job,
}
```

그리고 `tick` 에서 (지금 일, 발밑, 들고 있는지) 를 **튜플로 묶어 `match`** 해요. 일이 바뀔 때는 `self.job = …` 으로 적어 둬요.

```rust
match (self.job, view.here(), view.carrying()) {
    (Job::Tend, Tile::Crop, false) => {
        self.job = Job::Deliver; // 거두면 이제 배달할 차례
        return Action::Harvest;
    }
    (Job::Deliver, Tile::Barn, true) => {
        self.job = Job::Tend;
        return Action::Store;
    }
    _ => {}
}
```

`Debug` 를 붙여 두었으니 게임 창에서 농부를 누르면 `self` 의 `job` 이 틱마다 어떻게 바뀌는지 볼 수 있어요.

이랑은 `Plot` →(심기)→ `Seed` →(물)→ `Sprout` →(4틱)→ `Crop` 순서로 변하는 걸 잊지 마세요.

목표: 170틱 안에 열매 9개를 창고에 넣으세요.
