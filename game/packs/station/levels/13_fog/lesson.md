# 센서 정전 — HashMap 으로 지도 기억하기

구역이 넓고 **정전으로 센서가 꺼졌어요.** `tile_at` 으로는 걸어서 `view.sight()` 걸음 안쪽만 보여요 (이번엔 3걸음). 한 번 본 슬롯도 멀어지면 다시 안 보여요. 그러니 **본 것을 기억해 둬야** 해요.

**새로 생긴 것**

- `view.sight()` — 볼 수 있는 거리.

기억할 곳으로 **`HashMap<K, V>`** 가 좋아요. 열쇠(`K`)로 값(`V`)을 바로 찾는 사전이에요. 여기서는 열쇠를 칸 좌표 `(x, y)`, 값을 그 칸에서 마지막으로 본 `Tile` 로 해요.

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Echo {
    seen: HashMap<(i32, i32), Tile>,
}
```

(`Tile` 은 `Debug` 를 구현하고 있어서 `Echo` 에 `#[derive(Debug)]` 가 그대로 돼요. 게임 창에서 로봇을 눌러 `seen` 이 늘어나는 걸 볼 수 있어요.)

매 틱 주변을 훑어서 기억을 고쳐 써요. 같은 열쇠로 `insert` 하면 값이 새것으로 바뀌어요.

```rust
let (px, py) = view.pos();
let r = view.sight();
for dy in -r..=r {
    for dx in -r..=r {
        if let Ok(tile) = view.tile_at(px + dx, py + dy) {
            self.seen.insert((px + dx, py + dy), tile);
        }
    }
}
```

가 볼 곳은 기억에서 골라요. `HashMap` 은 `iter()` 로 `(&열쇠, &값)` 쌍을 훑어요.

```rust
let target = self
    .seen
    .iter()
    .filter(|(_, t)| matches!(**t, Tile::Slot | Tile::Module | Tile::Core))
    .map(|(&at, _)| at)
    .min_by_key(|&(x, y)| (x - px).abs() + (y - py).abs());
```

기억에 후보가 없으면 **아직 한 번도 못 본 칸**(`seen` 에 열쇠가 없는 칸: `!self.seen.contains_key(&(x, y))`) 쪽으로 가서 새로 봐요. 지도 크기는 `view.size()`.

목표: 160틱 안에 코어 8개를 반응로에 넣으세요.
