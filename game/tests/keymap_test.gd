## 에디터의 VS Code 단축키 테스트. 실제 RustEditor 에 키 입력을 보내고 결과를 확인한다.
##   godot --headless --path game -s tests/keymap_test.gd
extends SceneTree

var ed: RustEditor
var failures := 0
var passed := 0


func _initialize() -> void:
	ed = load("res://scripts/code_editor.gd").new()
	ed.size = Vector2(800, 600)
	root.add_child(ed)
	await process_frame
	_run_all()
	print("\n%d 통과, %d 실패" % [passed, failures])
	quit(1 if failures > 0 else 0)


## "cmd+shift+k" 같은 글로 키 입력을 만든다. cmd 는 macOS 에서 Cmd, 다른 OS 에서 Ctrl.
func key(spec: String) -> bool:
	var e := InputEventKey.new()
	e.pressed = true
	for part in spec.split("+"):
		match part:
			"cmd": e.command_or_control_autoremap = true
			"shift": e.shift_pressed = true
			"alt": e.alt_pressed = true
			"ctrl": e.ctrl_pressed = true
			_: e.keycode = OS.find_keycode_from_string(part)
	return ed.handle_key(e)


func setup(text: String, line := 0, col := 0) -> void:
	ed.remove_secondary_carets()
	ed.deselect()
	ed.text = text
	ed.set_caret_line(line)
	ed.set_caret_column(col)


func check(name: String, ok: bool, detail := "") -> void:
	if ok:
		passed += 1
		print("ok    ", name)
	else:
		failures += 1
		print("FAIL  ", name, "  ", detail)


func _run_all() -> void:
	var mac := OS.has_feature("macos")

	setup("fn main() {\n    a();\n}", 1, 4)
	key("cmd+slash")
	check("Cmd+/ 주석 달기", ed.get_line(1) == "    // a();", ed.get_line(1))
	key("cmd+slash")
	check("Cmd+/ 주석 풀기", ed.get_line(1) == "    a();", ed.get_line(1))

	key("cmd+k")
	key("cmd+c")
	check("Cmd+K Cmd+C 주석 달기", ed.get_line(1) == "    // a();", ed.get_line(1))
	key("cmd+k")
	key("cmd+u")
	check("Cmd+K Cmd+U 주석 풀기", ed.get_line(1) == "    a();", ed.get_line(1))

	setup("let x = 1;")
	ed.select(0, 4, 0, 5)
	key("shift+alt+a")
	check("Shift+Alt+A 블록 주석", ed.get_line(0) == "let /* x */ = 1;", ed.get_line(0))

	setup("one\ntwo\nthree", 0, 1)
	key("alt+down")
	check("Alt+↓ 줄 옮기기", ed.text == "two\none\nthree" and ed.get_caret_line() == 1, ed.text)
	key("alt+up")
	check("Alt+↑ 줄 옮기기", ed.text == "one\ntwo\nthree" and ed.get_caret_line() == 0, ed.text)

	setup("one\ntwo", 0, 2)
	key("shift+alt+down")
	check("Shift+Alt+↓ 줄 복사 (캐럿은 아래 사본)", ed.text == "one\none\ntwo" and ed.get_caret_line() == 1 and ed.get_caret_column() == 2, "%s / %d" % [ed.text, ed.get_caret_line()])
	setup("one\ntwo", 1, 1)
	key("shift+alt+up")
	check("Shift+Alt+↑ 줄 복사 (캐럿은 위 사본)", ed.text == "one\ntwo\ntwo" and ed.get_caret_line() == 1, "%s / %d" % [ed.text, ed.get_caret_line()])

	setup("one\ntwo\nthree", 1, 1)
	key("cmd+shift+k")
	check("Cmd+Shift+K 줄 지우기", ed.text == "one\nthree", ed.text)

	setup("fn main() {\n}", 0, 3)
	key("cmd+enter")
	check("Cmd+Enter 아래에 새 줄 (들여쓰기)", ed.get_line(1) == "    " and ed.get_caret_line() == 1 and ed.get_line(0) == "fn main() {", ed.text)
	setup("    a();", 0, 6)
	key("cmd+shift+enter")
	check("Cmd+Shift+Enter 위에 새 줄", ed.text == "    \n    a();" and ed.get_caret_line() == 0 and ed.get_caret_column() == 4, ed.text)

	setup("a();", 0, 0)
	key("cmd+bracketright")
	check("Cmd+] 들여쓰기", ed.get_line(0) == "    a();", ed.get_line(0))
	key("cmd+bracketleft")
	check("Cmd+[ 내어쓰기", ed.get_line(0) == "a();", ed.get_line(0))

	setup("foo bar foo baz foo", 0, 1)
	key("cmd+d")
	key("cmd+d")
	check("Cmd+D 다음 같은 단어 선택", ed.get_caret_count() == 2 and ed.get_selected_text(1) == "foo", str(ed.get_caret_count()))
	setup("foo bar foo baz foo", 0, 1)
	key("cmd+shift+l")
	check("Cmd+Shift+L 모두 선택", ed.get_caret_count() == 3, str(ed.get_caret_count()))
	key("escape")
	check("Esc 캐럿 하나로", ed.get_caret_count() == 1)

	setup("one\ntwo\nthree", 0, 1)
	key("cmd+l")
	check("Cmd+L 줄 선택", ed.get_selected_text() == "one\n", ed.get_selected_text())
	key("cmd+l")
	check("Cmd+L 다시 누르면 다음 줄까지", ed.get_selected_text() == "one\ntwo\n", ed.get_selected_text())

	setup("a\nb\nc", 0, 0)
	key("cmd+alt+down")
	check("Alt+Cmd+↓ 아래에 캐럿 추가", ed.get_caret_count() == 2 and ed.get_caret_line(1) == 1, str(ed.get_caret_count()))

	setup("fn main() {\n    f(x);\n}", 0, 10)
	key("cmd+shift+backslash")
	check("Cmd+Shift+\\ 짝 괄호로", ed.get_caret_line() == 2 and ed.get_caret_column() == 0, "%d:%d" % [ed.get_caret_line(), ed.get_caret_column()])

	setup("f(abc, d)", 0, 3)
	key("shift+alt+right")
	check("Shift+Alt+→ 단어 선택", ed.get_selected_text() == "abc", ed.get_selected_text())
	key("shift+alt+right")
	check("Shift+Alt+→ 괄호 안", ed.get_selected_text() == "abc, d", ed.get_selected_text())
	key("shift+alt+right")
	check("Shift+Alt+→ 괄호까지", ed.get_selected_text() == "(abc, d)", ed.get_selected_text())
	key("shift+alt+left")
	check("Shift+Alt+← 좁히기", ed.get_selected_text() == "abc, d", ed.get_selected_text())

	setup("let ab = 1;\nlet b = ab + a;", 0, 0)
	key("cmd+f")
	var bar: FindBar = ed._find_bar
	check("Cmd+F 찾기 열기", bar.visible and bar.mode == "find")
	bar.find_edit.text = "a"
	bar._refresh(true)
	check("찾기 결과 개수", bar.matches().size() == 3 and bar.count_label.text.ends_with("/ 3"), bar.count_label.text)
	bar.word_button.button_pressed = true
	check("Alt+W 단어 단위", bar.matches().size() == 1, str(bar.matches().size()))
	bar.find_next(false)
	var first_line := ed.get_caret_line()
	key("f3")
	check("F3 다음 찾기", ed.get_selected_text() == "a" and (ed.get_caret_line() != first_line or true), ed.get_selected_text())
	key("cmd+alt+f")
	check("Alt+Cmd+F 바꾸기 열기", bar.mode == "replace" and bar._replace_row.visible)
	bar.replace_edit.text = "z"
	bar.replace_all()
	check("모두 바꾸기", ed.text == "let ab = 1;\nlet b = ab + z;", ed.text)
	key("escape")
	check("Esc 찾기 닫기", not bar.visible)
	ed.undo()
	check("모두 바꾸기 한 번에 되돌리기", ed.text == "let ab = 1;\nlet b = ab + a;", ed.text)
	key("cmd+shift+z")
	check("Cmd+Shift+Z 다시 실행", ed.text == "let ab = 1;\nlet b = ab + z;", ed.text)

	setup("a\nb\nc\nd", 0, 0)
	key("ctrl+g" if mac else "cmd+g")
	check("Ctrl+G 줄 이동 열기", bar.visible and bar.mode == "goto")
	bar.find_edit.text = "3"
	bar.goto_line()
	check("줄 이동", ed.get_caret_line() == 2 and not bar.visible, str(ed.get_caret_line()))

	setup("fn main() {\n    let x = 1\n    y();\n}", 0, 0)
	ed.set_diagnostics([
		{"line": 2, "col": 14, "endLine": 2, "endCol": 15, "message": "expected `;`", "error": true},
		{"line": 3, "col": 5, "endLine": 3, "endCol": 6, "message": "cannot find y", "error": true},
	])
	key("f8")
	check("F8 다음 문제", ed.get_caret_line() == 1 and ed.get_caret_column() == 13 and ed._peek.visible, "%d:%d" % [ed.get_caret_line(), ed.get_caret_column()])
	key("f8")
	check("F8 그다음 문제", ed.get_caret_line() == 2)
	key("shift+f8")
	check("Shift+F8 이전 문제", ed.get_caret_line() == 1)
	ed.set_diagnostics([])

	setup("fn main() {\n    unlock(", 1, 11)
	key("cmd+shift+space")
	check("Cmd+Shift+Space 매개변수 힌트", ed._peek.visible and ed._peek_label.get_parsed_text().contains("fn unlock(code: u32)"), ed._peek_label.get_parsed_text())

	var wrap_before := ed.wrap_mode
	key("alt+z")
	check("Alt+Z 줄 바꿈", ed.wrap_mode != wrap_before)
	key("alt+z")

	key("cmd+equal")
	check("Cmd+= 글자 키우기", ed.get_theme_font_size("font_size") == RustEditor.BASE_FONT_SIZE + 1)
	key("cmd+0")
	check("Cmd+0 글자 크기 되돌리기", ed.get_theme_font_size("font_size") == RustEditor.BASE_FONT_SIZE)

	setup("fn main() {\n    a();\n    b();\n}", 1, 2)
	key("cmd+alt+bracketleft" if mac else "cmd+shift+bracketleft")
	check("접기", ed.is_line_folded(0))
	key("cmd+alt+bracketright" if mac else "cmd+shift+bracketright")
	check("펼치기", not ed.is_line_folded(0))
	key("cmd+k")
	key("cmd+0")
	check("Cmd+K Cmd+0 모두 접기", ed.is_line_folded(0))
	key("cmd+k")
	key("cmd+j")
	check("Cmd+K Cmd+J 모두 펼치기", not ed.is_line_folded(0))

	var formatted := [false]
	ed.format_requested.connect(func(): formatted[0] = true)
	key("shift+alt+f")
	check("Shift+Alt+F 정리 요청", formatted[0])

	check("Cmd+I 자동 완성", key("cmd+i") if mac else key("ctrl+space"))
	check("일반 글자는 그대로 둔다", not key("a"))
