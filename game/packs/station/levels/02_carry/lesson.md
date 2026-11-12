# 코어 나르기 — if 와 Tile

이번엔 코어를 **반응로까지 가져와야** 해요. 그러려면 로봇이 상황을 보고 할 일을 골라야 해요.

**새로 생긴 것**

- `Action::Grab` — 발밑의 코어를 집어요.
- `Action::Insert` — 들고 있는 코어를 내려놓아요. 반응로 위라면 장착이 끝나서 목표 개수에 세어져요!
- `view.here()` — 발밑 칸. `Tile::Deck`, `Tile::Core`, `Tile::Reactor` 중 하나예요.
- `view.carrying()` — 코어를 들고 있으면 `true`.

`Grab` 도 `Wait` 처럼 **값**이에요. `tick` 이 `Action::Grab` 을 돌려주면 게임이 그 틱에 코어를 집어 줘요.

Rust 의 `if` 는 값을 돌려줄 수 있어서, 갈래마다 `Action` 을 하나씩 두면 그게 `tick` 의 답이 돼요.

```rust
if view.carrying() {
    // 들고 있을 때
} else if view.here() == Tile::Core {
    Action::Grab
} else {
    Action::Move(Dir::East)
}
```

목표: 동쪽의 코어를 반응로까지 가져오세요.
