# 신호를 따라 — Option 과 match

이번 구역은 실행할 때마다 코어 자리가 바뀌어요. 방향을 코드에 박아 두면 안 되겠죠? 로봇이 코어의 신호를 탐지해 길을 찾아요.

**새로 생긴 것**

- `view.core_dir()` — 가장 가까운 코어로 가는 첫걸음.
- `view.reactor_dir()` — 반응로로 가는 첫걸음.

둘 다 `Option<Dir>` 을 돌려줘요. `Option` 은 값이 **있을 수도 없을 수도** 있을 때 쓰는 타입이에요.

- `Some(dir)` — 그쪽으로 한 칸 가면 가까워져요.
- `None` — 갈 곳이 없어요 (코어 신호가 잡히지 않거나, 이미 반응로 위에 있어요).

`Option` 안의 값은 `match` 로 꺼내 써요.

```rust
match view.reactor_dir() {
    Some(dir) => Action::Move(dir),
    None => Action::Insert, // 반응로에 도착!
}
```

목표: 90틱 안에 코어 3개를 반응로로 모으세요.
