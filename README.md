# Crab Code

Rust 로 게임 속 세계를 움직이며 Rust 를 배우는 게임. 코드는 **내가 쓰는 IDE** 에서 짠다.

- 게임은 화면만 맡는다. 코드는 작업 폴더(진짜 cargo 워크스페이스)에 있고, VS Code·RustRover·Zed 등
  아무 IDE 로 열면 rust-analyzer 의 자동 완성·정의로 이동·에러 표시가 그대로 된다.
- IDE 에서 파일을 **저장하면** 게임이 알아채서 `cargo build` 하고 실행해, 그 결과를 재생한다.
  타임라인으로 아무 틱이나 돌려 보고, 행위자를 눌러 그 순간 `tick()` 이 고른 행동과 `{:?}` 상태를 볼 수 있다.
- 작업 폴더는 git 저장소다. 레벨을 깨면 커밋이 남고, 최고 기록에는 `best/<팩>/<레벨>` 태그가 붙는다.
- 게임 자체는 **게임팩** 으로 갈아끼운다. 지금 들어 있는 팩:
  - **조수 웅덩이** (`tidepool`) — `Crab` 트레이트를 구현해 꽃게의 두뇌를 만든다. 처음엔 걷기만, 레벨마다 쓸 수 있는 것이
    하나씩 늘어난다 (집기·내려놓기 → Option → 필드와 &mut self → 물때 enum → 갈매기와 여러 마리). 6개 레벨.
  - **퍼즐** (`puzzle`) — 꽃게 한 마리를 `move_forward()` 같은 함수로 움직인다. 『프로그래밍 러스트』 2~17장 순서, 51개 레벨.
    처음엔 `move_forward()`·`collect()` 뿐이고, 레벨마다 쓸 수 있는 함수가 늘어난다.

## 실행

Rust 툴체인(`cargo`)과 Godot 4.7 이 필요하다 (`brew install --cask godot`). 기록을 남기려면 git 도.

```
cargo build                              # 게임 본체의 Rust 확장
godot --headless --path game --import    # 처음 한 번 (스크립트 클래스 등록)
godot --path game                        # 게임 실행
```

처음 실행하면 작업 폴더를 만드는 창이 뜬다: 위치(기본 `~/CrabCode`)와 **커밋에 쓸 이름·이메일** 을 정한다.
이름·이메일은 그 작업 폴더 저장소에만 설정되고(전역 git 설정은 그대로), 작업 폴더에서는 커밋 서명을 끈다
(혼자 보는 학습 기록이라 필요 없고, 서명 키 암호를 묻느라 게임이 멈추면 안 되므로).
그다음 게임의 **IDE 로 열기** 를 누르거나 직접 그 폴더를 IDE 로 연다.

```
~/CrabCode/
  tidepool/levels/04_tide.rs   ← 이런 파일을 고치고 저장한다
  puzzle/levels/03_for.rs
  .crab/packs/                 게임팩 소스 (IDE 에서 F12 로 정의를 보면 여기로 온다. 게임이 관리한다)
  .crab/progress.json          클리어 기록
```

터미널에서 직접 돌려 볼 수도 있다: `cd ~/CrabCode && cargo run --bin tide` (결과가 글로 나온다).

환경 변수:

| 변수 | 뜻 |
|---|---|
| `CRAB_WORKSPACE` | 작업 폴더 위치. 이걸로 실행하면 개발·테스트 모드라 게임 설정 파일을 읽지도 쓰지도 않는다 |
| `CRAB_IDE` | **IDE 로 열기** 에 쓸 명령 (기본: VS Code 의 `code`). `<명령> <폴더> -g <파일:줄:칸>` 으로 부른다 |
| `CRAB_PACK`, `CRAB_START_LEVEL` | 시작할 게임팩과 레벨 번호 (개발용) |
| `CRAB_GIT_NAME`, `CRAB_GIT_EMAIL` | 작업 폴더 커밋의 이름·이메일 (주면 처음 창을 건너뛴다) |
| `CRAB_SHOT`, `CRAB_SHOT_DELAY` | 몇 초 뒤 화면을 PNG 로 찍고 끝내기 (개발용) |

## 테스트

```
cargo run --release -p crab-host --bin verify-pack                      # 게임팩 전부: 정답은 통과, 시작 코드는 실패하는지
cargo run --release -p crab-host --bin verify-pack -- game/packs/puzzle # 팩 하나
CRAB_WORKSPACE=/tmp/crab-ws godot --path game -s tests/ide_flow_test.gd # 저장 → 빌드 → 재생 → 클리어 커밋 흐름
CRAB_TEST_SETUP=1 CRAB_WORKSPACE=/tmp/crab-ws2 godot --path game -s tests/ide_flow_test.gd  # 처음 창부터
cargo run -p crab-host --example run_level -- <작업 폴더> <팩> <레벨>    # 레벨 하나를 게임과 똑같이 돌려 보기
```

## 구조

- `host/` (Rust, `crab-host`) — 게임 본체. 어떤 게임팩이든 똑같이 다룬다: 팩 읽기(`pack.rs`), 작업 폴더와 git
  (`workspace.rs`, `git.rs`), 저장 감지(`watch.rs`), cargo 로 빌드·실행하고 출력 모으기(`run.rs`), 팩 검증(`verify.rs`).
- `godot-ext/` (Rust) — `crab-host` 를 Godot 에 `CrabCore` 클래스로 연결한다.
- `game/` (Godot) — 화면. `host.tscn` + `scripts/host.gd` 가 레슨·작업 폴더·타임라인·출력 창을 맡고,
  세계는 게임팩의 화면이 그린다.
- `game/packs/<팩>/` — 게임팩.

## 게임팩 만들기

```
game/packs/<id>/
  pack.json        {"id", "name", "crate", "tagline", "agent": "꽃게", "tick": "틱", "colors": [...], "chapters"?: [...]}
  crate/           플레이어 코드가 의존하는 Rust 크레이트 (std 만 — 인터넷 없이 빌드되도록)
    src/lib.rs     플레이어용 API 와 세계 규칙
    src/bin/preview.rs   `preview <레벨> <시드>`: 레벨의 처음 모습만 출력 (코드가 컴파일되지 않을 때 보여 준다)
  levels/NN_<레벨>/
    lesson.md      첫 줄 "# 제목", 나머지는 마크다운 본문 ("목표:" 로 시작하는 문단은 목표로 쓰인다)
    start.rs       처음 코드 (작업 폴더의 <id>/levels/NN_<레벨>.rs 가 된다. 실행 파일 이름 = <레벨>)
    solution.rs    정답
    hint.md        힌트
    start_error.txt  (선택) 처음 코드가 일부러 내는 컴파일 에러 코드 (예: E0384). verify-pack 이 바로 그 에러인지 확인한다
  view/view.gd     세계를 그리는 Godot 화면
```

**레벨마다 API 를 하나씩 연다.** 두 팩 모두 같은 방식이다. 예를 들어 조수 웅덩이 팩은 레벨마다 모듈(`level1.rs` … `level6.rs`)을 두고, 레벨 파일은
`use tidepool::level3::*;` 처럼 자기 레벨 모듈만 가져온다. 모듈에는 그 레벨에서 쓸 수 있는 `Crab`·`View`·`Action`·`Dir`·`Tile`
만 짧게 정의돼 있고 새로 생긴 것에는 `(새로!)` 가 붙는다. 그래서 IDE 의 F12 가 그 짧은 파일로 가고, 아직 안 배운 것을 쓰면
rustc 가 "그런 변형/메서드는 없다" 고 알려 준다. 세계 시뮬레이션은 숨은 `engine` 모듈에 있고, `engine/bridge.rs` 가
레벨 타입과 엔진 타입을 서로 바꾼다.

플레이어 프로그램은 표준 출력으로 한 줄에 하나씩 `"\u{1}crab " + JSON` 을 보낸다 (머리글 없는 줄은 플레이어의 `println!`):

| `t` | 내용 |
|---|---|
| `start` | `map`: 바뀌지 않는 지도 정보 |
| `frame` | `tick` 과 그 틱의 모습 (팩 마음대로), `events: [{agent, text}]` |
| `think` / `agent` | 행위자 `agent` 의 두뇌가 돈다 / 고른 `action` 과 `{:?}` `state` (살펴보기 창) |
| `end` | `won`, `message`, `stats: {ticks}`. 없으면 마지막 `frame` 의 `outcome`·`stats` 를 쓴다 |

게임은 `CRAB_HOST=1`, `CRAB_LEVEL=<레벨>`, `CRAB_SEED=<시드>` 를 넣어 실행한다.

`view/view.gd` 는 Control 이고 이렇게 약속한다: `setup(map, frames)`, `time` (틱 단위 실수 — 틱 사이를 보간),
`selected`, `last_tick()`, `frame_at(t)`, `play_effects(t)`, `celebrate()`, `agent_count()`,
`describe_agent(i, t)`, `signal agent_selected(i)`. 두 팩의 `view.gd` 가 예시다.

## 함께 들어 있는 것

- [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono), [Pretendard](https://github.com/orioncactus/pretendard) — SIL Open Font License 1.1 (`game/fonts/`)
