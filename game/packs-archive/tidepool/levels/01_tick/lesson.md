# 꽃게의 두뇌, tick

지금부터 **꽃게의 두뇌**를 만들어요. 꽃게를 직접 조종하지 않아요. 게임이 매 **틱**(한 박자)마다 꽃게의 `tick` 을 불러 "이번엔 뭘 할래?" 하고 묻고, 꽃게는 할 일 하나를 돌려줘요.

파일을 보면 세 부분이 있어요.

- `struct Ferris;` — 꽃게 한 마리를 나타내는 타입이에요.
- `impl Crab for Ferris { … }` — `Ferris` 가 꽃게의 두뇌(`Crab` 트레이트) 노릇을 하게 해요. `new` 는 꽃게를 만들고, `tick` 은 할 일을 골라요.
- `fn main() { play::<Ferris>(); }` — 이 웅덩이를 `Ferris` 로 플레이해요.

`tick` 이 돌려주는 `Action` 은 **함수가 아니라 값**이에요. `Action::Wait` 는 "가만히 있겠다"는 뜻의 값이고, `Action::Move(Dir::East)` 는 "동쪽으로 걷겠다"는 뜻의 값이에요. `Move` 는 방향(`Dir`) 값을 하나 품고 있어요.

지금 쓸 수 있는 것은 이게 다예요.

- `Action::Wait`, `Action::Move(dir)`
- `Dir::North`, `Dir::East`, `Dir::South`, `Dir::West`
- `view.tick()` — 지금 몇 번째 틱인지

IDE 에서 `Action` 위에 커서를 두고 **F12**(정의로 이동)를 누르면 이 웅덩이에서 쓸 수 있는 것만 모아 둔 파일이 열려요. 웅덩이를 지날 때마다 하나씩 늘어나요.

목표: 동쪽의 먹이가 있는 칸까지 걸어가세요.
