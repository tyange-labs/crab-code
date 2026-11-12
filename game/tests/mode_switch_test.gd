## 퍼즐 모드 ↔ 꽃게 AI 모드 전환 확인.
##   godot --headless --path game -s tests/mode_switch_test.gd
extends SceneTree


func _initialize() -> void:
	change_scene_to_file("res://pool.tscn")
	await create_timer(1.0).timeout
	var a := current_scene.name
	current_scene.get_node("%Mode").pressed.emit()
	await create_timer(1.0).timeout
	var b := current_scene.name
	current_scene.get_node("%Mode").pressed.emit()
	await create_timer(1.0).timeout
	var c := current_scene.name
	print("switch: ", a, " -> ", b, " -> ", c)
	quit(0 if [a, b, c] == ["PoolMain", "Main", "PoolMain"] else 1)
