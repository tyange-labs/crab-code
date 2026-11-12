# 금고 — 데이터를 품은 enum 심화

반응로 옆 구역은 **잠긴 문** 너머에 있어요. 문마다 색이 있고, **같은 색 키카드**가 있어야 열려요. 카드는 바닥에 놓여 있고, **밟으면 저절로 주워요.**

**새로 생긴 것**

- `Tile::Door(Hue)` — 잠긴 문. 어떤 색인지가 **값 안에** 들어 있어요.
- `Tile::Card(Hue)` — 바닥에 놓인 키카드.
- `Hue::Red`, `Hue::Blue` — 색.
- `view.keys()` — 가진 키카드 목록, `Vec<Hue>`.
- `view.card_dir()` — 가장 가까운 키카드로 가는 첫걸음.
- `view.door_dir()` — **가진 키카드로 열 수 있는** 가장 가까운 문으로 가는 첫걸음.

`Door(Hue)` 처럼 변형이 값을 품은 `enum` 은 5레벨에서 봤죠. 안의 값을 **패턴으로 꺼내** 쓸 수 있어요.

```rust
match view.look(dir) {
    Tile::Door(hue) => { /* hue 가 이 문의 색 */ }
    Tile::Card(Hue::Red) => { /* 붉은 카드만 */ }
    _ => {}
}
```

그리고 `if` **가드**로 "내가 그 색 카드를 갖고 있을 때만" 같은 조건을 붙일 수 있어요.

```rust
match view.look(dir) {
    Tile::Door(hue) if !view.keys().contains(&hue) => Action::Wait, // 카드가 없으면 부딪히지 말고 기다려요
    _ => Action::Move(dir),
}
```

`Vec::contains(&값)` 은 목록에 그 값이 있는지 알려줘요. (`Hue` 는 `PartialEq` 를 구현하고 있어서 `==` 로 비교돼요.)

일을 **순서대로** 하면 돼요.

1. 코어에 갈 수 있으면 코어로 (`core_dir()`).
2. 안 되면 키카드로 (`card_dir()`).
3. 그래도 안 되면 가진 카드로 열 수 있는 문으로 (`door_dir()`). 문 쪽으로 걸어가면 열려요.

`Option` 을 이을 때는 `a.or(b).or(c)` 가 편해요.

목표: 160틱 안에 코어 3개를 반응로에 넣으세요.
