# 일 목록 — trait 과 dyn

농부가 할 일이 늘어나면 `tick` 안의 `match` 가 점점 길어져요. 이번에는 **일 하나하나를 타입으로** 만들고, 농부는 **일 목록**을 훑으면서 "지금 할 수 있는 일"을 골라요. 새로 생긴 API 는 없고, Rust 의 **`trait`** 를 직접 만들어 봐요.

```rust
trait Chore: std::fmt::Debug {
    /// 지금 할 수 있는 일이면 `Some(행동)`, 아니면 `None`.
    fn try_do(&self, view: &View) -> Option<Action>;
}

#[derive(Debug)]
struct Plant;

impl Chore for Plant {
    fn try_do(&self, view: &View) -> Option<Action> {
        (view.here() == Tile::Plot).then_some(Action::Plant)
    }
}
```

`bool.then_some(값)` 은 `true` 면 `Some(값)`, `false` 면 `None` 이에요.

일은 종류마다 **다른 타입**이라 한 `Vec` 에 그냥 못 담아요. 이럴 땐 `Box<dyn Chore>` — "`Chore` 를 구현한 아무 타입이나 상자에 담아 가리킨다" — 를 써요.

```rust
#[derive(Debug)]
struct Pip {
    chores: Vec<Box<dyn Chore>>,
}

fn new(_id: usize) -> Self {
    Pip { chores: vec![Box::new(Store), Box::new(Harvest), Box::new(Plant), Box::new(Water), Box::new(Walk)] }
}

fn tick(&mut self, view: &View) -> Action {
    self.chores.iter().find_map(|c| c.try_do(view)).unwrap_or(Action::Wait)
}
```

`find_map` 은 목록을 앞에서부터 훑다가 **처음 `Some` 을 돌려주는 일**의 값을 꺼내요. 그러니 **목록의 순서가 곧 우선순위**예요.

마지막 일 `Walk` 는 걸어갈 방향을 골라요: 열매를 들고 있으면 `barn_dir()`, 아니면 `crop_dir().or(plot_dir())`. 고블린이 위험하면(`view.danger(d)`) 기다리고요.

목표: 220틱 안에 열매 9개를 창고에 넣으세요.
