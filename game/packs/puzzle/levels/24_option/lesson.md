# Option — 있을 수도, 없을 수도

Rust 에는 null 이 없어요. 대신 값이 없을 수도 있으면 `Option` 을 씁니다. `Some(값)` 아니면 `None` 이죠.

`read_number()` 는 `Option<u32>` 를 돌려줘요. 숫자 타일 위면 `Some(n)`, 아니면 `None`. 상자를 열어 보기 전에는 안의 값을 쓸 수 없습니다:

`if let Some(n) = read_number() { ... }`

`match` 로 `Some(n)` 과 `None` 을 나눠도 돼요.

목표: 바닥 숫자의 합이 문의 비밀번호예요.
