# 냄새를 따라 — Option 과 match

이번 웅덩이는 실행할 때마다 먹이 자리가 바뀌어요. 방향을 코드에 박아 두면 안 되겠죠?

`view.food_dir()` 는 가장 가까운 먹이로 가는 첫걸음을 `Option<Dir>` 로 알려 줘요.

- `Some(dir)` 그쪽으로 한 칸 가면 먹이에 가까워져요.
- `None` 냄새가 나지 않아요.

`view.nest_dir()` 는 굴로 가는 첫걸음이고, 이미 굴 위라면 `None` 이에요. `Option` 은 `match` 로 꺼내 씁니다.

```rust
match view.nest_dir() {
    Some(dir) => Action::Move(dir),
    None => Action::Drop, // 굴에 도착!
}
```

목표: 90틱 안에 먹이 3개를 굴로 모으세요.
