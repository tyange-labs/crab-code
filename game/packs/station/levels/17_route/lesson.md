# 경로 계산 — 스레드와 채널

이번 구역은 **미로**예요. 코어가 어디 있는지는 `view.cores()` 로 알지만, 벽 때문에 **어느 길로 가야 하는지**는 스스로 계산해야 해요. 길 찾기(너비 우선 탐색)는 계산이 무거울 수 있으니, 로봇의 **본체와 별개로 일하는 계산 스레드**를 만들어요.

**새로 생긴 것은 없어요.** 쓰는 건 지금까지 배운 `view.tile_at`, `view.cores()`, `view.size()` 와 표준 라이브러리의 `std::thread`, `std::sync::mpsc` 예요.

**스레드**는 동시에 돌아가는 일꾼이에요. `thread::spawn` 에 **`move` 클로저**를 주면 새 스레드가 그 일을 시작해요. `move` 는 클로저가 쓰는 값의 **주인을 새 스레드로 넘긴다**는 뜻이에요.

**채널**은 스레드끼리 값을 주고받는 통로예요. `channel()` 은 보내는 쪽 `Sender` 와 받는 쪽 `Receiver` 를 한 쌍으로 만들어요.

```rust
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

let (to_worker, requests) = channel::<Request>();   // 본체 → 계산 스레드
let (replies_tx, from_worker) = channel::<Vec<Dir>>(); // 계산 스레드 → 본체

thread::spawn(move || {
    for request in requests {            // 요청이 올 때까지 기다리며 하나씩 받아요
        let path = find_path(&request);  // 무거운 계산
        replies_tx.send(path).ok();      // 결과를 돌려 보내요
    }                                    // 보내는 쪽이 사라지면 for 가 끝나고 스레드도 끝나요
});

to_worker.send(request).unwrap();        // 요청 보내기
let path = from_worker.recv().unwrap();  // 답이 올 때까지 기다리기
```

`send` 로 보낸 값은 **주인이 받는 쪽으로 넘어가요** (복사하거나 빌려 주지 않아요). 그래서 스레드 사이에 값을 주고받아도 데이터 경합이 없어요. 스레드 사이를 오갈 수 있는 타입은 `Send` 여야 하는데, `Tile`, `Dir` 같은 단순한 값은 자동으로 그래요. (`View` 는 `Send` 가 아니라서 스레드에 넘길 수 없어요. 필요한 칸 정보를 **복사해서** 보내요.)

이 레벨의 흐름은 이래요.

1. `new` 에서 계산 스레드를 만들어요 (채널 두 쌍).
2. 코어를 들고 있지 않고 가야 할 길이 없으면, 지도를 `tile_at` 으로 훑어 **열린 칸**(`tile.is_open()`) 표를 만들고, 가장 가까운 코어까지의 요청을 보내 길을 받아요.
3. 받은 길(`Vec<Dir>`)을 한 틱에 한 칸씩 따라가요. 코어에 닿으면 집고, 들고 있을 땐 `reactor_dir()` 로 돌아가요.

목표: 260틱 안에 코어 3개를 반응로에 넣으세요.
