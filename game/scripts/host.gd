## Crab Code 게임 본체.
##
## 코드는 플레이어의 IDE 에서 고친다. 이 화면은 게임팩을 고르고, 작업 폴더(cargo 워크스페이스 + git)를
## 준비하고, 파일이 저장되면 빌드·실행해서 게임팩의 화면(packs/<팩>/view/view.gd)으로 재생한다.
## 어떤 게임팩인지는 모른다. 그 세계를 그리는 일은 게임팩 화면이 맡는다.
extends Control

const SETTINGS_PATH := "user://host.json"
const TICKS_PER_SECOND := 5.0
const SPEEDS := [0.5, 1.0, 2.0, 4.0]
const WATCH_INTERVAL := 0.4

var core := CrabCore.new()
var packs_dir := ProjectSettings.globalize_path("res://packs")
var root := ""
## CRAB_WORKSPACE 로 작업 폴더를 정해 실행했으면 (개발·테스트) 설정 파일을 읽지도 쓰지도 않는다.
var dev := false
## 작업 폴더의 커밋에만 쓸 이름·이메일.
var git_name := ""
var git_email := ""
## 처음 실행할 때 뜨는 작업 폴더 만들기 창 (테스트가 들여다본다): {overlay, path, name, email, error, start}
var setup_dialog := {}
var packs: Array = []
var pack: Dictionary = {}
var level := 0
var files := {} # 레벨 id -> 절대 경로
var progress := {}
var view: Control = null

var phase := "idle" # idle | building | ready | won | failed
var result: Dictionary = {}
var playing := false
var autoplay := true
var speed_idx := 1
var _marks: TimelineMarks
var show_solution := false
var confirm_solution := false

var _run_token := -1
var _run_level := ""
var _shown_tick := -1
var _slider_lock := false
var _watch_clock := 0.0
var _queued_run := false
var _first_error: Dictionary = {}
var _build_started := 0
## 상태 바의 주 버튼이 지금 하는 일: "" | "ide" | "run" | "error" | "next" | "replay"
var _action := ""
## 상태 바 상세 글을 잠깐 바꿔 둔 경우 (IDE 를 열었다 등), 원래 글로 돌아갈 시각.
var _flash_until := 0
var _detail := ""

enum MoreItem { OPEN_IDE, OPEN_FOLDER, COPY_COMMAND }


func _ready() -> void:
	theme = UiTheme.build()
	_style_static_nodes()
	root = OS.get_environment("CRAB_WORKSPACE")
	dev = root != ""
	var settings := _load_settings()
	if root == "":
		root = settings.get("workspace", core.default_workspace())
	git_name = OS.get_environment("CRAB_GIT_NAME")
	git_email = OS.get_environment("CRAB_GIT_EMAIL")
	if git_name == "" and git_email == "":
		git_name = settings.get("git_name", "")
		git_email = settings.get("git_email", "")

	packs = JsonUtil.parse(core.packs(packs_dir))
	%PackPicker.clear()
	for p in packs:
		%PackPicker.add_item(p.name)
	%PackPicker.item_selected.connect(func(i): _select_pack(i, -1))
	%LevelPicker.item_selected.connect(func(index: int):
		var target: int = %LevelPicker.get_item_id(index)
		if target != level:
			select_level(target))

	%Hint.pressed.connect(_toggle_hint)
	%Solution.pressed.connect(_toggle_solution)
	var lesson_more: PopupMenu = %LessonMore.get_popup()
	lesson_more.add_item("처음 코드로 되돌리기…", 0)
	lesson_more.id_pressed.connect(func(_id: int):
		%ResetCard.visible = true)
	%ResetConfirm.pressed.connect(func():
		%ResetCard.visible = false
		_reset_level())
	%ResetCancel.pressed.connect(func(): %ResetCard.visible = false)
	%Sound.pressed.connect(_toggle_sound)
	%StatusAction.pressed.connect(_on_status_action)
	%TabOutput.pressed.connect(func(): _show_tab("output"))
	%TabProblems.pressed.connect(func(): _show_tab("problems"))
	_render_problems([])
	_show_tab("output")
	var more: PopupMenu = %More.get_popup()
	more.add_item("IDE 로 열기", MoreItem.OPEN_IDE)
	more.add_item("작업 폴더 열기", MoreItem.OPEN_FOLDER)
	more.add_item("터미널 명령 복사 (cargo run)", MoreItem.COPY_COMMAND)
	more.id_pressed.connect(_on_more)
	%Play.pressed.connect(_toggle_play)
	%Back.pressed.connect(func(): _step_by(-1))
	%Forward.pressed.connect(func(): _step_by(1))
	var group := ButtonGroup.new()
	for i in SPEEDS.size():
		var b := Button.new()
		b.text = ("%sx" % SPEEDS[i]).replace(".0x", "x")
		b.toggle_mode = true
		b.button_group = group
		b.button_pressed = i == speed_idx
		b.focus_mode = Control.FOCUS_NONE
		b.theme_type_variation = "ChipCurrent" if i == speed_idx else "Chip"
		b.tooltip_text = "재생 속도 (%d)" % (i + 1)
		b.pressed.connect(func(): _set_speed(i))
		%Speeds.add_child(b)
	_marks = TimelineMarks.new()
	%Slider.add_child(_marks)
	%Slider.value_changed.connect(_on_slider)

	if packs.is_empty():
		_set_phase("failed", "게임팩을 찾지 못했어요: %s" % packs_dir)
		return
	var pack_index := 0
	for i in packs.size():
		if packs[i].id == OS.get_environment("CRAB_PACK") or (OS.get_environment("CRAB_PACK") == "" and packs[i].id == settings.get("pack", "")):
			pack_index = i
	var start_level := -1
	if OS.get_environment("CRAB_START_LEVEL").is_valid_int():
		start_level = OS.get_environment("CRAB_START_LEVEL").to_int() - 1
	var first_run := not dev and not DirAccess.dir_exists_absolute(root.path_join(".git"))
	if core.git_available() and (git_name == "" or git_email == "" or first_run):
		# 처음이면 작업 폴더 위치와 커밋에 쓸 이름·이메일부터 정한다 (저장해 둔 값이 있으면 채워 둔다).
		_show_setup(func(): _select_pack(pack_index, start_level))
	else:
		_select_pack(pack_index, start_level)

	var shot := OS.get_environment("CRAB_SHOT")
	if shot != "":
		var delay := OS.get_environment("CRAB_SHOT_DELAY")
		await get_tree().create_timer(delay.to_float() if delay.is_valid_float() else 3.0).timeout
		get_viewport().get_texture().get_image().save_png(shot)
		get_tree().quit()


func _style_static_nodes() -> void:
	%LessonTitle.theme_type_variation = "PixelTitle"
	for l in [%Progress, %ChapterName, %TickLabel, %StatusTitle, %GoalLabel, %InspectorLabel, %Slash]:
		l.add_theme_font_override("font", Palette.pixel_font())
	%Slash.add_theme_color_override("font_color", Palette.TEXT3)
	%Slash.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%GoalLabel.add_theme_color_override("font_color", Palette.ACCENT)
	%GoalLabel.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%GoalText.callouts = false
	%HintText.callouts = false
	%HintLabel.add_theme_font_override("font", Palette.pixel_font())
	%HintLabel.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%HintLabel.add_theme_color_override("font_color", Palette.WARN)
	var hint_box := UiTheme.box(Color(Palette.WARN, 0.08), 8, 14, 10)
	hint_box.border_color = Color(Palette.WARN, 0.7)
	hint_box.set_border_width_all(2)
	%HintCard.add_theme_stylebox_override("panel", hint_box)
	var reset_box := UiTheme.box(Palette.PANEL_HI, 8, 14, 10)
	reset_box.border_color = Color(Palette.RED, 0.5)
	reset_box.set_border_width_all(2)
	%ResetCard.add_theme_stylebox_override("panel", reset_box)
	%ResetTitle.add_theme_font_override("font", Palette.pixel_font())
	%ResetTitle.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%ResetNote.add_theme_font_size_override("font_size", Palette.SIZE_SMALL)
	%ResetNote.add_theme_color_override("font_color", Palette.TEXT2)
	%LessonMore.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	var goal := UiTheme.box(Color(Palette.ACCENT, 0.08), 8, 14, 10)
	goal.border_width_left = 3
	goal.border_color = Palette.ACCENT
	%GoalCard.add_theme_stylebox_override("panel", goal)
	%InspectorLabel.add_theme_color_override("font_color", Palette.TEXT3)
	%InspectorLabel.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%Progress.add_theme_color_override("font_color", Palette.GREEN)
	for p in [%PackPicker, %LevelPicker]:
		p.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%ChapterName.add_theme_color_override("font_color", Palette.DIM)
	%ChapterName.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)

	%Progress.add_theme_color_override("font_color", Palette.GREEN)
	%Progress.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%StatusTitle.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%StatusDetail.add_theme_font_size_override("font_size", Palette.SIZE_SMALL)
	%StatusDetail.add_theme_color_override("font_color", Palette.TEXT2)
	%FileName.add_theme_font_override("font", Palette.code_font())
	%FileName.add_theme_font_size_override("font_size", Palette.SIZE_CODE)
	%FileName.add_theme_color_override("font_color", Palette.CODE)
	%More.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%TickLabel.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)
	%TickLabel.add_theme_color_override("font_color", Palette.DIM)
	%Console.add_theme_font_override("normal_font", Palette.code_font())
	%Console.add_theme_font_size_override("normal_font_size", 13)
	%Console.add_theme_constant_override("line_separation", 3)
	%InspectorText.add_theme_font_override("normal_font", Palette.code_font())
	%InspectorText.add_theme_font_size_override("normal_font_size", 13)
	for b in [%Back, %Play, %Forward]:
		b.add_theme_font_size_override("font_size", Palette.PIXEL_SIZE)


func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo and event.keycode == KEY_F5:
		get_viewport().set_input_as_handled()
		run(true)


## 단축키. 글자를 입력하는 칸(처음 실행 창 등)이 먼저 받으므로 _unhandled_input 에서 처리한다.
func _unhandled_input(event: InputEvent) -> void:
	if not (event is InputEventKey) or not event.pressed or not setup_dialog.is_empty():
		return
	var e := event as InputEventKey
	var cmd := e.is_command_or_control_pressed()
	var handled := true
	match e.keycode:
		KEY_SPACE:
			_toggle_play()
		KEY_LEFT:
			_step_by(-1)
		KEY_RIGHT:
			_step_by(1)
		KEY_HOME:
			_jump_to(0.0)
		KEY_END:
			_jump_to(INF)
		KEY_1, KEY_2, KEY_3, KEY_4:
			if cmd:
				handled = false
			else:
				_set_speed(e.keycode - KEY_1)
		KEY_R when not cmd:
			if not result.is_empty():
				_jump_to(0.0)
				_toggle_play()
		KEY_H when not cmd:
			_toggle_hint()
		KEY_O when cmd:
			_open_ide()
		KEY_E when cmd:
			if not _first_error.is_empty():
				_open_ide(_first_error)
		KEY_ENTER, KEY_KP_ENTER:
			if phase == "won" and level + 1 < pack.levels.size():
				select_level(level + 1)
			else:
				handled = false
		_:
			handled = false
	if handled:
		get_viewport().set_input_as_handled()


## 타임라인 위 표시: 사건이 있는 틱(한 가지 색)과 끝의 결과(성공 초록·실패 빨강).
## 사건 종류마다 색을 나누려면 팩이 events 에 kind 를 실어야 한다 (design/UI_REDESIGN.md 5장).
func _update_marks() -> void:
	var ticks: Array = []
	if not result.is_empty():
		for t in range(1, view.last_tick() + 1):
			if not view.frame_at(t).get("events", []).is_empty():
				ticks.append(t)
	_marks.ticks = ticks
	_marks.last = view.last_tick()
	_marks.outcome = "" if result.is_empty() else ("won" if result.won else "failed")
	_marks.queue_redraw()


func _set_speed(i: int) -> void:
	speed_idx = clampi(i, 0, SPEEDS.size() - 1)
	for n in %Speeds.get_child_count():
		var b: Button = %Speeds.get_child(n)
		b.set_pressed_no_signal(n == speed_idx)
		b.theme_type_variation = "ChipCurrent" if n == speed_idx else "Chip"


func _jump_to(t: float) -> void:
	if result.is_empty():
		return
	playing = false
	%Play.text = "재생"
	view.time = clampf(t, 0.0, view.last_tick())
	_shown_tick = -1
	_sync_timeline()


# ── 설정 ─────────────────────────────────────────────────────────

func _load_settings() -> Dictionary:
	if dev or not FileAccess.file_exists(SETTINGS_PATH):
		return {}
	var data = JSON.parse_string(FileAccess.get_file_as_string(SETTINGS_PATH))
	return data if data is Dictionary else {}


func _save_settings() -> void:
	if dev:
		return
	var settings := _load_settings()
	settings.workspace = root
	settings.git_name = git_name
	settings.git_email = git_email
	if not pack.is_empty():
		settings.pack = pack.id
		settings.level = lesson().get("id", "")
	var f := FileAccess.open(SETTINGS_PATH, FileAccess.WRITE)
	if f:
		f.store_string(JSON.stringify(settings, "\t"))


# ── 처음 실행: 작업 폴더 만들기 ───────────────────────────────────────

func _show_setup(then: Callable) -> void:
	var overlay := ColorRect.new()
	overlay.color = Color(0.05, 0.06, 0.09, 0.82)
	overlay.set_anchors_preset(Control.PRESET_FULL_RECT)
	overlay.mouse_filter = Control.MOUSE_FILTER_STOP
	add_child(overlay)
	var center := CenterContainer.new()
	center.set_anchors_preset(Control.PRESET_FULL_RECT)
	overlay.add_child(center)
	var panel := PanelContainer.new()
	panel.custom_minimum_size = Vector2(560, 0)
	panel.add_theme_stylebox_override("panel", UiTheme.box(Palette.PANEL, 14, 28, 24))
	center.add_child(panel)
	var col := VBoxContainer.new()
	col.add_theme_constant_override("separation", 12)
	panel.add_child(col)

	var title := Label.new()
	title.text = "작업 폴더 만들기"
	title.add_theme_font_size_override("font_size", 22)
	col.add_child(title)
	var intro := MarkdownView.new()
	intro.callouts = false
	intro.markdown = "코드는 **내가 쓰는 IDE** 에서 짜요. 게임이 그 코드를 담을 cargo 워크스페이스를 만들고, 레벨을 깰 때마다 **git 커밋**으로 풀이를 남겨요."
	col.add_child(intro)

	var identity: Dictionary = JsonUtil.parse(core.git_identity())
	var path := _setup_field(col, "위치", root)
	var name_edit := _setup_field(col, "커밋 이름", git_name if git_name != "" else identity.get("name", ""))
	var email_edit := _setup_field(col, "커밋 이메일", git_email if git_email != "" else identity.get("email", ""))
	var note := Label.new()
	note.text = "이 이름·이메일은 작업 폴더의 커밋에만 쓰여요. 전역 git 설정은 바꾸지 않고, 커밋에 서명도 하지 않아요."
	note.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	note.add_theme_font_size_override("font_size", 13)
	note.add_theme_color_override("font_color", Palette.DIM)
	col.add_child(note)
	var error := Label.new()
	error.add_theme_color_override("font_color", Palette.RED)
	error.visible = false
	col.add_child(error)
	var start := Button.new()
	start.text = "시작하기"
	start.theme_type_variation = "PrimaryButton"
	start.custom_minimum_size = Vector2(0, 42)
	col.add_child(start)
	setup_dialog = {"overlay": overlay, "path": path, "name": name_edit, "email": email_edit, "error": error, "start": start}

	var submit := func() -> void:
		var p := path.text.strip_edges()
		if p.begins_with("~"):
			p = OS.get_environment("HOME") + p.substr(1)
		var n := name_edit.text.strip_edges()
		var e := email_edit.text.strip_edges()
		if p == "" or n == "" or not e.contains("@"):
			error.text = "위치, 이름, 이메일(@ 포함)을 모두 적어 주세요."
			error.visible = true
			return
		root = p
		git_name = n
		git_email = e
		_save_settings()
		overlay.queue_free()
		setup_dialog = {}
		then.call()
	start.pressed.connect(submit)
	for edit in [path, name_edit, email_edit]:
		edit.text_submitted.connect(func(_t): submit.call())
	name_edit.grab_focus()


func _setup_field(col: VBoxContainer, label: String, value: String) -> LineEdit:
	var row := VBoxContainer.new()
	row.add_theme_constant_override("separation", 4)
	var l := Label.new()
	l.text = label
	l.add_theme_font_size_override("font_size", 13)
	l.add_theme_color_override("font_color", Palette.DIM)
	row.add_child(l)
	var edit := LineEdit.new()
	edit.text = value
	edit.custom_minimum_size = Vector2(0, 36)
	edit.add_theme_font_override("font", Palette.code_font())
	edit.add_theme_font_size_override("font_size", 14)
	edit.add_theme_stylebox_override("normal", UiTheme.box(Palette.EDITOR, 6, 10, 4))
	var focus := UiTheme.box(Palette.EDITOR, 6, 10, 4)
	focus.border_color = Palette.SKY
	focus.set_border_width_all(1)
	edit.add_theme_stylebox_override("focus", focus)
	row.add_child(edit)
	col.add_child(row)
	return edit


# ── 게임팩과 레벨 ─────────────────────────────────────────────────

func lesson() -> Dictionary:
	if pack.is_empty() or pack.levels.is_empty():
		return {}
	return pack.levels[level]


func _select_pack(index: int, start_level: int) -> void:
	pack = packs[index]
	%PackPicker.select(index)
	%PackPicker.tooltip_text = "게임팩 바꾸기 · " + String(pack.get("tagline", ""))
	var ws: Dictionary = JsonUtil.parse(core.ensure_workspace(root, packs_dir, pack.id, git_name, git_email))
	if not ws.get("ok", false):
		_set_phase("failed", "작업 폴더를 준비하지 못했어요: %s" % ws.get("error", ""))
		return
	files = ws.files
	if ws.get("note", "") != "":
		_log(ws.note, "warn")
	if ws.get("migrated") != null:
		_log("게임팩이 업데이트되어 레벨 파일을 새 처음 코드로 바꿨어요. 예전 파일은 %s/ 에 보관했고, git 에도 남아 있어요." % ws.migrated, "warn")

	# 게임팩 화면을 바꿔 끼운다.
	if view:
		view.queue_free()
	view = load("res://packs/%s/view/view.gd" % pack.id).new()
	view.set_anchors_preset(Control.PRESET_FULL_RECT)
	%ViewHost.add_child(view)
	view.agent_selected.connect(func(_i): _render_inspector())

	progress = JsonUtil.parse(core.progress(root)).get(pack.id, {})
	var first := 0
	for i in pack.levels.size():
		if not progress.get(pack.levels[i].id, {}).get("cleared", false):
			first = i
			break
	var saved: String = _load_settings().get("level", "")
	for i in pack.levels.size():
		if pack.levels[i].id == saved:
			first = i
	select_level(start_level if start_level >= 0 else first)
	if ws.get("fresh", false):
		_log("작업 폴더를 새로 만들었어요: %s" % root, "good")
		_log("IDE 로 열고 %s 을(를) 고쳐 보세요. 저장하면 여기서 바로 실행돼요." % lesson().file, "good")


func select_level(index: int) -> void:
	level = clampi(index, 0, pack.levels.size() - 1)
	show_solution = false
	confirm_solution = false
	%Solution.text = "정답 보기"
	%HintCard.visible = false
	%Hint.text = "힌트"
	%ResetCard.visible = false
	result = {}
	playing = false
	_render_lesson()
	_render_workspace()
	_save_settings()
	# 지금 파일로 한 번 돌려서 처음 모습을 보여 준다 (재생은 멈춘 채).
	run(false)


## 지금 레벨이 든 장 (장이 없는 게임팩이면 빈 딕셔너리).
func _chapter() -> Dictionary:
	for ch in pack.get("chapters", []):
		if level + 1 >= ch.from and level + 1 <= ch.to:
			return ch
	return {}


func _render_lesson() -> void:
	var l := lesson()
	var chapters: Array = pack.get("chapters", [])
	var ch := _chapter()
	var cleared_of := func(n: int) -> bool: return progress.get(pack.levels[n].id, {}).get("cleared", false)

	# 레벨 고르기: 장이 있는 팩은 장 이름으로 묶는다.
	%LevelPicker.clear()
	for n in pack.levels.size():
		for c in chapters:
			if c.from - 1 == n:
				%LevelPicker.add_separator(c.name)
		%LevelPicker.add_item(("✓ " if cleared_of.call(n) else "") + "%d. %s" % [n + 1, pack.levels[n].title], n)
	%LevelPicker.select(%LevelPicker.get_item_index(level))

	# 레벨 칩: 지금 장(장이 없으면 전부)
	for c in %Levels.get_children():
		c.queue_free()
	var lo: int = ch.from - 1 if not ch.is_empty() else 0
	var hi: int = ch.to if not ch.is_empty() else pack.levels.size()
	for i in range(lo, hi):
		var b := Button.new()
		b.text = str(i - lo + 1)
		b.theme_type_variation = "ChipCurrent" if i == level else ("ChipDone" if cleared_of.call(i) else "Chip")
		b.focus_mode = Control.FOCUS_NONE
		b.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
		b.tooltip_text = pack.levels[i].title
		b.pressed.connect(func():
			if i != level:
				select_level(i))
		%Levels.add_child(b)
	var done: int = range(pack.levels.size()).filter(func(n): return cleared_of.call(n)).size()
	%Progress.text = "%d / %d" % [done, pack.levels.size()]

	var meta := PackedStringArray()
	if not ch.is_empty():
		meta.append(ch.name)
	meta.append("레벨 %d / %d" % [level - lo + 1, hi - lo])
	var best: Dictionary = progress.get(l.id, {}).get("best", {})
	if not best.is_empty():
		meta.append("최고 기록 %d%s, %d줄" % [best.ticks, pack.get("tick", "틱"), best.lines])
	%ChapterName.text = " · ".join(meta)
	%LessonTitle.text = "%d. %s" % [level - lo + 1, l.title]

	# 목표는 맨 위 카드에 고정하고, 본문에서는 뺀다.
	var goal := _goal_text().trim_prefix("목표:").strip_edges()
	%GoalCard.visible = goal != ""
	%GoalText.markdown = goal
	var paras := PackedStringArray()
	for para in String(l.lesson).split("\n\n"):
		if not para.strip_edges().begins_with("목표:"):
			paras.append(para)
	var body := "\n\n".join(paras)
	if show_solution:
		body += "\n\n## 정답\n\n```rust\n%s\n```" % l.solution.strip_edges()
	%LessonBody.markdown = body
	if not show_solution:
		%LessonScroll.scroll_vertical = 0


func _render_workspace() -> void:
	var l := lesson()
	%FileName.text = String(l.file).get_file()
	var tip := "%s\n작업 폴더: %s" % [files.get(l.id, l.file), root]
	var commit = JsonUtil.parse(core.last_commit(root))
	if commit is Dictionary:
		tip += "\n마지막 커밋: %s %s (%s)" % [commit.hash, commit.subject, commit.when]
	elif not core.git_available():
		tip += "\ngit 이 없어서 기록을 남기지 못해요"
	%FileName.tooltip_text = tip
	if phase in ["idle", ""] and result.is_empty():
		_status("idle", "저장을 기다리는 중", "IDE 에서 저장하면 바로 빌드하고 실행해요", "ide")


## 상태 바 상세 글을 잠깐 바꾼다 (4초 뒤 원래 글로).
func _flash(text: String) -> void:
	%StatusDetail.text = text
	_flash_until = Time.get_ticks_msec() + 4000


## 상태 바 하나에 지금 상태를 보여 준다.
## kind: idle | ready | building | error | playing | won | failed. action: 주 버튼 ("" 이면 숨김).
func _status(kind: String, title: String, detail := "", action := "") -> void:
	var bg := Palette.PANEL
	var dot := Palette.TEXT3
	var fg := Palette.TEXT
	match kind:
		"building":
			bg = Palette.SKY.darkened(0.78)
			dot = Palette.SKY
		"error", "failed":
			bg = Palette.RED_BG
			dot = Palette.RED
			fg = Palette.RED
		"playing":
			dot = Palette.ACCENT
		"won":
			bg = Palette.GREEN_BG
			dot = Palette.GREEN
			fg = Palette.GREEN
		"ready":
			dot = Palette.GREEN
	var sb := UiTheme.box(bg, 8, 14, 6)
	sb.border_color = Palette.BORDER if kind in ["idle", "ready", "playing"] else bg.lightened(0.15)
	sb.set_border_width_all(2)
	%StatusBar.add_theme_stylebox_override("panel", sb)
	%StatusDot.color = dot
	%StatusTitle.text = title
	%StatusTitle.add_theme_color_override("font_color", fg)
	_detail = detail
	if Time.get_ticks_msec() >= _flash_until:
		%StatusDetail.text = detail
	_action = action
	var labels := {"ide": "IDE 로 열기", "run": "실행 F5", "error": "IDE 에서 보기", "next": "다음 레벨", "replay": "다시 보기"}
	%StatusAction.visible = action != ""
	%StatusAction.text = labels.get(action, "")
	var keys := {"ide": "⌘O", "run": "F5", "error": "⌘E", "next": "Enter", "replay": "R"}
	%StatusAction.tooltip_text = "단축키 " + keys.get(action, "") if keys.has(action) else ""
	%StatusAction.theme_type_variation = "PrimaryButton" if action in ["ide", "next", "run"] else ("DangerButton" if action == "error" else "OutlineButton")


func _on_more(id: int) -> void:
	if id == MoreItem.OPEN_IDE:
		_open_ide()
	elif id == MoreItem.OPEN_FOLDER:
		OS.shell_open(root)
	elif id == MoreItem.COPY_COMMAND:
		DisplayServer.clipboard_set("cd \"%s\" && cargo run --bin %s" % [root, lesson().id])
		_flash("터미널 명령을 복사했어요. 터미널에서 돌리면 결과가 글로 나와요.")


func _on_status_action() -> void:
	match _action:
		"ide":
			_open_ide()
		"run":
			run(true)
		"error":
			_open_ide(_first_error)
		"next":
			select_level(level + 1)
		"replay":
			_toggle_play()


# ── IDE ─────────────────────────────────────────────────────────

## IDE 로 작업 폴더와 파일을 연다. at 이 있으면 그 줄로 ({file, line, col}).
## CRAB_IDE 로 명령을 바꿀 수 있다 (기본: VS Code 의 code).
func _open_ide(at: Dictionary = {}) -> void:
	var file: String = files.get(lesson().id, "")
	var target := file
	if not at.is_empty():
		var f: String = at.get("file", "")
		target = (f if f.is_absolute_path() else root.path_join(pack.id).path_join(f)) + ":%d:%d" % [at.get("line", 1), at.get("col", 1)]
	var ide := OS.get_environment("CRAB_IDE")
	var candidates: Array = [ide] if ide != "" else ["code", "/usr/local/bin/code", "/opt/homebrew/bin/code",
		"/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"]
	for cmd in candidates:
		if cmd != "" and OS.execute(cmd, [root, "-g", target]) == 0:
			_flash("IDE 에서 열었어요. 고치고 저장하면 바로 실행돼요.")
			return
	# IDE 명령을 못 찾으면 운영체제가 정한 프로그램으로 파일을 연다.
	OS.shell_open(file)
	_flash("VS Code 명령(code)을 못 찾아 기본 프로그램으로 열었어요. CRAB_IDE 로 IDE 명령을 정할 수 있어요.")


# ── 실행 ─────────────────────────────────────────────────────────

## 지금 레벨 파일을 빌드하고 실행한다. play 면 결과를 바로 재생한다.
func run(play: bool) -> void:
	if lesson().is_empty():
		return
	if phase == "building":
		# 빌드 중에 또 저장하면 끝나고 한 번 더 돌린다.
		_queued_run = true
		autoplay = autoplay or play
		return
	autoplay = play
	playing = false
	_build_started = Time.get_ticks_msec()
	_set_phase("building")
	if play:
		Sfx.play("compile")
	_run_level = lesson().id
	_run_token = core.start_run(root, packs_dir, pack.id, lesson().id, randi() + 1)


func _process(delta: float) -> void:
	var out := core.poll_run()
	if out != "":
		_on_run_result(JsonUtil.parse(out))
	if phase == "building":
		%StatusTitle.text = "cargo build 중… %.1f초" % ((Time.get_ticks_msec() - _build_started) / 1000.0)
	if _flash_until > 0 and Time.get_ticks_msec() >= _flash_until:
		_flash_until = 0
		%StatusDetail.text = _detail

	_watch_clock += delta
	if _watch_clock >= WATCH_INTERVAL and not pack.is_empty():
		_watch_clock = 0.0
		var changed: Array = JsonUtil.parse(core.poll_changes(root, packs_dir, pack.id))
		if not changed.is_empty():
			# 다른 레벨 파일을 저장했으면 그 레벨로 옮겨 간다.
			var id: String = changed[-1]
			if id != lesson().id:
				for i in pack.levels.size():
					if pack.levels[i].id == id:
						level = i
						show_solution = false
						_render_lesson()
						_render_workspace()
						_save_settings()
			run(true)

	if playing and not result.is_empty() and view:
		var end := float(view.last_tick())
		view.time = minf(view.time + delta * TICKS_PER_SECOND * SPEEDS[speed_idx], end)
		_sync_timeline()
		if view.time >= end:
			playing = false
			%Play.text = "다시"
			_finish_run()


func _on_run_result(r: Dictionary) -> void:
	if r.token != _run_token:
		return
	if _queued_run:
		_queued_run = false
		phase = "idle"
		run(autoplay)
		return
	if _run_level != lesson().id:
		phase = "idle"
		return
	%Console.clear()
	_first_error = {}
	_render_problems(r.get("diagnostics", []))
	if not r.compiled:
		result = {}
		_update_marks()
		_render_inspector()
		# 코드가 컴파일되지 않아도 레벨의 처음 모습은 보여 준다.
		if r.has("preview"):
			view.setup(r.preview.map, r.preview.frames)
			_sync_timeline()
		_log_compiler(r.output, false)
		%Console.scroll_to_line(0)
		for d in r.diagnostics:
			if d.level == "error":
				_first_error = d
				break
		var errors: int = r.diagnostics.filter(func(d): return d.level == "error").size()
		var where := "아래 출력 창의 rustc 메시지를 읽어 보세요"
		if not _first_error.is_empty():
			where = "%s:%d · %s" % [String(_first_error.file).get_file(), _first_error.line, String(_first_error.message).split("\n")[0]]
		phase = "failed"
		_show_tab("problems")
		_status("error", "컴파일 에러 %d개" % maxi(errors, 1), where, "error" if not _first_error.is_empty() else "")
		if autoplay:
			Sfx.play("fail")
		return
	if r.frames.is_empty() or r.map == null:
		# 첫 모습도 나오기 전에 멈췄다 (시작하자마자 패닉, 시간 초과 등).
		result = {}
		_update_marks()
		_render_inspector()
		if r.has("preview"):
			view.setup(r.preview.map, r.preview.frames)
		for log in r.logs:
			_log(log.text, "bad" if log.kind == "stderr" else "")
		_set_phase("failed", r.message)
		_sync_timeline()
		return
	result = r
	_show_tab("output")
	view.setup(r.map, r.frames)
	_update_marks()
	_shown_tick = -1
	if autoplay:
		playing = true
		%Play.text = "일시정지"
		_set_phase("ready")
	else:
		playing = false
		%Play.text = "재생"
		_set_phase("idle")
	_sync_timeline()


func _finish_run() -> void:
	if phase != "ready":
		return
	if not result.won:
		_set_phase("failed", result.message)
		Sfx.play("fail")
		_render_console(view.last_tick())
		return
	var stats: Dictionary = result.stats
	var unit: String = pack.get("tick", "틱")
	var rec: Dictionary = JsonUtil.parse(core.record_clear(root, packs_dir, pack.id, lesson().id, stats.get("ticks", 0), stats.get("lines", 0)))
	progress = JsonUtil.parse(core.progress(root)).get(pack.id, {})
	var note := ""
	if rec.get("better", false) and rec.get("previous") != null:
		note = " · **최고 기록 경신!** (전 %d%s)" % [rec.previous.ticks, unit]
	elif not rec.get("better", false):
		var best: Dictionary = progress.get(lesson().id, {}).get("best", {})
		note = " · 최고 %d%s, %d줄" % [best.get("ticks", 0), unit, best.get("lines", 0)]
	var detail := note.replace("**", "").trim_prefix(" · ")
	var commit = rec.get("commit")
	if commit is Dictionary and commit.get("ok", false) and commit.get("hash", "") != "":
		detail += ("" if detail == "" else " · ") + "git %s 에 커밋" % commit.hash
	phase = "won"
	var last: bool = level + 1 >= pack.levels.size()
	_status("won", "성공! %d%s · 코드 %d줄" % [stats.get("ticks", 0), unit, stats.get("lines", 0)], detail, "" if last else "next")
	_render_lesson()
	_render_workspace()
	_render_console(view.last_tick())
	view.celebrate()
	Sfx.play("win")


## phase 를 바꾸고 상태 바에 보여 준다. (테스트가 phase 를 본다: idle | building | ready | won | failed)
func _set_phase(p: String, message := "") -> void:
	phase = p
	match p:
		"failed":
			_status("failed", "실패", message, "replay" if not result.is_empty() else "")
		"won":
			_status("won", message)
		"building":
			_status("building", "cargo build 중…", "끝나면 바로 재생해요" if autoplay else "처음 모습을 준비해요")
		"ready":
			_status("playing", _playing_title(), "결과는 타임라인 끝에서 나와요")
		_:
			if result.is_empty():
				_status("idle", "저장을 기다리는 중", "IDE 에서 저장하면 바로 빌드하고 실행해요", "ide")
			else:
				_status("ready", "준비됐어요", "실행(F5)하면 재생해요 · IDE 에서 저장해도 돼요", "run")


func _playing_title() -> String:
	if not view:
		return "재생 중"
	return "재생 중 · %s %d / %d" % [pack.get("tick", "틱"), int(floor(view.time + 0.0001)), view.last_tick()]


func _goal_text() -> String:
	for line in String(lesson().get("lesson", "")).split("\n"):
		if line.begins_with("목표:"):
			return line
	return ""


func _toggle_solution() -> void:
	if not confirm_solution and not show_solution:
		confirm_solution = true
		%Solution.text = "정말 볼까요?"
		return
	confirm_solution = false
	show_solution = not show_solution
	%Solution.text = "정답 숨기기" if show_solution else "정답 보기"
	_render_lesson()
	if show_solution:
		await get_tree().process_frame
		%LessonScroll.scroll_vertical = int(%LessonBody.size.y)


## 힌트는 레슨 안에 펼쳐 둔다 (출력 창처럼 지워지지 않게).
func _toggle_hint() -> void:
	%HintCard.visible = not %HintCard.visible
	%Hint.text = "힌트 숨기기" if %HintCard.visible else "힌트"
	if %HintCard.visible:
		%HintText.markdown = String(lesson().get("hint", "")).strip_edges()


func _reset_level() -> void:
	var r: Dictionary = JsonUtil.parse(core.reset_level(root, packs_dir, pack.id, lesson().id))
	if r.get("ok", false):
		_log("%s 을(를) 처음 코드로 되돌렸어요. 고치던 코드는 git 에 커밋해 두었어요." % lesson().file, "dim")
		_render_workspace()
		run(false)
	else:
		_log("되돌리지 못했어요: %s" % r.get("error", ""), "bad")


func _toggle_sound() -> void:
	Sfx.enabled = not Sfx.enabled
	%Sound.text = "소리" if Sfx.enabled else "음소거"


# ── 타임라인 ─────────────────────────────────────────────────────

func _toggle_play() -> void:
	if result.is_empty():
		return
	if not playing and view.time >= view.last_tick():
		view.time = 0.0
		_shown_tick = -1
	playing = not playing
	%Play.text = "일시정지" if playing else "재생"
	if playing and phase != "ready":
		_set_phase("ready")


func _step_by(n: int) -> void:
	if result.is_empty():
		return
	playing = false
	%Play.text = "재생"
	view.time = clampf(round(view.time) + n, 0.0, view.last_tick())
	_sync_timeline()


func _on_slider(value: float) -> void:
	if _slider_lock or result.is_empty():
		return
	playing = false
	%Play.text = "재생"
	view.time = value
	_sync_timeline()


func _sync_timeline() -> void:
	var has := not result.is_empty()
	for b in [%Back, %Play, %Forward]:
		b.disabled = not has
	for b in %Speeds.get_children():
		b.disabled = not has
	%Slider.editable = has
	if not view:
		return
	_slider_lock = true
	%Slider.max_value = maxf(view.last_tick(), 0.001)
	%Slider.value = view.time
	_slider_lock = false
	var tick := int(floor(view.time + 0.0001))
	%TickLabel.text = "%d / %d" % [tick, view.last_tick()]
	if tick != _shown_tick and phase == "ready":
		%StatusTitle.text = _playing_title()
	if tick != _shown_tick:
		if playing and tick > _shown_tick:
			for t in range(maxi(_shown_tick + 1, 1), tick + 1):
				view.play_effects(t)
		_shown_tick = tick
		_render_console(tick)
		_render_inspector()


# ── 출력 창과 행위자 살펴보기 ─────────────────────────────────────

var TONES := {
	"bad": Palette.hex(Palette.RED), "warn": Palette.hex(Palette.WARN), "good": Palette.hex(Palette.GREEN),
	"dim": Palette.hex(Palette.TEXT3), "": Palette.hex(Palette.TEXT),
}


static func _escape(s: String) -> String:
	return s.replace("[", "[lb]")


func _log(text: String, tone := "") -> void:
	%Console.append_text("[color=%s]%s[/color]\n" % [TONES.get(tone, TONES[""]), _escape(text)])


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
		_log(line, tone)


func _agent_hex(i: int) -> String:
	var colors: Array = pack.get("colors", [Palette.hex(Palette.ACCENT)])
	return colors[i % colors.size()]


func _agent_tag(tick: int, agent: int) -> String:
	return "[color=%s]t%-3d[/color][color=%s]#%d[/color] " % [Palette.hex(Palette.TEXT3), tick, _agent_hex(agent), agent]


## 지금 틱까지 일어난 출력·사건을 시간 순서로 보여 준다.
## 출력 / 문제 탭
func _show_tab(tab: String) -> void:
	%Console.visible = tab == "output"
	%ProblemsScroll.visible = tab == "problems"
	for pair in [[%TabOutput, tab == "output"], [%TabProblems, tab == "problems"]]:
		pair[0].theme_type_variation = "ChipCurrent" if pair[1] else "Chip"


## 컴파일 에러·경고를 카드로. 누르면 IDE 의 그 줄로 간다.
func _render_problems(diagnostics: Array) -> void:
	for c in %Problems.get_children():
		c.queue_free()
	var errors := diagnostics.filter(func(d): return d.level == "error")
	var shown := errors + diagnostics.filter(func(d): return d.level != "error")
	%TabProblems.text = "문제 %d" % shown.size() if not shown.is_empty() else "문제"
	if shown.is_empty():
		var none := Label.new()
		none.text = "문제가 없어요."
		none.add_theme_color_override("font_color", Palette.TEXT3)
		none.add_theme_font_size_override("font_size", Palette.SIZE_SMALL)
		%Problems.add_child(none)
		return
	for d in shown:
		%Problems.add_child(_problem_card(d))


func _problem_card(d: Dictionary) -> Control:
	var is_error: bool = d.level == "error"
	var card := Button.new()
	card.focus_mode = Control.FOCUS_NONE
	card.alignment = HORIZONTAL_ALIGNMENT_LEFT
	card.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
	var c: Color = Palette.RED if is_error else Palette.WARN
	var sb := UiTheme.box(Palette.EDITOR, 8, 12, 8)
	sb.border_width_left = 3
	sb.border_color = c
	var hover := sb.duplicate()
	hover.bg_color = Palette.PANEL_HI
	card.add_theme_stylebox_override("normal", sb)
	card.add_theme_stylebox_override("hover", hover)
	card.add_theme_stylebox_override("pressed", hover)
	card.custom_minimum_size = Vector2(0, 52)
	var box := VBoxContainer.new()
	box.set_anchors_preset(Control.PRESET_FULL_RECT)
	box.offset_left = 14
	box.offset_top = 6
	box.offset_right = -12
	box.mouse_filter = Control.MOUSE_FILTER_IGNORE
	box.add_theme_constant_override("separation", 2)
	var head := HBoxContainer.new()
	head.mouse_filter = Control.MOUSE_FILTER_IGNORE
	var msg := Label.new()
	var lines := String(d.message).split("\n")
	msg.text = ("error: " if is_error else "warning: ") + lines[0]
	msg.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	msg.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
	msg.add_theme_font_override("font", Palette.code_font())
	msg.add_theme_font_size_override("font_size", Palette.SIZE_CODE)
	msg.add_theme_color_override("font_color", c)
	var where := Label.new()
	where.text = "%s:%d:%d  IDE 에서 열기 →" % [String(d.file).get_file(), d.line, d.col]
	where.add_theme_font_size_override("font_size", Palette.SIZE_SMALL)
	where.add_theme_color_override("font_color", Palette.SKY)
	head.add_child(msg)
	head.add_child(where)
	box.add_child(head)
	if lines.size() > 1:
		var more := Label.new()
		more.text = lines[1].strip_edges()
		more.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
		more.add_theme_font_override("font", Palette.code_font())
		more.add_theme_font_size_override("font_size", Palette.SIZE_SMALL)
		more.add_theme_color_override("font_color", Palette.TEXT2)
		box.add_child(more)
	card.add_child(box)
	card.tooltip_text = String(d.message)
	card.pressed.connect(func(): _open_ide(d))
	return card


func _render_console(tick: int) -> void:
	if result.is_empty():
		return
	%Console.clear()
	if result.output != "":
		_log_compiler(result.output, true)
	var entries: Array = []
	var at_end: bool = tick >= view.last_tick()
	for log in result.logs:
		if log.tick < tick or (at_end and log.kind == "stderr"):
			entries.append([log.tick, 0, log.agent, log.text, "bad" if log.kind == "stderr" else ""])
	for t in range(1, tick + 1):
		for ev in view.frame_at(t).get("events", []):
			entries.append([t - 1, 1, ev.agent, ev.text, "dim"])
	entries.sort_custom(func(a, b): return a[0] < b[0] or (a[0] == b[0] and a[1] < b[1]))
	for e in entries:
		var color: String = TONES.get(e[4], TONES[""])
		if e[3].begins_with("배달 완료"):
			color = TONES.good
		elif e[3].begins_with("밀물") or e[3].begins_with("갈매기"):
			color = TONES.warn
		%Console.append_text(_agent_tag(e[0], e[2]) + "[color=%s]%s[/color]\n" % [color, _escape(e[3])])
	if at_end and phase in ["won", "failed"]:
		_log(result.message, "good" if result.won else "bad")


func _render_inspector() -> void:
	if result.is_empty() or not view:
		%InspectorText.text = "[color=%s]실행이 끝나면 %s를 눌러 그 틱의 상태와 행동을 볼 수 있어요.[/color]" % [Palette.hex(Palette.TEXT2), pack.get("agent", "행위자")]
		return
	var i: int = view.selected
	var tick := int(floor(view.time + 0.0001))
	if i >= view.agent_count():
		return
	var lines := PackedStringArray()
	var where: String = view.describe_agent(i, tick)
	var dim := Palette.hex(Palette.TEXT2)
	lines.append("[color=%s]%s #%d[/color]  [color=%s]%s %d · %s[/color]" % [_agent_hex(i), pack.get("agent", ""), i, dim, pack.get("tick", "틱"), tick, where])
	var acts: Array = result.actions[tick] if tick < result.actions.size() else []
	var states: Array = result.states[tick] if tick < result.states.size() else []
	if i < acts.size() and acts[i] != null:
		lines.append("[color=%s]tick() →[/color] [color=%s]%s[/color]" % [dim, Palette.hex(Palette.CODE), _escape(acts[i])])
	if i < states.size() and states[i] != null:
		lines.append("[color=%s]self   =[/color] [color=%s]%s[/color]" % [dim, Palette.hex(Palette.SYN_TYPE), _escape(states[i])])
	elif tick >= view.last_tick():
		lines.append("[color=%s]시뮬레이션 끝[/color]" % dim)
	%InspectorText.text = "\n".join(lines)


## 타임라인 슬라이더 위에 사건 틱을 그리는 막대들.
class TimelineMarks extends Control:
	var ticks: Array = []
	var last := 0
	var outcome := ""

	func _init() -> void:
		mouse_filter = Control.MOUSE_FILTER_IGNORE
		set_anchors_preset(Control.PRESET_FULL_RECT)

	func _x(t: int) -> float:
		# 슬라이더 손잡이 반쪽만큼 양끝을 비운다.
		var inset := 8.0
		return inset + (size.x - inset * 2.0) * (float(t) / maxf(last, 1))

	func _draw() -> void:
		if last <= 0:
			return
		var h := size.y
		for t in ticks:
			draw_rect(Rect2(round(_x(t)) - 1.0, h * 0.5 - 7.0, 2.0, 14.0), Color(Palette.WARN, 0.85))
		if outcome != "":
			var c := Palette.GREEN if outcome == "won" else Palette.RED
			draw_rect(Rect2(round(_x(last)) - 1.0, h * 0.5 - 9.0, 3.0, 18.0), c)
