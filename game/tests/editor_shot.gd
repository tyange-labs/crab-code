## 게임 화면에서 찾기 상자, 문제 보기(F8), rustfmt 정리를 확인하고 화면을 찍는다.
##   godot --path game -s tests/editor_shot.gd -- 출력.png
extends SceneTree


func _initialize() -> void:
	var out := OS.get_cmdline_user_args()[0] if OS.get_cmdline_user_args().size() > 0 else "editor.png"
	OS.set_environment("CRAB_START", "7")
	var main: Control = load("res://main.tscn").instantiate()
	root.add_child(main)
	await create_timer(0.5).timeout
	var ed: RustEditor = main.code
	# rustfmt
	ed.text = "fn main(){let mut count=0;\nfor _ in 0..4{move_forward();collect();count+=1;}\nmove_forward();unlock(count);}"
	ed.format_requested.emit()
	print("formatted:\n", ed.text)
	# 일부러 에러를 내고 진단을 기다린다
	ed.text = ed.text.replace("let mut count", "let count")
	main._check_code()
	await create_timer(3.0).timeout
	var k := InputEventKey.new()
	k.pressed = true
	k.keycode = KEY_F8
	ed.handle_key(k)
	var f := InputEventKey.new()
	f.pressed = true
	f.keycode = KEY_F
	f.command_or_control_autoremap = true
	ed.handle_key(f)
	ed._find_bar.find_edit.text = "count"
	ed._find_bar._refresh(true)
	ed._show_peek(ed._peek_label.text, ed.get_caret_line())
	await create_timer(0.4).timeout
	root.get_texture().get_image().save_png(out)
	quit()
