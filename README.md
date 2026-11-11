# Crab Code

꽃게 페리스를 Rust 코드로 움직이며 Rust 를 배우는 퍼즐 게임. [macroquad](https://macroquad.rs) 로 만들었다.

에디터에 쓴 코드를 진짜 `rustc` 로 컴파일해서 실행하기 때문에, 컴파일 에러도 실제 Rust 컴파일러 메시지 그대로 나온다.
레벨 51개는 『프로그래밍 러스트』(개정판)의 목차 순서(2장~17장)를 따른다.

## 실행

Rust 툴체인(`rustc`, `cargo`)이 필요하다. macOS 시스템 폰트를 읽어 쓰므로 다른 OS 에서는 한글이 깨질 수 있다.

```
cargo run --release
```

- 실행: F5 또는 Cmd+Enter
- 코드와 진행 상황은 `~/.crab-code/` 에 저장된다.

## 레벨 검증

모든 레벨의 정답 코드가 통과하고 시작 코드는 통과하지 못하는지 실제로 컴파일해서 확인한다.

```
cargo run --release -- --verify
```

## 구조

- `src/sim.rs` — 월드 시뮬레이션과 레벨 지도. 게임과 플레이어 프로그램이 함께 쓴다.
- `runtime/crab.rs` — 플레이어 코드에서 부르는 함수들 (`move_forward`, `collect` 등).
- `src/runner.rs` — 플레이어 코드를 `rustc` 로 컴파일하고 실행한다.
- `src/lessons.rs` — 레슨 본문, 시작 코드, 정답.
- `src/editor.rs`, `src/ui.rs`, `src/main.rs` — 에디터와 화면.
