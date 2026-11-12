# 정거장 장부 — Rc 와 RefCell

13레벨에서 본 칸을 `HashMap` 에 적어 두었죠. 이번엔 로봇의 두뇌를 **두 부품**으로 나눠요.

- **스캐너** — 주변을 훑어서 장부에 **적기만** 해요.
- **계획기** — 장부를 **읽어서** 갈 곳을 골라요.

두 부품이 **같은 장부**를 봐야 해요. 그런데 Rust 에서는 값의 주인이 한 명이에요. 장부를 스캐너에게 주면 계획기는 못 가져요. 이럴 땐 **`Rc`** 로 주인을 여럿으로 만들어요.

```rust
use std::cell::RefCell;
use std::rc::Rc;

let ledger = Rc::new(RefCell::new(Ledger::default()));
let scanner = Scanner { ledger: Rc::clone(&ledger) }; // 주인 하나 더 (복사가 아니라 같은 장부를 가리켜요)
let planner = Planner { ledger };                     // 마지막 주인
```

`Rc<T>` 는 "참조를 세는 상자"예요. `Rc::clone` 은 장부를 복사하지 않고 **주인 수만 1 늘려요**. 주인이 다 사라지면 장부도 지워져요. (`Rc::strong_count(&ledger)` 로 주인 수를 볼 수 있어요.)

그런데 `Rc` 안의 값은 **읽기만** 돼요. 장부에 적으려면 **`RefCell`** 이 필요해요. `RefCell<T>` 는 "**실행 중에** 빌림 규칙을 확인하는 상자"예요.

```rust
// 적기
self.ledger.borrow_mut().seen.insert((x, y), tile);

// 읽기
let ledger = self.ledger.borrow();
let work = ledger.seen.iter().filter(|(_, t)| matches!(**t, Tile::Slot | Tile::Module | Tile::Core));
```

`borrow()` 는 읽기, `borrow_mut()` 는 쓰기예요. **쓰는 동안에는 아무도 읽을 수 없어요.** 규칙을 어기면(예: 읽는 중에 `borrow_mut()`) 컴파일은 되지만 **실행 중에 패닉**이 나요. 그래서 `borrow_mut()` 를 오래 쥐지 말고, 쓰고 곧바로 놓아 주세요.

`Debug` 를 붙여 두면 게임 창에서 로봇을 눌렀을 때 `ledger` 의 내용이 나와서, 스캐너가 적을 때마다 계획기가 같은 내용을 보는 걸 확인할 수 있어요.

이번 구역도 센서가 꺼져 있어서(`view.sight()` 걸음만 보여요) 본 걸 기억해야 해요.

목표: 160틱 안에 코어 8개를 반응로에 넣으세요.
