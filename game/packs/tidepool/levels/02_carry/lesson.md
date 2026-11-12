# 먹이 나르기 — if 와 Tile

이번엔 먹이를 **굴까지 가져와야** 해요. 그러려면 꽃게가 상황을 보고 할 일을 골라야 해요.

**새로 생긴 것**

- `Action::Pick` — 발밑의 먹이를 집어요.
- `Action::Drop` — 들고 있는 먹이를 내려놓아요. 굴 위라면 배달 완료!
- `view.here()` — 발밑 칸. `Tile::Sand`, `Tile::Food`, `Tile::Nest` 중 하나예요.
- `view.carrying()` — 먹이를 들고 있으면 `true`.

`Pick` 도 `Wait` 처럼 **값**이에요. `tick` 이 `Action::Pick` 을 돌려주면 게임이 그 틱에 먹이를 집어 줘요.

Rust 의 `if` 는 값을 돌려줄 수 있어서, 갈래마다 `Action` 을 하나씩 두면 그게 `tick` 의 답이 돼요.

```rust
if view.carrying() {
    // 들고 있을 때
} else if view.here() == Tile::Food {
    Action::Pick
} else {
    Action::Move(Dir::East)
}
```

목표: 동쪽의 먹이를 굴까지 가져오세요.
