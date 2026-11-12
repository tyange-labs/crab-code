# match — 빠짐없이 나누기

`match` 는 값에 따라 갈래를 나눠요. 중요한 규칙: 모든 경우를 빠짐없이 다뤄야 합니다. 하나라도 빼먹으면 컴파일러가 잡아 줘요!

바닥의 표지판은 네 가지 중 하나예요:

`enum Sign { Straight, Left, Right, Finish }`

• `read_sign()` 발밑의 표지판 읽기. 없으면 `Sign::Straight`

`loop` 는 `break` 를 만날 때까지 계속 도는 반복문입니다.

목표: 표지판을 따라 깃발까지 가세요. 빠진 갈래를 채워 넣으세요.
