`parse_core` 만 채우면 돼요: `strip_prefix("C:")?` 로 앞을 떼고, `split_once(',')?` 로 둘로 나누고, 각각 `parse().ok()?` 로 숫자로 바꿔 `Some((x, y))`. 나머지(`closest`, 걸어가기)는 이미 짜여 있어요. 코어를 들고 있으면 `reactor_dir()` 로 가요.
