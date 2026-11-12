# 진공 통로 — 데이터를 품은 enum

반응로와 코어 사이에 **통로**가 있어요. 통로는 기압이 정상일 땐 걸을 수 있지만, 감압돼 진공이 되면 그 위에 있던 로봇이 휩쓸려 반응로로 돌아와요. 들고 있던 코어도 잃고요.

**새로 생긴 것**

- `Tile::Duct` — 기압이 정상인 통로. `Tile::Void` — 허공 (진공이 된 통로도).
- `view.pressure()` 와 `Pressure` — 지금 기압.

```rust
enum Pressure {
    Safe { turns_left: u32 },   // 기압 정상. turns_left 틱 뒤 진공
    Vacuum { turns_left: u32 }, // 진공. turns_left 틱 뒤 기압 정상
    Stable,                     // 기압 변화가 없는 구역
}
```

`Pressure` 의 변형은 **데이터를 품고** 있어요. `match` 로 안의 값을 꺼내고, `if` **가드**로 조건을 붙일 수 있어요.

```rust
match view.pressure() {
    Pressure::Safe { turns_left } if turns_left >= 6 => { /* 건너도 안전 */ }
    _ => { /* 기다리기 */ }
}
```

통로는 다섯 칸이에요. 건너는 동안 진공이 되지 않으려면 언제 발을 들여야 할까요?

목표: 100틱 안에 코어 2개를 반응로로 가져오세요.
