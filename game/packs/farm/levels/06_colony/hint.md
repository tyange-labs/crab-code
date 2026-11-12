가려는 쪽이 `view.danger(dir)` 이면 한 틱 기다리세요. `match` 의 가드로 `Some(dir) if view.danger(dir) => Action::Wait` 처럼 쓸 수 있어요.
