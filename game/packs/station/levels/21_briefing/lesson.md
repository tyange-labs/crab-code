# 작전 지시문 — FromStr, 에러 타입, From

이번에는 본부가 **글**로 작전 지시문을 보내와요. `view.briefing()` 이 돌려주는 `&str` 예요.

```text
E3 S2 G W3 N2 I Q7 E8 S4 G W8 N4 E? I
```

공백으로 나뉜 **명령 조각**들이에요.

- `N` `E` `S` `W` + 숫자 — 그쪽으로 그만큼 걷기 (`E3` 은 동쪽 세 걸음, 숫자가 없으면 한 걸음).
- `G` — 코어 집기 (`Grab`).
- `I` — 반응로에 넣기 (`Insert`).

그런데 **잡음**이 섞여 있어요. `Q7` 은 모르는 코드이고, `E?` 는 숫자가 아니에요. 잡음을 **조용히 삼키지 말고**, 무엇이 왜 틀렸는지 알려 주면서 건너뛰고 싶어요.

**새로 생긴 것**

- `view.briefing()` — 작전 지시문, `&str`.

**1. 명령 타입과 에러 타입을 만들어요.**

```rust
#[derive(Debug)]
enum Command {
    Go(Dir, u32),
    Grab,
    Insert,
}

#[derive(Debug)]
enum ParseError {
    Empty,
    UnknownCode(String),
    BadCount(std::num::ParseIntError), // 숫자 해석이 실패한 까닭을 품어요
}
```

**2. `From` 으로 에러를 바꿔요.** 숫자 해석(`parse::<u32>()`)은 `ParseIntError` 를 내는데, `?` 로 그냥 넘기면 **내 에러 타입으로 저절로 바뀌게** 하려면 `From` 을 구현해요.

```rust
impl From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> ParseError {
        ParseError::BadCount(e)
    }
}
```

**3. `FromStr` 을 구현하면 `"E3".parse::<Command>()` 가 돼요.**

```rust
use std::str::FromStr;

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(token: &str) -> Result<Command, ParseError> {
        let mut chars = token.chars();
        let code = chars.next().ok_or(ParseError::Empty)?;
        let rest = chars.as_str(); // 첫 글자 뒤의 나머지 글
        match code {
            'N' | 'E' | 'S' | 'W' => {
                let steps = if rest.is_empty() { 1 } else { rest.parse::<u32>()? }; // ? 가 From 으로 바꿔 줘요
                let dir = match code { 'N' => Dir::North, 'E' => Dir::East, 'S' => Dir::South, _ => Dir::West };
                Ok(Command::Go(dir, steps))
            }
            'G' => Ok(Command::Grab),
            'I' => Ok(Command::Insert),
            _ => Err(ParseError::UnknownCode(token.to_string())),
        }
    }
}
```

`ok_or(에러)` 는 `Option` 을 `Result` 로 바꿔요. `None` 이면 그 에러를 `Err` 로 돌려줘요.

**4. 처음 한 번 읽어서 실행 대기열을 만들어요.** 틀린 조각은 `println!` 으로 알려 주고 건너뛰어요. 출력창에 그대로 떠요.

```rust
for token in view.briefing().split(' ') {
    match token.parse::<Command>() {
        Ok(command) => { /* 대기열에 넣기 */ }
        Err(error) => println!("건너뜀: {token} ({error:?})"),
    }
}
```

대기열(`VecDeque<Action>`)에서 틱마다 하나씩 꺼내 실행해요. `Go(Dir::East, 3)` 은 `Move(East)` 세 번으로 풀어서 넣어요.

목표: 60틱 안에 코어 2개를 반응로에 넣으세요.
