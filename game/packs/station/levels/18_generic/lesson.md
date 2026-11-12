# 일반 부품 — 제네릭과 trait 경계

이번 구역에는 갈 곳이 **두 종류**예요. 다 된 **코어**와, 손이 필요한 **슬롯**이에요. 둘 다 "위치가 있는 것"이고, 둘 다 "가장 가까운 것"을 찾고 싶어요. 그런데 `Vec<CoreSpot>` 용 함수와 `Vec<SlotSpot>` 용 함수를 따로 두 번 짜면 똑같은 코드가 두 개가 돼요.

**새로 생긴 것은 없어요.** 대신 **제네릭**(`<T>`)으로 한 번만 짜요.

먼저 "위치가 있다"는 **약속**을 `trait` 로 만들어요.

```rust
trait Located {
    fn at(&self) -> (i32, i32);
}

#[derive(Debug, Clone, Copy)]
struct CoreSpot { x: i32, y: i32 }

#[derive(Debug, Clone, Copy)]
struct SlotSpot { x: i32, y: i32 }

impl Located for CoreSpot {
    fn at(&self) -> (i32, i32) { (self.x, self.y) }
}
impl Located for SlotSpot {
    fn at(&self) -> (i32, i32) { (self.x, self.y) }
}
```

이제 **어떤 타입이든 `Located` 를 구현했다면** 쓸 수 있는 함수를 만들어요. `T: Located` 가 **trait 경계**예요. "`T` 는 `Located` 를 구현한 타입이어야 한다"는 뜻이에요.

```rust
fn nearest<T: Located>(items: &[T], from: (i32, i32)) -> Option<&T> {
    items.iter().min_by_key(|item| {
        let (x, y) = item.at();
        (x - from.0).abs() + (y - from.1).abs()
    })
}
```

`item.at()` 을 부를 수 있는 건 경계 덕분이에요. 경계가 없으면 `T` 가 뭔지 몰라서 컴파일러가 막아요. 같은 함수를 `nearest(&cores, pos)` 와 `nearest(&slots, pos)` 처럼 **두 타입 모두**에 써요. (컴파일할 때 타입마다 함수가 따로 만들어져서, 일반 코드인데도 실행이 느려지지 않아요. 이걸 **단형화**라고 해요.)

경계가 길어지면 `where` 로 따로 적어도 돼요.

```rust
fn nearest<T>(items: &[T], from: (i32, i32)) -> Option<&T>
where
    T: Located,
{ /* … */ }
```

로봇이 할 일: 다 된 코어(`view.cores()`)가 있으면 그중 가장 가까운 것으로, 없으면 지도를 훑어 모은 `Slot`/`Module` 칸 중 가장 가까운 것으로 가요. 슬롯에 서면 `Install`, 모듈이면 `Charge`, 코어를 집고 반응로에 넣는 건 지금까지처럼이에요.

목표: 200틱 안에 코어 8개를 반응로에 넣으세요.
