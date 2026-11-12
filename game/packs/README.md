# 게임팩 (`game/packs/`)

게임 하나가 폴더 하나예요. 폴더를 넣고 빼면 게임이 바뀌어요. 자세한 형식은 저장소 README 의 "게임팩" 절을 보세요.

## 분류

| 폴더 | 이름 | 상태 | 이야기 | 레벨 |
|---|---|---|---|---|
| `station/` | 정거장 복구 | **활성** | 버려진 우주정거장에서 정비 로봇의 두뇌를 짠다 (어두운 SF, 그림은 코드로 그림) | 23 |
| `farm/` | 해바라기 농장 | 보관 | 농부를 움직여 씨를 뿌리고 열매를 거둔다 (Sunnyside World 그림 사용, 정거장 복구의 원형) | 14 |
| `puzzle/` | 퍼즐 | 보관 | 함수로 꽃게를 움직인다 (『프로그래밍 러스트』 2~17장 순서) | 51 |
| `tidepool/` | 조수 웅덩이 | 보관 | 꽃게의 두뇌를 짠다 | 6 |

## 보관 팩

`pack.json` 에 `"archived": true` 가 있는 팩은 게임의 팩 목록에서 숨겨져요. 지우지 않고 남겨 둔 거예요.

- 보관 팩을 보려면: `CRAB_SHOW_ARCHIVED=1 godot --path game`
- 다시 활성으로 돌리려면: 그 팩의 `pack.json` 에서 `"archived": true` 를 지워요.
- `farm/` 의 그림은 Sunnyside World (danieldiggle, itch.io) 에서 가져왔고 재배포 조건이 있어요. 조건은 `farm/art/sunnyside/README.md` 를 보세요.

## 새 팩 만들기

`station/` 을 복사해서 `pack.json` 의 `id`·`name`·`crate` 와 레벨, 그림(`art/draw.py` → `view/assets/`)을 바꾸세요.
