# 설치와 충전 — 튜플 match 와 Option 이어 붙이기

이번 구역에는 바닥에 놓인 코어가 없어요. 대신 **슬롯** 세 개가 있어요. 모듈을 설치하고, 충전하고, 몇 틱 기다리면 코어가 생겨요. 그 코어를 집어서 반응로에 넣으면 돼요.

**새로 생긴 것**

- `Action::Install` — 발밑 슬롯에 모듈을 설치해요.
- `Action::Charge` — 발밑 슬롯의 모듈을 충전해요. 충전하면 부팅이 시작돼요 (4틱 뒤 코어).
- `Tile::Slot` · `Tile::Module` · `Tile::Booting` — 빈 슬롯 · 모듈만 설치된 슬롯 · 부팅 중인 슬롯.
- `view.slot_dir()` — 손이 필요한 슬롯(빈 슬롯이나 충전 안 한 모듈)으로 가는 첫걸음. 없으면 `None`.

한 슬롯은 이렇게 돌아가요: `Slot` →(설치)→ `Module` →(충전)→ `Booting` →(4틱)→ `Core` →(집기)→ `Slot`.

발밑 칸과 들고 있는지를 **한꺼번에** 보려면 값 두 개를 **튜플**로 묶어 `match` 해요.

```rust
match (view.here(), view.carrying()) {
    (Tile::Slot, _) => Action::Install,
    (Tile::Core, false) => Action::Grab,
    (Tile::Reactor, true) => Action::Insert,
    _ => Action::Wait,
}
```

`_` 는 "아무 값이나"예요. 그리고 `Option` 두 개를 이을 땐 `a.or(b)` 가 좋아요. `a` 가 `Some` 이면 그걸, `None` 이면 `b` 를 돌려줘요.

```rust
let target = view.slot_dir().or(view.core_dir());
```

부팅을 기다리는 동안 놀지 말고 다음 슬롯으로 가야 시간 안에 끝나요.

목표: 70틱 안에 코어 3개를 반응로에 넣으세요.
