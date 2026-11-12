# 회수 작전 — Vec 과 반복자

코어가 갑판 여기저기에 놓여 있어요. 이번에는 **신호가 닿지 않아서** `core_dir()` 는 늘 `None` 이에요. 대신 코어가 **어디 있는지 전부** 알려 줘요.

**새로 생긴 것**

- `view.pos()` — 지금 서 있는 곳 `(x, y)`. 동쪽으로 갈수록 `x`, **남쪽으로 갈수록 `y`** 가 커져요.
- `view.cores()` — 바닥에 놓인 코어의 위치 목록, `Vec<(i32, i32)>`. 가까운 순서가 아니에요.

`Vec` 은 같은 타입의 값을 여러 개 담는 목록이에요. 반복자로 훑으면서 **가장 가까운 것**을 고를 수 있어요.

```rust
let (px, py) = view.pos();
let nearest = view
    .cores()
    .into_iter()
    .min_by_key(|&(x, y)| (x - px).abs() + (y - py).abs());
```

`min_by_key` 는 값마다 **열쇠**(여기서는 거리)를 만들어 가장 작은 것을 `Option` 에 담아 돌려줘요. 목록이 비어 있으면 `None` 이에요. `|&(x, y)| …` 는 **클로저**, 이름 없는 작은 함수예요.

이제 목표 `(x, y)` 가 정해졌으니, 비교해서 방향을 고르면 돼요.

```rust
if x > px { Dir::East } else if x < px { Dir::West } else if y > py { Dir::South } else { Dir::North }
```

목표: 140틱 안에 코어 6개를 반응로에 넣으세요.
