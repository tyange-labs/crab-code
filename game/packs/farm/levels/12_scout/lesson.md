# 정찰 — Result 와 ?

이번 밭에서는 `plot_dir()` 가 늘 `None` 이고 `crop_dir()` 도 소용없어요. **이랑이 어디 있는지 직접 찾아야** 해요.

**새로 생긴 것**

- `view.size()` — 지도 크기 `(가로, 세로)`.
- `view.tile_at(x, y)` — 아무 칸이나 들여다봐요. 그런데 **실패할 수 있어서** `Result<Tile, SightError>` 를 돌려줘요.
- `SightError::OffMap` — 지도 밖. `SightError::TooFar` — 너무 멀어요 (시야는 걸어서 7걸음).

`Option` 은 "있거나 없거나"였죠. **`Result<T, E>`** 는 "성공(`Ok(값)`)이거나 실패(`Err(까닭)`)" 예요. 까닭을 알려 준다는 게 `None` 과 달라요.

```rust
match view.tile_at(x, y) {
    Ok(Tile::Plot) => { /* 빈 이랑을 찾았어요 */ }
    Ok(_) => { /* 다른 칸 */ }
    Err(SightError::TooFar) => { /* 아직 안 보여요. 가까이 가면 보일 거예요 */ }
    Err(SightError::OffMap) => { /* 지도 밖 */ }
}
```

함수 안에서 `Result` 를 또 `Result` 로 돌려줄 땐 **`?`** 가 편해요. `Err` 이면 그 자리에서 함수를 끝내고 그 `Err` 를 그대로 돌려줘요.

```rust
fn worth_a_visit(view: &View, x: i32, y: i32) -> Result<bool, SightError> {
    let tile = view.tile_at(x, y)?; // 안 보이면 여기서 Err 를 돌려주고 끝
    Ok(matches!(tile, Tile::Plot | Tile::Seed | Tile::Crop))
}
```

받는 쪽에서는 `unwrap_or(기본값)` 으로 "실패하면 이 값으로 치자" 라고 할 수 있어요.

```rust
if worth_a_visit(view, x, y).unwrap_or(false) { /* 후보 */ }
```

지도 전체(`0..w`, `0..h`)를 훑으며 후보 중 가장 가까운 칸을 고르고, 8레벨처럼 그쪽으로 걸어가세요. (`Tile::Plot` 이면 `Plant`, `Seed` 면 `Water`, `Crop` 이면 `Harvest`.)

목표: 220틱 안에 열매 8개를 창고에 넣으세요.
