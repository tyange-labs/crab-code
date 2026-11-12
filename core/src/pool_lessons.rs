// 꽃게 AI 모드(조수 웅덩이)의 레슨. 순서는 pool.rs 의 SPECS 와 같아야 한다 (verify 가 id 를 대조한다).
// 본문은 마크다운으로 그려진다.

pub struct PoolLesson {
    pub id: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub start: &'static str,
    pub solution: &'static str,
    pub hint: &'static str,
}

pub const POOL_LESSONS: &[PoolLesson] = &[
    PoolLesson {
        id: "tick",
        title: "꽃게의 두뇌, tick",
        body: r#"지금까지는 꽃게에게 명령을 하나씩 내렸죠. 이제부터는 **꽃게의 두뇌**를 만듭니다.

게임은 매 **틱**(한 박자)마다 꽃게의 `tick` 을 불러 "이번엔 뭘 할래?" 하고 물어요. `tick` 은 할 일 하나를 `Action` 으로 돌려주면 됩니다.

- `Action::Move(Dir::East)` 동쪽으로 한 칸 옆걸음 (꽃게는 몸을 돌리지 않아요)
- `Action::Pick` 발밑의 먹이 집기
- `Action::Drop` 들고 있는 먹이 내려놓기. 굴 위라면 배달 완료!
- `Action::Wait` 가만히 있기

지금 상황은 `view` 로 봐요. `view.here()` 는 발밑 칸(`Tile::Food`, `Tile::Nest` …), `view.carrying()` 은 먹이를 들고 있는지예요.

`Action`, `Tile`, `View` 가 실제로 어떻게 생겼는지는 에디터 위의 **crab.rs** 탭에서 볼 수 있어요. 밑줄 친 코드를 누르거나, 내 코드의 이름 위에서 **F12**(또는 Cmd+클릭)를 누르면 그 정의로 가요. 돌아올 때는 Ctrl+-.

실행하면 아래 타임라인으로 틱을 앞뒤로 돌려 볼 수 있고, 꽃게를 누르면 그 순간의 상태(`{:?}`)가 보여요.

목표: 동쪽의 먹이를 굴까지 가져오세요."#,
        start: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 매 틱 동쪽으로 걷기만 해요. 먹이를 만나면? 들고 나면?
        Action::Move(Dir::East)
    }
}
"#,
        solution: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            if view.here() == Tile::Nest {
                Action::Drop
            } else {
                Action::Move(Dir::West)
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            Action::Move(Dir::East)
        }
    }
}
"#,
        hint: "`if view.carrying() { … } else if view.here() == Tile::Food { … } else { … }` 처럼 상황을 나눠 보세요. 굴로 돌아오면 `Action::Drop`!",
    },
    PoolLesson {
        id: "smell",
        title: "냄새를 따라 — Option 과 match",
        body: r#"이번 웅덩이는 실행할 때마다 먹이 자리가 바뀌어요. 방향을 코드에 박아 두면 안 되겠죠?

`view.food_dir()` 는 가장 가까운 먹이로 가는 첫걸음을 `Option<Dir>` 로 알려 줘요.

- `Some(dir)` 그쪽으로 한 칸 가면 먹이에 가까워져요.
- `None` 냄새가 나지 않아요.

`view.nest_dir()` 는 굴로 가는 첫걸음이고, 이미 굴 위라면 `None` 이에요. `Option` 은 `match` 로 꺼내 씁니다.

```rust
match view.nest_dir() {
    Some(dir) => Action::Move(dir),
    None => Action::Drop, // 굴에 도착!
}
```

목표: 90틱 안에 먹이 3개를 굴로 모으세요."#,
        start: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 지난 웅덩이의 코드예요. 먹이가 동쪽에만 있지는 않아요!
        if view.carrying() {
            if view.here() == Tile::Nest {
                Action::Drop
            } else {
                Action::Move(Dir::West)
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            Action::Move(Dir::East)
        }
    }
}
"#,
        solution: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            match view.nest_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Drop,
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            match view.food_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}
"#,
        hint: "들고 있을 때는 `view.nest_dir()`, 빈손일 때는 `view.food_dir()` 를 `match` 하세요. `None` 일 때 할 일도 정해야 해요.",
    },
    PoolLesson {
        id: "maze",
        title: "기억하는 꽃게 — 필드와 &mut self",
        body: r#"바위 미로 속이라 먹이 냄새가 **한 칸**밖에 퍼지지 않아요. 굴 냄새(`nest_dir`)는 여전히 잘 맡지만요.

길을 찾으려면 **오른손 법칙**을 써 봐요: 오른쪽 벽에 손을 대고 걸으면 언젠가 미로를 다 돌아요. 그러려면 "지금 어느 쪽으로 가던 중인지" 기억해야 하는데, `tick` 이 끝나면 지역 변수는 사라져요.

꽃게 구조체에 **필드**를 두면 틱이 지나도 남아요. `tick` 이 `&mut self` 를 받기 때문에 필드를 바꿀 수 있어요.

```rust
#[derive(Debug)]
struct Ferris {
    heading: Dir,
}
```

`view.look(dir)` 는 옆 칸, `.is_open()` 은 걸어갈 수 있는지예요. `Dir` 에는 `turn_right()`, `turn_left()`, `opposite()` 가 있어요.

목표: 미로 속 먹이를 찾아 굴로 가져오세요."#,
        start: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 냄새가 안 닿으면 food_dir() 은 None 이라 계속 기다리기만 해요.
        if view.carrying() {
            match view.nest_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Drop,
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            match view.food_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}
"#,
        solution: r#"#[derive(Debug)]
struct Ferris {
    heading: Dir,
}

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris { heading: Dir::East }
    }

    fn tick(&mut self, view: &View) -> Action {
        if view.carrying() {
            return match view.nest_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Drop,
            };
        }
        if view.here() == Tile::Food {
            return Action::Pick;
        }
        if let Some(dir) = view.food_dir() {
            return Action::Move(dir);
        }
        // 오른손 법칙: 오른쪽, 앞, 왼쪽, 뒤 순서로 처음 열린 쪽.
        let h = self.heading;
        for dir in [h.turn_right(), h, h.turn_left(), h.opposite()] {
            if view.look(dir).is_open() {
                self.heading = dir;
                return Action::Move(dir);
            }
        }
        Action::Wait
    }
}
"#,
        hint: "`for dir in [h.turn_right(), h, h.turn_left(), h.opposite()]` 순서로 열린 쪽을 찾고, 찾으면 `self.heading = dir;` 로 기억해 두세요.",
    },
    PoolLesson {
        id: "tide",
        title: "밀물과 썰물 — 데이터를 품은 enum",
        body: r#"굴과 먹이 사이에 **갯벌**(`Tile::Flat`)이 있어요. 밀물이 들어오면 갯벌은 물(`Tile::Water`)에 잠기고, 그 위에 있던 꽃게는 휩쓸려 굴로 떠밀려 와요. 들고 있던 먹이도 잃고요.

`view.tide()` 는 `Tide` 를 돌려줘요. 각 변형이 **데이터를 품고** 있어요.

```rust
enum Tide {
    Low { turns_left: u32 },  // 물이 빠져 있음. turns_left 틱 뒤 밀물
    High { turns_left: u32 }, // 물이 차 있음. turns_left 틱 뒤 썰물
    Calm,                     // 물때가 없는 웅덩이
}
```

`match` 로 안의 값을 꺼내고, `if` **가드**로 조건을 붙일 수 있어요.

```rust
match view.tide() {
    Tide::Low { turns_left } if turns_left >= 6 => { /* 건너도 안전 */ }
    _ => { /* 기다리기 */ }
}
```

갯벌은 다섯 칸이에요. 건너는 동안 물이 들어오지 않으려면 언제 발을 들여야 할까요?

목표: 100틱 안에 먹이 2개를 굴로 가져오세요."#,
        start: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        // 물때를 보지 않고 곧장 건너요. 실행해서 무슨 일이 생기는지 보세요!
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        match target {
            Some(dir) => Action::Move(dir),
            None if view.carrying() => Action::Drop,
            None => Action::Wait,
        }
    }
}
"#,
        solution: r#"#[derive(Debug)]
struct Ferris;

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        let Some(dir) = target else {
            return if view.carrying() { Action::Drop } else { Action::Wait };
        };
        // 갯벌에 처음 발을 들이기 전에 물때를 본다.
        let entering = view.here() != Tile::Flat && matches!(view.look(dir), Tile::Flat | Tile::Water);
        if entering {
            match view.tide() {
                Tide::Low { turns_left } if turns_left >= 6 => Action::Move(dir),
                _ => Action::Wait,
            }
        } else {
            Action::Move(dir)
        }
    }
}
"#,
        hint: "갯벌 다섯 칸을 건너 모래에 닿기까지 여섯 걸음이에요. 갯벌에 들어서기 직전에 `Tide::Low { turns_left } if turns_left >= 6` 일 때만 들어가세요.",
    },
    PoolLesson {
        id: "colony",
        title: "꽃게 군단 — 같은 코드, 다른 self",
        body: r#"이번엔 꽃게가 **세 마리**예요. 게임은 `Ferris::new(0)`, `Ferris::new(1)`, `Ferris::new(2)` 를 불러 꽃게마다 자기 `self` 를 따로 만들어요. 같은 코드가 세 번 돌지만, 상태는 꽃게마다 따로예요.

모래밭에는 **갈매기** 두 마리가 가로로 왔다 갔다 해요. 갈매기와 같은 칸이 되면 먹이를 떨어뜨리고 굴로 도망쳐요. `view.danger(dir)` 는 그쪽으로 가면 다음 틱에 갈매기에게 잡힐 때 `true` 예요.

잡힌 꽃게는 먹이를 떨어뜨리고 굴에서 **12틱** 동안 숨어 있어요. 먹이는 배달할 때마다 새로 생기고, 꽃게끼리는 서로 지나갈 수 있어요. `println!` 은 출력 창에 꽃게 번호와 틱이 붙어 나와요.

목표: 90틱 안에 먹이 10개를 모으세요."#,
        start: r#"#[derive(Debug)]
struct Ferris {
    id: usize,
}

impl Crab for Ferris {
    fn new(id: usize) -> Self {
        Ferris { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        // 갈매기를 신경 쓰지 않는 꽃게들이에요.
        if view.carrying() {
            match view.nest_dir() {
                Some(dir) => Action::Move(dir),
                None => {
                    println!("{}번 꽃게 배달!", self.id);
                    Action::Drop
                }
            }
        } else if view.here() == Tile::Food {
            Action::Pick
        } else {
            match view.food_dir() {
                Some(dir) => Action::Move(dir),
                None => Action::Wait,
            }
        }
    }
}
"#,
        solution: r#"#[derive(Debug)]
struct Ferris {
    id: usize,
}

impl Crab for Ferris {
    fn new(id: usize) -> Self {
        Ferris { id }
    }

    fn tick(&mut self, view: &View) -> Action {
        if !view.carrying() && view.here() == Tile::Food {
            return Action::Pick;
        }
        let target = if view.carrying() { view.nest_dir() } else { view.food_dir() };
        match target {
            Some(dir) if view.danger(dir) => Action::Wait,
            Some(dir) => Action::Move(dir),
            None if view.carrying() => {
                println!("{}번 꽃게 배달!", self.id);
                Action::Drop
            }
            None => Action::Wait,
        }
    }
}
"#,
        hint: "가려는 쪽이 `view.danger(dir)` 이면 한 틱 기다리세요. `match` 의 가드로 `Some(dir) if view.danger(dir) => Action::Wait` 처럼 쓸 수 있어요.",
    },
];
