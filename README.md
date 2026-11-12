# Crab Code

꽃게 페리스의 두뇌를 Rust 로 짜며 Rust 를 배우는 게임.

## 두 가지 모드

- **꽃게 AI · 조수 웅덩이** (기본) — 꽃게를 조종하지 않는다. `Crab` 트레이트를 구현해 꽃게의 두뇌를 만들면,
  게임이 매 틱 `tick(&mut self, view: &View) -> Action` 을 불러 꽃게들이 스스로 움직인다.
  밀물이 갯벌을 덮고, 갈매기가 순찰하고, 여러 마리가 같은 코드로 따로 움직인다.
  실행이 끝나면 타임라인으로 아무 틱이나 돌려 보고, 꽃게를 눌러 그 순간의 `{:?}` 상태와 행동을 볼 수 있다.
  웅덩이마다 걸린 틱 수와 코드 줄 수로 최고 기록이 남는다.
  에디터의 읽기 전용 `crab.rs` 탭은 내 코드와 함께 컴파일되는 실제 소스(`Crab`, `View`, `Action`, `Dir`, `Tile`, `Tide`)를
  보여 준다. 소스에서 `//>> 참고` ~ `//<< 참고` 로 표시한 구간을 그대로 모은 것이라 실제 코드와 어긋나지 않는다.
  F12·Cmd+클릭으로 정의로 가고 Ctrl+-(Windows·Linux: Alt+←)로 돌아온다. 레슨 본문의 코드 조각도 누르면 정의로 간다. (지금은 프로토타입 5개 웅덩이)
- **퍼즐** — 꽃게 한 마리를 명령(`move_forward()` 등)으로 움직이는 51개 레벨. 『프로그래밍 러스트』 2~17장 순서.

```rust
#[derive(Debug)]
struct Ferris {
    heading: Dir,
}

impl Crab for Ferris {
    fn new(_id: usize) -> Self {
        Ferris { heading: Dir::East }
    }

    fn tick(&mut self, view: &View) -> Action {
        match view.food_dir() {
            Some(dir) => Action::Move(dir),
            None => Action::Wait,
        }
    }
}
```

에디터에 쓴 코드를 진짜 `rustc` 로 컴파일해서 실행하기 때문에, 컴파일 에러도 실제 Rust 컴파일러 메시지 그대로 나온다.

## 기술 스택

[Godot 4](https://godotengine.org) 게임이고, 게임 로직은 Rust 로 짜서 [godot-rust](https://godot-rust.github.io) 로 붙였다.

- **core/ (Rust)** — 레벨, 레슨, 월드 시뮬레이션. 플레이어 코드를 `rustc` 로 컴파일·실행한다.
- **godot-ext/ (Rust)** — core 를 Godot 에 `CrabCore` 클래스로 연결하는 확장.
- **game/ (Godot)** — 화면. 에디터는 Godot 의 CodeEdit, 맵과 연출·효과음은 GDScript 로 그린다.

## 실행

Rust 툴체인(`rustc`, `cargo`)과 Godot 4.7 이 필요하다 (`brew install --cask godot`).

```
cargo build                 # Rust 확장을 먼저 빌드한다
godot --path game           # 게임 실행
godot -e --path game        # Godot 에디터로 열기
```

- 실행: F5
- 에디터는 VS Code 기본 단축키를 따른다 (macOS 는 Cmd, 다른 OS 는 Ctrl). 전체 목록은 `game/scripts/code_editor.gd` 맨 위에 있다.
  - 편집: `Cmd+/` 주석, `Shift+Alt+A` 블록 주석, `Alt+↑/↓` 줄 옮기기, `Shift+Alt+↑/↓` 줄 복사, `Cmd+Shift+K` 줄 지우기, `Cmd+Enter` 아래에 새 줄, `Cmd+]/[` 들여쓰기
  - 선택: `Cmd+D` 다음 같은 단어, `Cmd+Shift+L` 모두, `Alt+Cmd+↑/↓` 캐럿 추가, `Cmd+L` 줄, `Shift+Alt+→/←` 넓히기/좁히기
  - 찾기: `Cmd+F`, `Alt+Cmd+F`(Windows `Ctrl+H`) 바꾸기, `F3`/`Cmd+G` 다음, `Ctrl+G` 줄 이동
  - 그 밖에: `F8` 다음 문제, `Ctrl+Space` 자동 완성, `Cmd+Shift+Space` 매개변수 힌트, `Shift+Alt+F` rustfmt 정리, `Alt+Cmd+[`/`]` 접기, `Alt+Z` 줄 바꿈, `Cmd+=`/`-` 글자 크기
- 입력을 멈추면 컴파일 에러가 에디터에 물결 밑줄로 표시되고, 마우스를 올리면 메시지가 나온다.
- 레슨 본문은 마크다운으로 그린다 (`game/scripts/markdown_view.gd`): 인라인 코드, 굵게·기울임, 링크, 제목, 목록, 인용, 코드 블록, "목표:" 콜아웃. 인라인 코드를 누르면 복사된다.
- 코드와 진행 상황은 Godot 의 사용자 폴더(`user://save.json`)에 저장된다. 예전 버전의 `~/.crab-code/` 저장은 처음 실행할 때 가져온다.

개발용 환경 변수: `CRAB_START=레벨번호`, `CRAB_DEMO=1`(정답 자동 실행) 또는 `start`(시작 코드 실행), `CRAB_SHOT=파일.png`(몇 초 뒤 화면을 찍고 종료).

## 테스트

```
godot --headless --path game -s tests/keymap_test.gd          # 에디터 단축키 (실제 에디터에 키 입력)
godot --headless --path game -s tests/mode_switch_test.gd     # 두 모드 사이 전환
godot --path game -s tests/definition_test.gd                 # crab.rs 탭과 정의로 가기
godot --path game -s tests/markdown_shot.gd -- markdown.png   # 마크다운 미리 보기를 그림으로
```

## 레벨 검증

모든 레벨의 정답 코드가 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.

```
cargo run --release --bin verify            # 두 모드 모두
cargo run --release --bin verify -- pool    # 꽃게 AI 모드만 (정답의 틱 수도 보여 준다)
cargo run --release --example pool_debug -- 4 start 1   # 웅덩이 하나를 틱마다 글자 지도로 찍기
```

## 구조

- `core/src/game.rs` — 화면이 부르는 기능(레슨 목록, 실행, 검사)과 레벨 검증.
- `core/src/sim.rs` — 월드 시뮬레이션과 레벨 지도. 게임과 플레이어 프로그램이 함께 쓴다.
- `core/src/runner.rs` — 플레이어 코드를 `rustc` 로 컴파일하고 실행한다.
- `core/src/lessons.rs` — 레슨 본문, 시작 코드, 정답.
- `core/src/pool.rs` — 꽃게 AI 모드의 시뮬레이션(물때·갈매기·먹이)과 웅덩이 지도. 게임과 플레이어 프로그램이 함께 쓴다.
- `core/src/pool_game.rs`, `core/src/pool_lessons.rs` — 꽃게 AI 모드의 실행·재생·검증과 레슨.
- `core/runtime/pool_crab.rs` — `Crab` 트레이트와, 매 틱 꽃게마다 `tick` 을 부르는 실행 루프.
- `core/runtime/crab.rs` — 플레이어 코드에서 부르는 함수들 (`move_forward`, `collect` 등).
- `godot-ext/src/lib.rs` — `CrabCore`: 컴파일은 백그라운드 스레드에서 돌리고 화면이 결과를 받아 간다.
- `game/main.tscn` — 화면 배치. `game/scripts/` 에 스크립트:
  `pool_main.gd`·`pool_view.gd`(꽃게 AI 화면·웅덩이 그림), `main.gd`(퍼즐 전체 흐름), `code_editor.gd`(에디터·단축키), `find_bar.gd`(찾기·바꾸기), `markdown_view.gd`(레슨 본문), `world_view.gd`(맵·애니메이션), `sfx.gd`(효과음), `crab_art.gd`(꽃게·보석 그림).

## 함께 들어 있는 것

- [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono), [Pretendard](https://github.com/orioncactus/pretendard) — SIL Open Font License 1.1 (`game/fonts/`)
