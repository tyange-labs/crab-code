# 씨 뿌리고 물 주고 — 튜플 match 와 Option 이어 붙이기

이번 밭에는 열매가 없어요. 대신 **이랑** 세 개가 있어요. 씨앗을 심고, 물을 주고, 몇 틱 기다리면 열매가 열려요. 그 열매를 거둬서 창고에 넣으면 돼요.

**새로 생긴 것**

- `Action::Plant` — 발밑 이랑에 씨앗을 심어요.
- `Action::Water` — 발밑 이랑의 씨앗에 물을 줘요. 물을 줘야 자라요 (4틱 뒤 열매).
- `Tile::Plot` · `Tile::Seed` · `Tile::Sprout` — 빈 이랑 · 씨앗을 심은 이랑 · 자라는 중인 싹.
- `view.plot_dir()` — 손이 필요한 이랑(빈 이랑이나 물을 안 준 씨앗)으로 가는 첫걸음. 없으면 `None`.

한 이랑은 이렇게 돌아가요: `Plot` →(심기)→ `Seed` →(물)→ `Sprout` →(4틱)→ `Crop` →(거두기)→ `Plot`.

발밑 칸과 들고 있는지를 **한꺼번에** 보려면 값 두 개를 **튜플**로 묶어 `match` 해요.

```rust
match (view.here(), view.carrying()) {
    (Tile::Plot, _) => Action::Plant,
    (Tile::Crop, false) => Action::Harvest,
    (Tile::Barn, true) => Action::Store,
    _ => Action::Wait,
}
```

`_` 는 "아무 값이나"예요. 그리고 `Option` 두 개를 이을 땐 `a.or(b)` 가 좋아요. `a` 가 `Some` 이면 그걸, `None` 이면 `b` 를 돌려줘요.

```rust
let target = view.plot_dir().or(view.crop_dir());
```

기다리는 동안 놀지 말고 다음 이랑으로 가야 시간 안에 끝나요.

목표: 70틱 안에 열매 3개를 창고에 넣으세요.
