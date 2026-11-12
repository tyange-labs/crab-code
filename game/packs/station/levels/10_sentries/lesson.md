# 오작동 드론 — impl 로 코드 나누기

슬롯 사이를 오작동한 **경비 드론**이 오가요. 닿으면 들고 있던 코어를 잃고 반응로로 쫓겨나 12틱 동안 숨어 있어야 해요. 이번에도 **새로 생긴 API 는 없어요.** 6레벨에서 배운 `view.danger(dir)` 를 쓰는 거예요.

그런데 걸을 때마다 "위험하면 기다리기"를 쓰다 보면 같은 코드가 여러 군데에 생겨요. 이럴 땐 **`impl` 블록에 함수로 빼요.**

```rust
impl Echo {
    /// 그쪽으로 한 걸음. 위험하면 기다려요.
    fn go(view: &View, dir: Option<Dir>) -> Action {
        match dir {
            Some(d) if view.danger(d) => Action::Wait,
            Some(d) => Action::Move(d),
            None => Action::Wait,
        }
    }
}
```

`self` 를 받지 않는 함수라서 `Self::go(view, dir)` 로 불러요 (연관 함수). `self` 를 받으면 `self.go(view)` 처럼 부르는 **메서드**가 돼요.

`Bot` 트레이트의 `impl Bot for Echo` 블록에는 트레이트가 정한 함수만 넣어야 해요. 내가 만든 도우미 함수는 **따로 `impl Echo { … }`** 블록에 둬요.

목표: 220틱 안에 코어 9개를 반응로에 넣으세요.
