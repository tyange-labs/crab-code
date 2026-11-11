# Crab Code

꽃게 페리스를 Rust 코드로 움직이며 Rust 를 배우는 퍼즐 게임.

에디터에 쓴 코드를 진짜 `rustc` 로 컴파일해서 실행하기 때문에, 컴파일 에러도 실제 Rust 컴파일러 메시지 그대로 나온다.
레벨 51개는 『프로그래밍 러스트』(개정판)의 목차 순서(2장~17장)를 따른다.

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
godot --path game -s tests/markdown_shot.gd -- markdown.png   # 마크다운 미리 보기를 그림으로
```

## 레벨 검증

모든 레벨의 정답 코드가 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.

```
cargo run --release --bin verify
```

## 구조

- `core/src/game.rs` — 화면이 부르는 기능(레슨 목록, 실행, 검사)과 레벨 검증.
- `core/src/sim.rs` — 월드 시뮬레이션과 레벨 지도. 게임과 플레이어 프로그램이 함께 쓴다.
- `core/src/runner.rs` — 플레이어 코드를 `rustc` 로 컴파일하고 실행한다.
- `core/src/lessons.rs` — 레슨 본문, 시작 코드, 정답.
- `core/runtime/crab.rs` — 플레이어 코드에서 부르는 함수들 (`move_forward`, `collect` 등).
- `godot-ext/src/lib.rs` — `CrabCore`: 컴파일은 백그라운드 스레드에서 돌리고 화면이 결과를 받아 간다.
- `game/main.tscn` — 화면 배치. `game/scripts/` 에 스크립트:
  `main.gd`(전체 흐름), `code_editor.gd`(에디터·단축키), `find_bar.gd`(찾기·바꾸기), `markdown_view.gd`(레슨 본문), `world_view.gd`(맵·애니메이션), `sfx.gd`(효과음), `crab_art.gd`(꽃게·보석 그림).

## 함께 들어 있는 것

- [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono), [Pretendard](https://github.com/orioncactus/pretendard) — SIL Open Font License 1.1 (`game/fonts/`)
