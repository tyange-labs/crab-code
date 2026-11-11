# Crab Code

꽃게 페리스를 Rust 코드로 움직이며 Rust 를 배우는 퍼즐 게임.

에디터에 쓴 코드를 진짜 `rustc` 로 컴파일해서 실행하기 때문에, 컴파일 에러도 실제 Rust 컴파일러 메시지 그대로 나온다.
레벨 51개는 『프로그래밍 러스트』(개정판)의 목차 순서(2장~17장)를 따른다.

## 기술 스택

[Tauri 2](https://tauri.app) 앱이다.

- **백엔드 (Rust)** — 플레이어 코드를 `rustc` 로 컴파일·실행하고, 월드를 시뮬레이션한다.
- **화면 (웹뷰)** — 빌드 도구 없는 순수 HTML/CSS/JS. 에디터는 [Monaco](https://microsoft.github.io/monaco-editor/)(VS Code 의 에디터), 맵은 Canvas 로 그린다.

## 실행

Rust 툴체인(`rustc`, `cargo`)만 있으면 된다. Node 나 npm 은 필요 없다.

```
cargo run --release
```

- 실행: F5 또는 Cmd+Enter
- 입력을 멈추면 컴파일 에러가 에디터에 밑줄로 표시된다.
- 게임 함수(`move_forward` 등)는 자동 완성되고, 마우스를 올리면 설명이 나온다.
- 코드와 진행 상황은 `~/.crab-code/` 에 저장된다.

## 레벨 검증

모든 레벨의 정답 코드가 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.

```
cargo run --release -- --verify
```

## 구조

- `src/main.rs` — Tauri 명령(화면에서 부르는 함수)과 레벨 검증.
- `src/sim.rs` — 월드 시뮬레이션과 레벨 지도. 게임과 플레이어 프로그램이 함께 쓴다.
- `src/runner.rs` — 플레이어 코드를 `rustc` 로 컴파일하고 실행한다.
- `src/lessons.rs` — 레슨 본문, 시작 코드, 정답.
- `runtime/crab.rs` — 플레이어 코드에서 부르는 함수들 (`move_forward`, `collect` 등).
- `ui/` — 화면. `app.js` 가 전부이고, `vendor/monaco/` 는 Monaco 배포본(Rust 문법만 남김)이다.

## 함께 들어 있는 것

- [Monaco Editor](https://github.com/microsoft/monaco-editor) 0.52.2 — MIT (`ui/vendor/monaco/LICENSE`)
- [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono), [Pretendard](https://github.com/orioncactus/pretendard) — SIL Open Font License 1.1 (`ui/fonts/`)
