`go` 함수 안에서 `match dir { … }` 을 하세요. `Some(d) if view.danger(d)` 면 `Action::Wait`, 그냥 `Some(d)` 면 `Action::Move(d)`, `None` 이면 `Action::Wait` 이에요.
