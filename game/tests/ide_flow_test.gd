## IDE 흐름 확인: 새 작업 폴더를 만들고, 파일을 "IDE 에서 저장"한 것처럼 바꾼 뒤
## 게임이 알아채서 빌드·실행·재생하고, 클리어를 git 에 커밋하는지 본다.
##   CRAB_WORKSPACE=/tmp/crab-ws godot --path game -s tests/ide_flow_test.gd -- [화면.png]
extends SceneTree

var failures := 0


func check(name: String, ok: bool, detail := "") -> void:
	print("ok    " if ok else "FAIL  ", name, "" if ok else "  " + detail)
	if not ok:
		failures += 1


func wait_for(host: Control, cond: Callable, seconds: float) -> bool:
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
		pack_id = "tidepool"
		OS.set_environment("CRAB_PACK", pack_id)
	var through_dialog := OS.get_environment("CRAB_TEST_SETUP") != ""
	if not through_dialog:
		OS.set_environment("CRAB_GIT_NAME", "Crab Test")
		OS.set_environment("CRAB_GIT_EMAIL", "crab-test@localhost")
	var host: Control = load("res://host.tscn").instantiate()
	root.add_child(host)
	await create_timer(0.5).timeout
	if through_dialog:
		check("처음 실행하면 작업 폴더 만들기 창이 뜸", not host.setup_dialog.is_empty())
		check("작업 폴더가 아직 없음", not DirAccess.dir_exists_absolute(ws))
		host.setup_dialog.name.text = "Crab Test"
		host.setup_dialog.email.text = "no-at-sign"
		host.setup_dialog.start.pressed.emit()
		check("이메일이 이상하면 막음", host.setup_dialog.error.visible)
		host.setup_dialog.email.text = "crab-test@localhost"
		host.setup_dialog.start.pressed.emit()
		await create_timer(0.5).timeout
		check("창이 닫히고 작업 폴더가 생김", host.setup_dialog.is_empty() and DirAccess.dir_exists_absolute(ws))

	check("작업 폴더가 git 저장소", DirAccess.dir_exists_absolute(ws.path_join(".git")))
	var first_id: String = host.pack.levels[0].id
	var file: String = host.files.get(first_id, "")
	check("레벨 파일이 생김", FileAccess.file_exists(file), file)
	check("Cargo.toml 이 있음", FileAccess.file_exists(ws.path_join("Cargo.toml")))

	var built := await wait_for(host, func(): return host.phase == "idle" and not host.result.is_empty(), 60.0)
	check("처음 코드가 빌드되어 첫 모습이 보임", built, host.phase)

	# IDE 에서 정답을 저장한 것처럼 파일을 바꾼다.
	var lesson: Dictionary = host.lesson()
	var f := FileAccess.open(file, FileAccess.WRITE)
	f.store_string(lesson.solution)
	f.close()
	var won := await wait_for(host, func(): return host.phase == "won", 60.0)
	check("저장을 알아채고 빌드·실행·재생해서 성공", won, host.phase)

	var log := []
	OS.execute("git", ["-C", ws, "log", "--format=%s"], log)
	var subjects: String = log[0] if log.size() > 0 else ""
	check("클리어가 git 에 커밋됨", subjects.contains("클리어"), subjects)
	var author := []
	OS.execute("git", ["-C", ws, "log", "-1", "--format=%an <%ae>"], author)
	check("커밋 이름·이메일은 작업 폴더 설정을 따름", (author[0] if author.size() > 0 else "").strip_edges() == "Crab Test <crab-test@localhost>", str(author))
	var sign := []
	OS.execute("git", ["-C", ws, "config", "--local", "--get", "commit.gpgsign"], sign)
	check("작업 폴더에서는 서명을 끔", (sign[0] if sign.size() > 0 else "").strip_edges() == "false", str(sign))
	var raw := []
	OS.execute("git", ["-C", ws, "cat-file", "-p", "HEAD"], raw)
	check("커밋에 서명이 없음", not (raw[0] if raw.size() > 0 else "").contains("gpgsig"))
	var tags := []
	OS.execute("git", ["-C", ws, "tag"], tags)
	check("최고 기록 태그", (tags[0] if tags.size() > 0 else "").contains("best/%s/%s" % [pack_id, first_id]))

	# 컴파일 에러: 세미콜론을 하나 지운다.
	f = FileAccess.open(file, FileAccess.WRITE)
	f.store_string(lesson.solution + "\nfn broken() { let x: u32 = \"문자열\"; }\n")
	f.close()
	var failed := await wait_for(host, func(): return host.phase == "failed", 60.0)
	check("컴파일 에러를 보여 줌", failed and not host._first_error.is_empty(), str(host._first_error))

	f = FileAccess.open(file, FileAccess.WRITE)
	f.store_string(lesson.solution)
	f.close()
	await wait_for(host, func(): return host.phase == "won", 60.0)
	await create_timer(1.0).timeout
	var out := OS.get_cmdline_user_args()
	if out.size() > 0:
		root.get_texture().get_image().save_png(out[0])
	print("모두 통과" if failures == 0 else "%d개 실패" % failures)
	quit(1 if failures > 0 else 0)
