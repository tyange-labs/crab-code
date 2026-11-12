# 기억하는 로봇 — 필드와 &mut self

벽으로 막힌 미로 속이라 코어 신호가 **한 칸**밖에 닿지 않아요 (`core_dir()` 이 바로 옆에서만 `Some`). 반응로 신호(`reactor_dir()`)는 여전히 잘 잡혀요.

**새로 생긴 것**

- `view.look(dir)` — 그쪽 바로 옆 칸.
- `Tile::Wall` 과 `tile.is_open()` — 걸어 들어갈 수 있는 칸이면 `true`.
- `dir.turn_right()`, `dir.turn_left()`, `dir.opposite()` — 돈 방향을 돌려주는 `Dir` 의 메서드.

길을 찾으려면 **오른손 법칙**을 써 봐요: 오른쪽 벽에 손을 대고 걸으면 언젠가 미로를 다 돌아요. 그러려면 "지금 어느 쪽으로 가던 중인지" 기억해야 하는데, `tick` 이 끝나면 지역 변수는 사라져요.

로봇 구조체에 **필드**를 두면 틱이 지나도 남아요. `tick` 이 `&mut self` 를 받기 때문에 필드를 바꿀 수 있어요.

```rust
#[derive(Debug)]
struct Echo {
    heading: Dir,
}
```

게임 창에서 로봇을 누르면 틱마다 `self` 의 모습(`Echo { heading: South }`)을 볼 수 있어요.

목표: 미로 속 코어를 찾아 반응로로 가져오세요.
