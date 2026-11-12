# 스트럭트의 필드

스트럭트(struct)는 이름 붙은 값들을 하나로 묶은 타입이에요. `inspect` 는 이런 스트럭트를 돌려줍니다:

`struct GemInfo { color: Color, weight: u32, shiny: bool }`

각 값을 필드라고 하고, `info.weight` 처럼 점으로 꺼내요.

• `inspect(&gem)` 보석을 살펴보고 `GemInfo` 를 돌려줘요

목표: 비밀번호는 보석 3개의 무게 합이에요. 무게는 실행할 때마다 달라집니다.
