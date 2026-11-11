## Crab Code 의 화면 전체를 이끈다: 레슨 고르기, 실행, 결과 재생, 저장.
## 컴파일·실행·시뮬레이션은 Rust 확장(CrabCore)이 맡는다.
extends Control

const SAVE_PATH := "user://save.json"

@onready var code: RustEditor = %Code
@onready var world_view: WorldView = %World
@onready var console: RichTextLabel = %Console

var core := CrabCore.new()
var chapters: Array = []
var lessons: Array = []
var level := 0
var codes := {}
var cleared := {}
var phase := "idle" # idle | compiling | playing | won | failed
var outcome := {}
var dev := false
var confirm_solution := false

var _run_token := -1
var _run_level := -1
var _check_token := -1
var _check_version := ""


func _ready() -> void:
	theme = UiTheme.build()
	_style_static_nodes()
	var data: Dictionary = JsonUtil.parse(core.lessons())
	chapters = data.chapters
	lessons = data.lessons

	# 개발용 환경 변수: CRAB_START=레벨번호, CRAB_DEMO=1 (정답 자동 실행) 또는 start (시작 코드 실행).
	# 둘 중 하나라도 있으면 저장 파일을 읽지도 쓰지도 않는다. CRAB_SHOT=경로 면 몇 초 뒤 화면을 찍고 끝낸다.
	var start_env := OS.get_environment("CRAB_START")
	var demo_env := OS.get_environment("CRAB_DEMO")
	dev = start_env != "" or demo_env != ""
	if not dev:
		_load_save()

	%Run.pressed.connect(run)
	%Reset.pressed.connect(reset_code)
	%Hint.pressed.connect(func(): log_line("힌트: " + lesson().hint.replace("`", ""), "warn"))
	%Solution.pressed.connect(show_solution)
	%Speed.pressed.connect(func(): %Speed.text = "속도 %dx" % int(world_view.cycle_speed()))
	%Sound.pressed.connect(_toggle_sound)
	%Next.pressed.connect(func(): set_level(level + 1))
	code.idle_edited.connect(func():
		_save_code()
		_check_code())
	code.text_changed.connect(func():
		if confirm_solution:
			confirm_solution = false
			%Solution.text = "정답 보기")
	code.format_requested.connect(_format_code)
	world_view.printed.connect(log_line)
	world_view.world_changed.connect(_render_world_info)
	world_view.finished.connect(_finish_run)

	var first := 0
	for i in lessons.size():
		if not cleared.has(lessons[i].id):
			first = i
			break
	if start_env.is_valid_int():
		first = clampi(start_env.to_int(), 1, lessons.size()) - 1
	set_level(first)
	if demo_env != "":
		if demo_env != "start":
			code.text = lesson().solution
		run()

	var shot := OS.get_environment("CRAB_SHOT")
	if shot != "":
		var delay := OS.get_environment("CRAB_SHOT_DELAY")
		await get_tree().create_timer(delay.to_float() if delay.is_valid_float() else 2.0).timeout
		get_viewport().get_texture().get_image().save_png(shot)
		get_tree().quit()


func _style_static_nodes() -> void:
	%BrandName.add_theme_font_size_override("font_size", 21)
	%BrandSub.add_theme_color_override("font_color", Palette.DIM)
	%BrandSub.add_theme_font_size_override("font_size", 12)
	%ChapterName.add_theme_color_override("font_color", Palette.DIM)
	%ChapterName.add_theme_font_size_override("font_size", 13)
	%LessonTitle.add_theme_font_size_override("font_size", 21)
	%Progress.add_theme_color_override("font_color", Palette.GREEN)
	%Progress.add_theme_font_size_override("font_size", 13)
	%BannerText.callouts = false
	%TabName.add_theme_font_override("font", Palette.code_font())
	%TabName.add_theme_font_size_override("font_size", 13)
	%TabHint.add_theme_color_override("font_color", Palette.FAINT)
	%TabHint.add_theme_font_size_override("font_size", 12)
	%Run.theme_type_variation = "PrimaryButton"
	%ConsoleLabel.add_theme_color_override("font_color", Palette.DIM)
	%ConsoleLabel.add_theme_font_size_override("font_size", 12)
	console.add_theme_font_override("normal_font", Palette.code_font())
	console.add_theme_font_size_override("normal_font_size", 13)
	console.add_theme_constant_override("line_separation", 3)
	%TabBar.add_theme_stylebox_override("panel", UiTheme.box(Palette.CHROME, 0, 14, 8))


func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo:
		# 실행은 VS Code 처럼 F5 (디버깅 시작) 또는 Ctrl+F5 (디버깅 없이 실행).
		# Cmd+Enter 는 VS Code 와 같이 에디터의 "아래에 새 줄" 이다.
		if event.keycode == KEY_F5:
			get_viewport().set_input_as_handled()
			run()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		_save_code()
		core.cleanup()


func _process(_delta: float) -> void:
	var out := core.poll_run()
	if out != "":
		_on_run_result(JsonUtil.parse(out))
	out = core.poll_check()
	if out != "":
		var result: Dictionary = JsonUtil.parse(out)
		# 그 사이에 코드가 바뀌었으면 결과가 맞지 않으니 버린다.
		if result.token == _check_token and code.text == _check_version and phase != "compiling":
			code.set_diagnostics(result.diagnostics)


# ── 레슨과 저장 ───────────────────────────────────────────────────

func lesson() -> Dictionary:
	return lessons[level]


func _load_save() -> void:
	var text := FileAccess.get_file_as_string(SAVE_PATH)
	if text == "":
		_import_old_save()
		return
	var data = JSON.parse_string(text)
	if data is Dictionary:
		codes = data.get("codes", {})
		for id in data.get("cleared", []):
			cleared[id] = true


## 예전 버전(Tauri)이 ~/.crab-code 에 남긴 저장을 한 번 가져온다.
func _import_old_save() -> void:
	var home := OS.get_environment("HOME")
	if home == "":
		home = OS.get_environment("USERPROFILE")
	var dir := home.path_join(".crab-code")
	if home == "" or not DirAccess.dir_exists_absolute(dir):
		return
	for l in lessons:
		var path: String = dir.path_join(l.id + ".rs")
		if FileAccess.file_exists(path):
			codes[l.id] = FileAccess.get_file_as_string(path)
	var ids := FileAccess.get_file_as_string(dir.path_join("cleared.txt"))
	for id in ids.split("\n", false):
		cleared[id.strip_edges()] = true
	_write_save()


func _write_save() -> void:
	if dev:
		return
	var f := FileAccess.open(SAVE_PATH, FileAccess.WRITE)
	if f:
		f.store_string(JSON.stringify({"codes": codes, "cleared": cleared.keys()}, "\t"))


func _save_code() -> void:
	if lessons.is_empty():
		return
	codes[lesson().id] = code.text
	_write_save()


func _random_seed() -> int:
	return randi() + 1


func set_level(index: int) -> void:
	if code.text != "":
		_save_code()
	level = index
	_run_token = -1
	_check_token = -1
	var l := lesson()
	code.text = codes.get(l.id, l.start)
	code.clear_undo_history()
	code.set_diagnostics([])
	confirm_solution = false
	%Solution.text = "정답 보기"
	world_view.set_world(JsonUtil.parse(core.world(level, _random_seed())))
	_set_phase("idle")
	_intro()
	_render_lesson()
	code.set_caret_line(0)
	code.set_caret_column(0)
	code.grab_focus()
	_check_code()


func _check_code() -> void:
	_check_version = code.text
	_check_token = core.start_check(code.text)


# ── 레슨 표시 ────────────────────────────────────────────────────

static func _escape(s: String) -> String:
	return s.replace("[", "[lb]")


func _chip(label: String, current: bool, done: bool, on_press: Callable) -> Button:
	var b := Button.new()
	b.text = label
	b.theme_type_variation = "ChipCurrent" if current else ("ChipDone" if done else "Chip")
	b.focus_mode = Control.FOCUS_NONE
	b.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
	b.pressed.connect(on_press)
	return b


func _levels_in(chapter: int) -> Array:
	return range(lessons.size()).filter(func(i): return lessons[i].chapter == chapter)


func _render_lesson() -> void:
	var l := lesson()
	for c in %Chapters.get_children():
		c.queue_free()
	for c in chapters.size():
		var levels := _levels_in(c)
		var done := levels.all(func(i): return cleared.has(lessons[i].id))
		# "12·13장 연산자와 …" 에서 앞부분만 칩에 쓴다.
		var chip_label: String = chapters[c].name.get_slice(" ", 0)
		%Chapters.add_child(_chip(chip_label, c == l.chapter, done, func():
			if c == lesson().chapter:
				return
			# 그 장에서 아직 깨지 않은 첫 레벨로 간다.
			var next: int = levels[0]
			for i in levels:
				if not cleared.has(lessons[i].id):
					next = i
					break
			set_level(next)))

	for c in %Levels.get_children():
		c.queue_free()
	var levels := _levels_in(l.chapter)
	for n in levels.size():
		var i: int = levels[n]
		%Levels.add_child(_chip(str(n + 1), i == level, cleared.has(lessons[i].id), func():
			if i != level:
				set_level(i)))

	var done_count := lessons.filter(func(x): return cleared.has(x.id)).size()
	%Progress.text = "클리어 %d / %d" % [done_count, lessons.size()]
	%ChapterName.text = "%s · 레벨 %d / %d" % [chapters[l.chapter].name, levels.find(level) + 1, levels.size()]
	%LessonTitle.text = "%d. %s" % [levels.find(level) + 1, l.title]
	%LessonBody.markdown = l.body
	%LessonScroll.scroll_vertical = 0


func _goal_text() -> String:
	for line in lesson().body.split("\n"):
		if line.begins_with("목표:"):
			return line
	return ""


# ── 출력 창 ──────────────────────────────────────────────────────

const TONES := {
	"bad": "#ff6b7a", "warn": "#f2c94c", "good": "#5fd38d", "dim": "#717791", "": "#e4e6f0",
}


func log_line(text: String, tone := "") -> void:
	console.append_text("[color=%s]%s[/color]\n" % [TONES.get(tone, TONES[""]), _escape(text)])


func _clear_console() -> void:
	console.clear()


func _intro() -> void:
	_clear_console()
	log_line("실행 버튼(F5)을 누르면 코드를 진짜 rustc 로 컴파일해서 돌려요.", "dim")
	log_line("println! 출력과 컴파일러 메시지가 여기에 나옵니다.", "dim")


func _log_compiler(output: String, warnings_only: bool) -> void:
	for line in output.split("\n"):
		var t := line.strip_edges(true, false)
		var tone := ""
		if t.begins_with("error"):
			tone = "bad"
		elif t.begins_with("warning"):
			tone = "warn"
		elif t.begins_with("help") or t.begins_with("= help"):
			tone = "good"
		elif t.begins_with("-->") or t.begins_with("= note") or t.begins_with("note"):
			tone = "dim"
		if warnings_only and tone != "warn":
			tone = "dim"
		log_line(line, tone)


# ── 실행 ─────────────────────────────────────────────────────────

func _set_phase(p: String, message := "") -> void:
	phase = p
	%Run.text = "컴파일 중..." if p == "compiling" else "실행 (F5)"
	%Run.disabled = p == "compiling"
	%Next.visible = false
	var bg := Palette.PANEL_HI
	var color := Palette.TEXT
	var text := ""
	match p:
		"failed":
			bg = Palette.RED_BG
			color = Palette.RED
			text = message
		"won":
			bg = Palette.GREEN_BG
			color = Palette.GREEN
			var last := level + 1 >= lessons.size()
			text = "졸업을 축하해요! 이제 진짜 Rust 를 쓸 준비가 됐어요." if last else "성공! 잘했어요."
			%Next.visible = not last
		"compiling":
			color = Palette.DIM
			text = "`rustc` 가 코드를 컴파일하는 중..."
		_:
			# 목표는 본문을 스크롤하지 않아도 항상 보이게 여기에 고정한다.
			text = _goal_text()
	%Banner.add_theme_stylebox_override("panel", UiTheme.box(bg, 10, 14, 10))
	%BannerText.base_color = color
	%BannerText.markdown = text


func run() -> void:
	if phase == "compiling" or lessons.is_empty():
		return
	_save_code()
	world_view.stop()
	_set_phase("compiling")
	_clear_console()
	log_line("컴파일 중...", "dim")
	Sfx.play("compile")
	_check_token = -1
	_run_level = level
	_run_token = core.start_run(code.text, level, _random_seed())


func _on_run_result(result: Dictionary) -> void:
	if result.token != _run_token or _run_level != level:
		return
	_clear_console()
	code.set_diagnostics(result.diagnostics)
	world_view.set_world(result.world)

	if not result.compiled:
		_log_compiler(result.output, false)
		console.scroll_to_line(0)
		var line := code.first_error_line()
		if line >= 0:
			code.set_caret_line(line)
			code.center_viewport_to_caret()
		_set_phase("failed", "컴파일 에러! 아래 컴파일러 메시지를 읽어 보세요.")
		Sfx.play("fail")
		return
	if result.output != "":
		_log_compiler(result.output, true)
	outcome = {"won": result.won, "message": result.message}
	_set_phase("playing")
	world_view.play(result.steps)


func _finish_run() -> void:
	if phase != "playing":
		return
	if not outcome.won:
		log_line(outcome.message, "bad")
		_set_phase("failed", outcome.message)
		Sfx.play("fail")
		return
	log_line(outcome.message, "good")
	cleared[lesson().id] = true
	_write_save()
	_set_phase("won")
	_render_lesson()
	world_view.celebrate()
	Sfx.play("win")


func reset_code() -> void:
	code.replace_all(lesson().start)
	world_view.set_world(JsonUtil.parse(core.world(level, _random_seed())))
	_set_phase("idle")
	_intro()


func show_solution() -> void:
	if not confirm_solution:
		confirm_solution = true
		%Solution.text = "정말 볼까요?"
		return
	code.replace_all(lesson().solution)


## Shift+Alt+F: rustfmt 로 코드를 정리한다.
func _format_code() -> void:
	var result: Dictionary = JsonUtil.parse(core.format(code.text))
	if result.ok:
		if result.code != code.text:
			code.replace_all(result.code, true)
		return
	_clear_console()
	log_line("rustfmt 가 코드를 정리하지 못했어요. 문법 에러부터 고쳐 보세요.", "warn")
	for line in String(result.error).split("\n"):
		log_line(line, "dim")


func _toggle_sound() -> void:
	Sfx.enabled = not Sfx.enabled
	%Sound.text = "소리 켜짐" if Sfx.enabled else "소리 꺼짐"


# ── 가방과 정보 ──────────────────────────────────────────────────

func _render_world_info(w: Dictionary) -> void:
	for c in %Bag.get_children():
		c.queue_free()
	var title := Label.new()
	title.text = "가방"
	title.add_theme_color_override("font_color", Palette.DIM)
	%Bag.add_child(title)
	var held: Array = w.gems.filter(func(g): return g.held)
	if held.is_empty():
		var empty := Label.new()
		empty.text = "비어 있음"
		empty.add_theme_color_override("font_color", Palette.FAINT)
		%Bag.add_child(empty)
	for g in held:
		var icon := GemIcon.new()
		icon.gem = g
		%Bag.add_child(icon)

	for c in %Info.get_children():
		c.queue_free()
	var items := []
	if w.get("dial") != null:
		items.append(["다이얼", str(w.dial)])
	if not w.scroll.is_empty():
		items.append(["두루마리", "[%s]" % ", ".join(PackedStringArray(w.scroll.map(func(n): return str(n))))])
	if w.get("note") != null:
		items.append(["쪽지", JSON.stringify(w.note)])
	if w.get("note2") != null:
		items.append(["쪽지 2", JSON.stringify(w.note2)])
	%Info.visible = not items.is_empty()
	for item in items:
		var row := HBoxContainer.new()
		row.add_theme_constant_override("separation", 6)
		var label := Label.new()
		label.text = item[0]
		label.add_theme_color_override("font_color", Palette.DIM)
		row.add_child(label)
		var value := MarkdownView.new()
		value.fit_width = true
		value.size_flags_horizontal = Control.SIZE_SHRINK_BEGIN
		value.size_flags_vertical = Control.SIZE_SHRINK_CENTER
		value.markdown = "``%s``" % item[1].replace("`", "'")
		row.add_child(value)
		%Info.add_child(row)


## 가방에 든 보석 아이콘.
class GemIcon extends Control:
	var gem: Dictionary

	func _init() -> void:
		custom_minimum_size = Vector2(20, 20)

	func _draw() -> void:
		CrabArt.draw_gem(self, size / 2.0, 8.0, gem)
