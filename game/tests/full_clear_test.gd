## 모든 레벨을 정답으로 차례로 깨 보는 점검: 레벨마다 "IDE 에서 정답을 저장한 것처럼" 파일을 바꾸고,
## 게임이 빌드·실행·재생해 성공하는지, 클리어 기록과 git 커밋·태그가 쌓이는지 본다. (오래 걸린다)
##   CRAB_WORKSPACE=/tmp/crab-full godot --path game -s tests/full_clear_test.gd
extends SceneTree

var failures := 0


func check(name: String, ok: bool, detail := "") -> void:
	print("ok    " if ok else "FAIL  ", name, "" if ok else "  " + detail)
	if not ok:
		failures += 1


func wait_for(cond: Callable, seconds: float) -> bool:
	var t := 0.0
	while t < seconds:
		if cond.call():
			return true
		await create_timer(0.1).timeout
		t += 0.1
	return cond.call()


func _initialize() -> void:
	var ws := OS.get_environment("CRAB_WORKSPACE")
	if ws == "" or not ws.begins_with("/tmp"):
		print("CRAB_WORKSPACE=/tmp/... 로 실행해 주세요 (진짜 작업 폴더를 건드리지 않게).")
		quit(2)
		return
	OS.execute("rm", ["-rf", ws])
	OS.set_environment("CRAB_START_LEVEL", "1")
	var pack_id := OS.get_environment("CRAB_PACK")
	if pack_id == "":
		pack_id = "station"
		OS.set_environment("CRAB_PACK", pack_id)
	OS.set_environment("CRAB_GIT_NAME", "Full Test")
	OS.set_environment("CRAB_GIT_EMAIL", "full-test@localhost")
	var host: Control = load("res://host.tscn").instantiate()
	root.add_child(host)
	await create_timer(0.5).timeout
	var total: int = host.pack.levels.size()
	for n in total:
		var lesson: Dictionary = host.pack.levels[n]
		host.select_level(n)
		await wait_for(func(): return host.phase == "idle" and not host.result.is_empty(), 120.0)
		var file: String = host.files.get(lesson.id, "")
		var f := FileAccess.open(file, FileAccess.WRITE)
		f.store_string(lesson.solution)
		f.close()
		var won := await wait_for(func(): return host.phase == "won", 180.0)
		check("%2d. %s 클리어" % [n + 1, lesson.id], won, host.phase)
		if not won:
			print("      첫 에러: ", host._first_error)
	await create_timer(1.0).timeout
	var tags := []
	OS.execute("git", ["-C", ws, "tag"], tags)
	var tag_text: String = tags[0] if tags.size() > 0 else ""
	var missing := []
	for lv in host.pack.levels:
		if not tag_text.contains("best/%s/%s" % [pack_id, lv.id]):
			missing.append(lv.id)
	check("모든 레벨에 최고 기록 태그", missing.is_empty(), str(missing))
	var count := []
	OS.execute("git", ["-C", ws, "rev-list", "--count", "HEAD"], count)
	var commits := int((count[0] if count.size() > 0 else "0").strip_edges())
	check("클리어 커밋이 레벨 수만큼 쌓임 (%d 개)" % commits, commits >= total, str(commits))
	var done: int = range(total).filter(func(i): return host.progress.get(host.pack.levels[i].id, {}).get("cleared", false)).size()
	check("진행 기록에 %d / %d 개 클리어" % [done, total], done == total, str(done))
	print("모두 통과" if failures == 0 else "%d개 실패" % failures)
	quit(1 if failures > 0 else 0)
