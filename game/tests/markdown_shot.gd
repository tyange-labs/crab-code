## 마크다운 미리 보기 확인용: 모든 문법을 한 화면에 그려 PNG 로 저장한다.
##   godot --path game -s tests/markdown_shot.gd -- 출력.png
extends SceneTree

const SAMPLE := """# 제목 1
## 제목 2
보통 문단에 `인라인 코드`, **굵게**, *기울임*, ``백틱 `이` 든 코드``, [링크](https://www.rust-lang.org) 가 섞여 있어요.
줄바꿈도 그대로 살아요. 아주아주 긴 문장이 이어지면 어절 단위로 줄이 바뀌어야 하고 `let mut count = 0;` 같은 코드는 중간에서 끊기면 안 됩니다.

- 목록 첫째 `move_forward()`
- 목록 둘째 **중요**
1. 번호 목록
2. 둘째 번호

> 인용문은 흐리게, 왼쪽에 막대.

```rust
fn main() {
    // 주석
    let n: u32 = 42;
    println!("{}", n);
}
```

---

목표: 콜아웃 상자 안의 `코드` 도 잘 보여야 해요.
"""


func _initialize() -> void:
	var out := OS.get_cmdline_user_args()[0] if OS.get_cmdline_user_args().size() > 0 else "markdown.png"
	var bg := ColorRect.new()
	bg.color = Palette.PANEL
	bg.size = Vector2(680, 860)
	root.add_child(bg)
	var md := MarkdownView.new()
	md.position = Vector2(20, 20)
	md.size = Vector2(640, 800)
	md.markdown = SAMPLE
	bg.add_child(md)
	await process_frame
	await process_frame
	await create_timer(0.5).timeout
	root.get_texture().get_image().get_region(Rect2i(0, 0, 680, 860)).save_png(out)
	quit()
