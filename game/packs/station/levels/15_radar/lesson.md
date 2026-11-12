# 레이더 해독 — &str 과 라이프타임

이번 구역에서는 코어가 어디 있는지 **글**로 알려 줘요. `view.core_dir()` 는 소용없고, 레이더가 이런 글을 보내요.

```text
C:4,2 C:7,5 C:1,6
```

코어마다 `C:x,y` 한 조각이고 공백으로 이어져 있어요. 그런데 가끔 `C:?,?` 나 `###` 같은 **알아볼 수 없는 조각**이 섞여요. 잡음이에요.

**새로 생긴 것**

- `view.radar()` — 레이더가 보내는 글, `&str`.

`&str` 는 글자들을 **빌려서 보는** 타입이에요 (문자열 조각). `String` 처럼 글자를 소유하지 않고, 어딘가에 있는 글을 가리켜요. 그래서 `split`, `strip_prefix`, `split_once` 로 자르면 새 글자를 만드는 게 아니라 **원래 글의 일부를 가리키는** `&str` 가 나와요.

```rust
let rest = "C:4,2".strip_prefix("C:")?;       // Option<&str>: "4,2"
let (x, y) = rest.split_once(',')?;           // ("4", "2")
let x: i32 = x.parse().ok()?;                 // Result 를 Option 으로 바꿔 ? 로 풀어요
```

`?` 는 `None` 이면 그 자리에서 함수를 `None` 으로 끝내요. 조각 하나를 해독하는 함수는 이렇게 쓸 수 있어요.

```rust
fn parse_core(token: &str) -> Option<(i32, i32)> {
    let rest = token.strip_prefix("C:")?;
    let (x, y) = rest.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}
```

`C:?,?` 는 `parse()` 가 실패해서 `None`, `###` 는 `strip_prefix` 가 실패해서 `None` 이에요. 잡음이 저절로 걸러져요.

**라이프타임.** 레이더 글에서 조각을 하나 골라 `&str` 로 돌려주는 함수는, 그 조각이 **어느 글에서 빌려 온 것인지** 컴파일러에 알려야 해요. `'a` 가 그 이름표예요.

```rust
fn closest<'a>(radar: &'a str, at: (i32, i32)) -> Option<&'a str> {
    radar
        .split(' ')
        .filter(|t| parse_core(t).is_some())
        .min_by_key(|t| { /* 거리 */ })
}
```

"돌려주는 `&str` 는 `radar` 와 같은 수명 `'a` 를 갖는다" 는 뜻이에요. 그래서 `radar` 가 사라진 뒤에 그 조각을 쓰려 하면 컴파일러가 막아요. (인자가 하나뿐이면 컴파일러가 알아서 이름표를 붙여 줘서 쓰지 않아도 돼요. 이걸 **라이프타임 생략**이라고 해요.)

목표: 160틱 안에 코어 6개를 반응로에 넣으세요.
