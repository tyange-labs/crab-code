들고 있으면 `reactor_dir()` 로, 빈손이면 `view.core_dir().or(view.card_dir()).or(view.door_dir())` 로 걸어갈 방향을 골라요. 걷기 직전에 `view.look(dir)` 이 `Tile::Door(hue)` 이고 `!view.keys().contains(&hue)` 이면 `Action::Wait`.
