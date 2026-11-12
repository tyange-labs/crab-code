# 홍수와 물 빠짐 — 데이터를 품은 enum

창고와 열매 사이에 **논**이 있어요. 홍수가 들어오면 논은 물에 잠기고, 그 위에 있던 농부는 휩쓸려 창고로 떠밀려 와요. 들고 있던 열매도 잃고요.

**새로 생긴 것**

- `Tile::Paddy` — 물이 빠진 논. `Tile::Water` — 깊은 물 (홍수에 잠긴 논도).
- `view.flood()` 와 `Flood` — 지금 수위.

```rust
enum Flood {
    Low { turns_left: u32 },  // 물이 빠져 있음. turns_left 틱 뒤 홍수
    High { turns_left: u32 }, // 물이 차 있음. turns_left 틱 뒤 물 빠짐
    Calm,                     // 수위가 없는 밭
}
```

`Flood` 의 변형은 **데이터를 품고** 있어요. `match` 로 안의 값을 꺼내고, `if` **가드**로 조건을 붙일 수 있어요.

```rust
match view.flood() {
    Flood::Low { turns_left } if turns_left >= 6 => { /* 건너도 안전 */ }
    _ => { /* 기다리기 */ }
}
```

논은 다섯 칸이에요. 건너는 동안 물이 들어오지 않으려면 언제 발을 들여야 할까요?

목표: 100틱 안에 열매 2개를 창고로 가져오세요.
