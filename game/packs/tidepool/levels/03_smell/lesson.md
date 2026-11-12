# 냄새를 따라 — Option 과 match

이번 웅덩이는 실행할 때마다 먹이 자리가 바뀌어요. 방향을 코드에 박아 두면 안 되겠죠?

**새로 생긴 것**

- `view.food_dir()` — 가장 가까운 먹이로 가는 첫걸음.
- `view.nest_dir()` — 굴로 가는 첫걸음.

둘 다 `Option<Dir>` 을 돌려줘요. `Option` 은 값이 **있을 수도 없을 수도** 있을 때 쓰는 타입이에요.

- `Some(dir)` — 그쪽으로 한 칸 가면 가까워져요.
- `None` — 갈 곳이 없어요 (먹이 냄새가 안 나거나, 이미 굴 위에 있어요).

`Option` 안의 값은 `match` 로 꺼내 써요.

```rust
match view.nest_dir() {
    Some(dir) => Action::Move(dir),
    None => Action::Drop, // 굴에 도착!
}
```

목표: 90틱 안에 먹이 3개를 굴로 모으세요.
