# 냄새를 따라 — Option 과 match

이번 밭는 실행할 때마다 열매 자리가 바뀌어요. 방향을 코드에 박아 두면 안 되겠죠?

**새로 생긴 것**

- `view.crop_dir()` — 가장 가까운 열매로 가는 첫걸음.
- `view.barn_dir()` — 창고로 가는 첫걸음.

둘 다 `Option<Dir>` 을 돌려줘요. `Option` 은 값이 **있을 수도 없을 수도** 있을 때 쓰는 타입이에요.

- `Some(dir)` — 그쪽으로 한 칸 가면 가까워져요.
- `None` — 갈 곳이 없어요 (열매 냄새가 안 나거나, 이미 창고 위에 있어요).

`Option` 안의 값은 `match` 로 꺼내 써요.

```rust
match view.barn_dir() {
    Some(dir) => Action::Move(dir),
    None => Action::Store, // 창고에 도착!
}
```

목표: 90틱 안에 열매 3개를 창고로 모으세요.
