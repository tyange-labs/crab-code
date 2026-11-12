# Sprout Code

Rust 로 게임 속 세계를 움직이며 Rust 를 배우는 게임. 지금 들어 있는 주 게임팩은 **해바라기 농장**: 코드로 농부를 움직여 씨를 뿌리고 열매를 거둔다. 코드는 **내가 쓰는 IDE** 에서 짠다.

- 게임은 화면만 맡는다. 코드는 작업 폴더(진짜 cargo 워크스페이스)에 있고, VS Code·RustRover·Zed 등
  아무 IDE 로 열면 rust-analyzer 의 자동 완성·정의로 이동·에러 표시가 그대로 된다.
- IDE 에서 파일을 **저장하면** 게임이 알아채서 `cargo build` 하고 실행해, 그 결과를 재생한다.
  타임라인으로 아무 틱이나 돌려 보고, 행위자를 눌러 그 순간 `tick()` 이 고른 행동과 `{:?}` 상태를 볼 수 있다.
- 작업 폴더는 git 저장소다. 레벨을 깨면 커밋이 남고, 최고 기록에는 `best/<팩>/<레벨>` 태그가 붙는다.
- 게임 자체는 **게임팩** 으로 갈아끼운다 (`game/packs/<id>/` 폴더 하나가 팩 하나, 폴더를 넣고 빼면 된다). 지금 들어 있는 팩:
  - **해바라기 농장** (`farm`) — `Farmer` 트레이트를 구현해 농부의 두뇌를 만든다. 걷기 → 거두기·넣기 → Option → 필드와 &mut self →
    홍수 enum → 고블린과 여러 명 → 씨 뿌리고 물 주기(튜플 match) → 수확 철(Vec). 11개 레벨 (8: Vec·반복자로 가장 가까운 열매 찾기, 9: 농부 3명과 enum 으로 나눈 일, 10: impl 로 코드 나누기, 11: trait 로 만든 일 목록). 그림은 Sunnyside World (아래 출처).

  꽃게 이야기였던 옛 팩(조수 웅덩이, 퍼즐)은 `game/packs-archive/` 에 보관해 두었다. `game/packs/` 로 옮기면 다시 목록에 뜬다.

## 실행

Rust 툴체인(`cargo`)과 Godot 4.7 이 필요하다 (`brew install --cask godot`). 기록을 남기려면 git 도.

```
cargo build                              # 게임 본체의 Rust 확장
godot --headless --path game --import    # 처음 한 번 (스크립트 클래스 등록)
godot --path game                        # 게임 실행
```

처음 실행하면 작업 폴더를 만드는 창이 뜬다: 위치(기본 `~/SproutCode`)와 **커밋에 쓸 이름·이메일** 을 정한다.
이름·이메일은 그 작업 폴더 저장소에만 설정되고(전역 git 설정은 그대로), 작업 폴더에서는 커밋 서명을 끈다
(혼자 보는 학습 기록이라 필요 없고, 서명 키 암호를 묻느라 게임이 멈추면 안 되므로).
그다음 게임의 **IDE 로 열기** 를 누르거나 직접 그 폴더를 IDE 로 연다.

```
~/SproutCode/
  farm/levels/04_maze.rs   ← 이런 파일을 고치고 저장한다
  farm/levels/07_sow.rs
  .crab/packs/                 게임팩 소스 (IDE 에서 F12 로 정의를 보면 여기로 온다. 게임이 관리한다)
  .crab/progress.json          클리어 기록
```

터미널에서 직접 돌려 볼 수도 있다: `cd ~/SproutCode && cargo run --bin tide` (결과가 글로 나온다).

환경 변수:

| 변수 | 뜻 |
|---|---|
| `CRAB_WORKSPACE` | 작업 폴더 위치. 이걸로 실행하면 개발·테스트 모드라 게임 설정 파일을 읽지도 쓰지도 않는다 |
| `CRAB_IDE` | **IDE 로 열기** 에 쓸 명령 (기본: VS Code 의 `code`). `<명령> <폴더> -g <파일:줄:칸>` 으로 부른다 |
| `CRAB_PACK`, `CRAB_START_LEVEL` | 시작할 게임팩과 레벨 번호 (개발용) |
| `CRAB_GIT_NAME`, `CRAB_GIT_EMAIL` | 작업 폴더 커밋의 이름·이메일 (주면 처음 창을 건너뛴다) |
| `CRAB_SHOT`, `CRAB_SHOT_DELAY` | 몇 초 뒤 화면을 PNG 로 찍고 끝내기 (개발용) |
| `CRAB_UI_SCALE` | UI 배율 (기본: 화면 배율, 레티나 2). 화면 사진을 늘 같게 찍으려면 1 로 (개발용) |

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
  pack.json        {"id", "name", "crate", "api", "tagline", "agent": "꽃게", "tick": "틱", "colors": [...], "chapters"?: [...]}
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
`use farm::level3::*;` 처럼 자기 레벨 모듈만 가져온다. 모듈에는 그 레벨에서 쓸 수 있는 `Farmer`·`View`·`Action`·`Dir`·`Tile`
만 짧게 정의돼 있고 새로 생긴 것에는 `(새로!)` 가 붙는다. 그래서 IDE 의 F12 가 그 짧은 파일로 가고, 아직 안 배운 것을 쓰면
rustc 가 "그런 변형/메서드는 없다" 고 알려 준다. 세계 시뮬레이션은 숨은 `engine` 모듈에 있고, `engine/bridge.rs` 가
레벨 타입과 엔진 타입을 서로 바꾼다.

**API 를 바꾸면 `pack.json` 의 `api` 를 올린다.** 작업 폴더는 게임팩마다 어느 `api` 로 만들어졌는지
`.crab/packs.json` 에 적어 두고, 버전이 바뀌면 지금 상태를 커밋한 뒤 예전 레벨 파일을 `<팩>/levels-v<예전 버전>/` 으로
옮겨 보관하고 새 처음 코드로 시작한다 (예전 파일은 새 크레이트와 맞지 않으므로).

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

## 그림

게임 화면은 16x16 픽셀 아트다. 16픽셀 격자에 그린 뒤 정수 배율로 키워서 픽셀이 뭉개지지 않게 한다.

- 지형·꽃게·갈매기·먹이·보석 등은 게임팩마다 `art/sprites.py` 의 **글자 격자** 로 그려져 있다 (글자 하나 = 픽셀 하나).
  격자를 고치고 실행하면 `view/assets/` 의 PNG 가 다시 만들어진다: `python3 game/packs/tidepool/art/sprites.py`
- 반짝이·물방울·느낌표·모래시계 같은 아이콘은 Kenney 의 1-Bit Pack 에서 가져왔다.
- 제목·버튼·HUD 는 픽셀 폰트(갈무리11), 긴 글(레슨 본문·출력)은 읽기 편한 Pretendard 와 JetBrains Mono.

## 함께 들어 있는 것

- [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono), [Pretendard](https://github.com/orioncactus/pretendard) — SIL Open Font License 1.1 (`game/fonts/`)
- [갈무리 Galmuri](https://github.com/quiple/galmuri) — SIL Open Font License 1.1 (`game/fonts/Galmuri-LICENSE.txt`)
- [Kenney 1-Bit Pack](https://kenney.nl/assets/1-bit-pack) — CC0 (`game/packs/tidepool/view/assets/kenney/`)

## 그림 출처

- **Sunnyside World** by danieldiggle — https://danieldiggle.itch.io/sunnyside (해바라기 농장 팩의 타일·캐릭터·작물·동물·UI).
  원본은 `game/packs/farm/art/sunnyside/` 에 있고, `python3 game/packs/farm/art/sunnyside.py` 가 게임용 그림을 만든다. 조건은 그 폴더의 README 참고.
- Kenney 1-Bit Pack (CC0), Galmuri (OFL).
