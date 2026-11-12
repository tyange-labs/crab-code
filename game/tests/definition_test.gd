## crab.rs 탭과 정의로 가기(F12 · Cmd+클릭 · 레슨 코드 링크) 확인.
##   godot --path game -s tests/definition_test.gd -- [화면.png]
extends SceneTree

var failures := 0


func check(name: String, ok: bool, detail := "") -> void:
	print("ok    " if ok else "FAIL  ", name, "" if ok else "  " + detail)
	if not ok:
		failures += 1


func key(ed: RustEditor, code: int, ctrl := false) -> void:
	var e := InputEventKey.new()
	e.pressed = true
	e.keycode = code
	e.ctrl_pressed = ctrl
	ed.handle_key(e)


func _initialize() -> void:
	OS.set_environment("CRAB_POOL_START", "4")
	var main: Control = load("res://pool.tscn").instantiate()
	root.add_child(main)
	await create_timer(0.6).timeout
	var code: RustEditor = main.code
	var ref: RustEditor = main.ref_code

	check("crab.rs 에 Action 정의가 있다", ref.text.contains("pub enum Action {"))
	check("읽기 전용", not ref.editable)
	var before := ref.text
	ref.set_caret_line(10)
	var slash := InputEventKey.new()
	slash.pressed = true
	slash.keycode = KEY_SLASH
	slash.command_or_control_autoremap = true
	ref.handle_key(slash)
	check("읽기 전용 탭에서는 Cmd+/ 가 내용을 바꾸지 않는다", ref.text == before)

	# 내 코드에서 Tile 위에 캐럿을 두고 F12
	var line := -1
	for i in code.get_line_count():
		if code.get_line(i).contains("Tile::Food"):
			line = i
			break
	code.set_caret_line(line)
	code.set_caret_column(code.get_line(line).find("Tile") + 1)
	key(code, KEY_F12)
	check("F12 → crab.rs 탭", main._tab == "crab" and ref.visible and not code.visible)
	check("F12 → pub enum Tile 줄", ref.get_line(ref.get_caret_line()).begins_with("pub enum Tile"), ref.get_line(ref.get_caret_line()))
	key(ref, KEY_MINUS, true) if OS.has_feature("macos") else null
	if not OS.has_feature("macos"):
		var back := InputEventKey.new()
		back.pressed = true
		back.keycode = KEY_LEFT
		back.alt_pressed = true
		ref.handle_key(back)
	check("돌아가기 → ferris.rs 원래 줄", main._tab == "ferris" and code.get_caret_line() == line)

	# 점 뒤 이름은 메서드로: view.tick → View::tick, 그냥 tick → Crab::tick
	main._go_to_definition("tick", true)
	check("view.tick → View 의 메서드", ref.get_line(ref.get_caret_line()).contains("pub fn tick(&self) -> u32"), ref.get_line(ref.get_caret_line()))
	main._go_to_definition("tick", false)
	check("tick → Crab 트레이트", ref.get_line(ref.get_caret_line()).contains("fn tick(&mut self, view: &View) -> Action"), ref.get_line(ref.get_caret_line()))

	# 레슨 본문의 코드 링크
	var cases := {
		"Tide": "pub enum Tide",
		"Tile::Water": "Water,",
		"view.tide()": "pub fn tide(&self)",
		"Tide::Low { turns_left } if turns_left >= 6": "Low { turns_left: u32 },",
	}
	for t in cases:
		var hit: Dictionary = main._resolve_code(t)
		check("레슨 코드 `%s` 는 링크" % t, not hit.is_empty())
		if not hit.is_empty():
			main._go_to_definition(hit.word, hit.after_dot)
			check("  → %s" % cases[t], ref.get_line(ref.get_caret_line()).contains(cases[t]), ref.get_line(ref.get_caret_line()))
	check("`match` 는 링크 아님", main._resolve_code("match").is_empty())

	main._go_to_definition("Tide", false)
	await create_timer(0.3).timeout
	var out := OS.get_cmdline_user_args()
	if out.size() > 0:
		root.get_texture().get_image().save_png(out[0])
	print("%s" % ("모두 통과" if failures == 0 else "%d개 실패" % failures))
	quit(1 if failures > 0 else 0)
