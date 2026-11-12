## 꽃게 AI 모드(조수 웅덩이) 화면: 레슨, 실행, 타임라인 재생, 꽃게 살펴보기, 기록.
## 컴파일·실행·시뮬레이션은 Rust 확장(CrabCore)이 맡는다.
extends Control

const SAVE_PATH := "user://pool_save.json"
const TICKS_PER_SECOND := 5.0
const SPEEDS := [1.0, 2.0, 4.0, 0.5]

# [이름, 넣을 글, 시그니처, 설명]
const API := [
	["here", "here()", "fn here(&self) -> Tile", "발밑 칸: Sand, Flat, Food, Nest 중 하나."],
	["look", "look(Dir::", "fn look(&self, dir: Dir) -> Tile", "그쪽 바로 옆 칸."],
	["carrying", "carrying()", "fn carrying(&self) -> bool", "먹이를 들고 있으면 true."],
	["food_dir", "food_dir()", "fn food_dir(&self) -> Option<Dir>", "가장 가까운 먹이로 가는 첫걸음. 냄새가 닿지 않으면 None."],
	["nest_dir", "nest_dir()", "fn nest_dir(&self) -> Option<Dir>", "굴로 가는 첫걸음. 이미 굴 위라면 None."],
	["tide", "tide()", "fn tide(&self) -> Tide", "지금 물때: Low { turns_left } / High { turns_left } / Calm."],
	["danger", "danger(", "fn danger(&self, dir: Dir) -> bool", "그쪽으로 가면 다음 틱에 갈매기에게 잡히면 true."],
	["tick", "tick()", "fn tick(&self) -> u32", "View::tick: 지금 몇 번째 틱인지 (0부터). Crab::tick 은 게임이 매 틱 부르는 함수예요."],
	["id", "id()", "fn id(&self) -> usize", "이 꽃게의 번호 (0부터)."],
	["pos", "pos()", "fn pos(&self) -> (i32, i32)", "지금 위치 (x, y). 왼쪽 위가 (0, 0)."],
	["delivered", "delivered()", "fn delivered(&self) -> u32", "지금까지 굴에 배달한 먹이 수 (꽃게 모두 합쳐서)."],
	["goal", "goal()", "fn goal(&self) -> u32", "목표 배달 수."],
	["is_open", "is_open()", "fn is_open(self) -> bool", "Tile: 지금 걸어 들어갈 수 있는 칸이면 true."],
	["turn_right", "turn_right()", "fn turn_right(self) -> Dir", "Dir: 시계 방향으로 90도."],
	["turn_left", "turn_left()", "fn turn_left(self) -> Dir", "Dir: 반시계 방향으로 90도."],
	["opposite", "opposite()", "fn opposite(self) -> Dir", "Dir: 반대쪽."],
]

# [이름, 시그니처, 설명]
const TYPES := [
	["Crab", "trait Crab: Debug {\n    fn new(id: usize) -> Self;\n    fn tick(&mut self, view: &View) -> Action;\n}", "꽃게의 두뇌. 게임이 꽃게마다 new 로 만들고, 매 틱 tick 을 부른다."],
	["Ferris", "struct Ferris", "내가 만드는 꽃게. 게임이 Ferris::new(번호) 로 꽃게 수만큼 만든다."],
	["View", "struct View", "꽃게가 한 틱에 보는 것. tick 이 &View 로 빌려 받는다."],
	["Action", "enum Action { Wait, Move(Dir), Pick, Drop }", "꽃게가 한 틱에 할 수 있는 일."],
	["Dir", "enum Dir { North, East, South, West }", "방향. 꽃게는 몸을 돌리지 않고 어느 쪽으로든 옆걸음질한다."],
	["Tile", "enum Tile { Sand, Rock, Water, Flat, Food, Nest, Gull }", "꽃게가 보는 칸."],
	["Tide", "enum Tide { Low { turns_left: u32 }, High { turns_left: u32 }, Calm }", "물때."],
]

const VARIANTS := [
	"Action::Wait", "Action::Move(", "Action::Pick", "Action::Drop",
	"Dir::North", "Dir::East", "Dir::South", "Dir::West",
	"Tile::Sand", "Tile::Rock", "Tile::Water", "Tile::Flat", "Tile::Food", "Tile::Nest", "Tile::Gull",
	"Tide::Low { turns_left }", "Tide::High { turns_left }", "Tide::Calm",
]

const CRAB_HEX := ["#f4511e", "#3fb6a8", "#e2b33c", "#b07cf0"]

@onready var code: RustEditor = %Code
@onready var ref_code: RustEditor = %RefCode
@onready var pool: PoolView = %Pool
@onready var console: RichTextLabel = %Console

var core := CrabCore.new()
var lessons: Array = []
var level := 0
var codes := {}
var cleared := {}
var best := {} # id -> {ticks, lines}
var dev := false
var phase := "idle" # idle | compiling | ready | won | failed
var confirm_solution := false

var result: Dictionary = {} # 마지막 실행 결과
var playing := false
var speed_idx := 0
var _shown_tick := -1
var _slider_lock := false
var _compiler_note := ""

# crab.rs 에서 이름이 정의된 줄: 이름 -> [{line, kind, owner}]
var _defs := {}
# 정의로 가기 전 자리들: [탭, 줄, 칸]
var _nav_history: Array = []
var _tab := "ferris"

var _run_token := -1
var _run_level := -1
var _check_token := -1
var _check_version := ""


func _ready() -> void:
	theme = UiTheme.build()
	_style_static_nodes()
	lessons = JsonUtil.parse(core.pool_lessons()).lessons
	code.set_vocabulary(API, TYPES, VARIANTS)
	_setup_reference()

	var start_env := OS.get_environment("CRAB_POOL_START")
	var demo_env := OS.get_environment("CRAB_DEMO")
	dev = start_env != "" or demo_env != ""
	if not dev:
		_load_save()

	%Run.pressed.connect(run)
	%Reset.pressed.connect(reset_code)
	%Hint.pressed.connect(func(): _log("힌트: " + lesson().hint.replace("`", ""), "warn"))
	%Solution.pressed.connect(show_solution)
	%Sound.pressed.connect(_toggle_sound)
	%Mode.pressed.connect(_switch_mode)
	%Next.pressed.connect(func(): set_level(level + 1))
	%Play.pressed.connect(_toggle_play)
	%Back.pressed.connect(func(): _step_by(-1))
	%Forward.pressed.connect(func(): _step_by(1))
	%Speed.pressed.connect(func():
		speed_idx = (speed_idx + 1) % SPEEDS.size()
		%Speed.text = ("%sx" % SPEEDS[speed_idx]).replace(".0x", "x"))
	%Slider.value_changed.connect(_on_slider)
	pool.crab_selected.connect(func(_i): _render_inspector())
	code.format_requested.connect(_format_code)
	code.idle_edited.connect(func():
		_save_code()
		_check_code())
	code.text_changed.connect(func():
		if confirm_solution:
			confirm_solution = false
			%Solution.text = "정답 보기")

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
	for tab in [%TabFerris, %TabCrab]:
		tab.add_theme_font_override("font", Palette.code_font())
		tab.add_theme_font_size_override("font_size", 13)
		tab.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
	%TabHint.add_theme_color_override("font_color", Palette.FAINT)
	%TabHint.add_theme_font_size_override("font_size", 12)
	%Run.theme_type_variation = "PrimaryButton"
	%Mode.add_theme_font_size_override("font_size", 13)
	%TickLabel.add_theme_font_override("font", Palette.code_font())
	%TickLabel.add_theme_font_size_override("font_size", 13)
	%TickLabel.add_theme_color_override("font_color", Palette.DIM)
	%ConsoleLabel.add_theme_color_override("font_color", Palette.DIM)
	%ConsoleLabel.add_theme_font_size_override("font_size", 12)
	console.add_theme_font_override("normal_font", Palette.code_font())
	console.add_theme_font_size_override("normal_font_size", 13)
	console.add_theme_constant_override("line_separation", 3)
	%InspectorText.add_theme_font_override("normal_font", Palette.code_font())
	%InspectorText.add_theme_font_size_override("normal_font_size", 13)
	%Inspector.add_theme_stylebox_override("panel", UiTheme.box(Palette.EDITOR, 8, 12, 8))
	%TabBar.add_theme_stylebox_override("panel", UiTheme.box(Palette.CHROME, 0, 8, 0))
	for b in [%Back, %Play, %Forward, %Speed]:
		b.add_theme_font_size_override("font_size", 13)
		for state in ["normal", "hover", "pressed", "disabled"]:
			var sb: StyleBoxFlat = b.get_theme_stylebox(state).duplicate()
			sb.content_margin_left = 8
			sb.content_margin_right = 8
			sb.content_margin_top = 4
			sb.content_margin_bottom = 4
			b.add_theme_stylebox_override(state, sb)


func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo and event.keycode == KEY_F5:
		get_viewport().set_input_as_handled()
		run()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		_save_code()
		core.cleanup()


# ── 저장 ─────────────────────────────────────────────────────────

func lesson() -> Dictionary:
	return lessons[level]


func _load_save() -> void:
	var data = JSON.parse_string(FileAccess.get_file_as_string(SAVE_PATH))
	if data is Dictionary:
		codes = data.get("codes", {})
		for id in data.get("cleared", []):
			cleared[id] = true
		best = data.get("best", {})


func _write_save() -> void:
	if dev:
		return
	var f := FileAccess.open(SAVE_PATH, FileAccess.WRITE)
	if f:
		f.store_string(JSON.stringify({"codes": codes, "cleared": cleared.keys(), "best": best}, "\t"))


func _save_code() -> void:
	if lessons.is_empty():
		return
	codes[lesson().id] = code.text
	_write_save()


func _check_code() -> void:
	_check_version = code.text
	_check_token = core.start_pool_check(code.text)


func _switch_mode() -> void:
	_save_code()
	get_tree().change_scene_to_file("res://main.tscn")


# ── crab.rs 탭과 정의로 가기 ────────────────────────────────────────

func _setup_reference() -> void:
	ref_code.set_vocabulary(API, TYPES, VARIANTS)
	ref_code.text = core.pool_api()
	ref_code.clear_undo_history()
	_index_definitions(ref_code.text)
	for ed in [code, ref_code]:
		ed.definition_check = func(word: String) -> bool: return _defs.has(word)
		ed.definition_requested.connect(_go_to_definition)
		ed.go_back_requested.connect(_go_back)
	%TabFerris.pressed.connect(func(): _show_tab("ferris"))
	%TabCrab.pressed.connect(func(): _show_tab("crab"))
	%LessonBody.code_link = func(t: String) -> bool: return not _resolve_code(t).is_empty()
	%LessonBody.code_link_clicked.connect(func(t: String):
		var hit := _resolve_code(t)
		if not hit.is_empty():
			_go_to_definition(hit.word, hit.after_dot))


## crab.rs 를 훑어 이름마다 정의된 줄을 모은다.
func _index_definitions(src: String) -> void:
	_defs.clear()
	var item := RegEx.create_from_string("^pub (trait|struct|enum) (\\w+)")
	var impl_re := RegEx.create_from_string("^impl (\\w+)")
	var fn_re := RegEx.create_from_string("^\\s*(?:pub )?(?:const )?fn (\\w+)|^\\s*pub const (\\w+)")
	var variant_re := RegEx.create_from_string("^    ([A-Z]\\w*)\\b")
	var owner := ""
	var kind := ""
	var lines := src.split("\n")
	for i in lines.size():
		var line: String = lines[i]
		var m := item.search(line)
		if m:
			owner = m.get_string(2)
			kind = m.get_string(1)
			_add_def(owner, i, kind, "")
			continue
		m = impl_re.search(line)
		if m:
			owner = m.get_string(1)
			kind = "impl"
			continue
		if line.begins_with("}"):
			owner = ""
			kind = ""
			continue
		m = fn_re.search(line)
		if m:
			var name := m.get_string(1) if m.get_string(1) != "" else m.get_string(2)
			_add_def(name, i, "trait_fn" if kind == "trait" else ("fn" if owner == "" else "method"), owner)
			continue
		if kind == "enum":
			m = variant_re.search(line)
			if m:
				_add_def(m.get_string(1), i, "variant", owner)
				_add_def(owner + "::" + m.get_string(1), i, "variant", owner)


func _add_def(name: String, line: int, kind: String, owner: String) -> void:
	if not _defs.has(name):
		_defs[name] = []
	_defs[name].append({"line": line, "kind": kind, "owner": owner})


## 이름의 정의 줄. 점 뒤의 이름(view.tick)이면 메서드를, 아니면 타입·트레이트 쪽을 먼저 고른다.
func _find_def(word: String, after_dot: bool) -> int:
	var list: Array = _defs.get(word, [])
	if list.is_empty():
		return -1
	var prefer := ["method", "trait_fn", "fn"] if after_dot else ["trait", "struct", "enum", "trait_fn", "variant", "method", "fn"]
	for k in prefer:
		for d in list:
			if d.kind == k:
				return d.line
	return list[0].line


## 레슨 본문의 인라인 코드(`Tide`, `view.food_dir()`, `Action::Move(dir)` …)가 가리키는 이름.
func _resolve_code(t: String) -> Dictionary:
	var words := RegEx.create_from_string("[A-Za-z_]\\w*").search_all(t)
	if words.is_empty():
		return {}
	# Tide::Low 처럼 경로면 변형을, view.x() 면 메서드를, 아니면 첫 이름을 본다.
	var path := RegEx.create_from_string("([A-Z]\\w*)::([A-Z]\\w*)").search(t)
	if path and _defs.has(path.get_string(1) + "::" + path.get_string(2)):
		return {"word": path.get_string(1) + "::" + path.get_string(2), "after_dot": false}
	var method := RegEx.create_from_string("\\.(\\w+)\\s*\\(").search(t)
	if method and _defs.has(method.get_string(1)):
		return {"word": method.get_string(1), "after_dot": true}
	var first := words[0].get_string()
	if _defs.has(first) and first[0] == first[0].to_upper():
		return {"word": first, "after_dot": false}
	return {}


func _show_tab(tab: String) -> void:
	_tab = tab
	code.visible = tab == "ferris"
	ref_code.visible = tab == "crab"
	for pair in [[%TabFerris, tab == "ferris"], [%TabCrab, tab == "crab"]]:
		var b: Button = pair[0]
		var active: bool = pair[1]
		var sb := UiTheme.box(Palette.EDITOR if active else Color(0, 0, 0, 0), 0, 14, 7)
		sb.corner_radius_top_left = 6
		sb.corner_radius_top_right = 6
		if active:
			sb.border_width_top = 2
			sb.border_color = Palette.ACCENT
		for state in ["normal", "hover", "pressed", "focus"]:
			b.add_theme_stylebox_override(state, sb if state != "hover" or active else UiTheme.box(Color(1, 1, 1, 0.04), 6, 14, 7))
		var fg := Palette.TEXT if active else Palette.DIM
		for c in ["font_color", "font_hover_color", "font_pressed_color", "font_focus_color"]:
			b.add_theme_color_override(c, fg)
	(code if tab == "ferris" else ref_code).grab_focus()


func _current_editor() -> RustEditor:
	return code if _tab == "ferris" else ref_code


func _go_to_definition(word: String, after_dot: bool) -> void:
	var line := _find_def(word, after_dot)
	if line < 0:
		return
	var ed := _current_editor()
	_nav_history.append([_tab, ed.get_caret_line(), ed.get_caret_column()])
	_show_tab("crab")
	ref_code.reveal_word(line, word.get_slice("::", word.get_slice_count("::") - 1))


func _go_back() -> void:
	if _nav_history.is_empty():
		if _tab == "crab":
			_show_tab("ferris")
		return
	var spot: Array = _nav_history.pop_back()
	_show_tab(spot[0])
	var ed := _current_editor()
	ed.remove_secondary_carets()
	ed.deselect()
	ed.set_caret_line(spot[1])
	ed.set_caret_column(spot[2])
	ed.center_viewport_to_caret()


# ── 레벨 ─────────────────────────────────────────────────────────

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
	_nav_history.clear()
	_show_tab("ferris")
	_show_preview()
	_render_lesson()
	code.set_caret_line(0)
	code.set_caret_column(0)
	code.grab_focus()
	_check_code()


func _show_preview() -> void:
	var data: Dictionary = JsonUtil.parse(core.pool_level(level, randi() + 1))
	result = {}
	playing = false
	pool.setup(data.map, [data.frame])
	_compiler_note = ""
	_sync_timeline()
	_set_phase("idle")
	_clear_console()
	_log("실행(F5)하면 꽃게마다 Ferris::new(번호) 를 만들고, 매 틱 tick(&view) 을 불러요.", "dim")
	_log("실행이 끝나면 타임라인으로 아무 틱이나 돌려 볼 수 있어요. 꽃게를 누르면 그 순간의 상태가 보여요.", "dim")
	_render_inspector()


func _render_lesson() -> void:
	var l := lesson()
	for c in %Levels.get_children():
		c.queue_free()
	for i in lessons.size():
		var b := Button.new()
		b.text = str(i + 1)
		b.theme_type_variation = "ChipCurrent" if i == level else ("ChipDone" if cleared.has(lessons[i].id) else "Chip")
		b.focus_mode = Control.FOCUS_NONE
		b.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
		b.tooltip_text = lessons[i].title
		b.pressed.connect(func():
			if i != level:
				set_level(i))
		%Levels.add_child(b)
	%Progress.text = "클리어 %d / %d" % [lessons.filter(func(x): return cleared.has(x.id)).size(), lessons.size()]
	var record := ""
	if best.has(l.id):
		record = " · 최고 기록 %d틱, %d줄" % [best[l.id].ticks, best[l.id].lines]
	%ChapterName.text = "조수 웅덩이 · 웅덩이 %d / %d%s" % [level + 1, lessons.size(), record]
	%LessonTitle.text = "%d. %s" % [level + 1, l.title]
	%LessonBody.markdown = l.body
	%LessonScroll.scroll_vertical = 0


func _goal_text() -> String:
	for line in lesson().body.split("\n"):
		if line.begins_with("목표:"):
			return line
	return ""


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
			text = message
			%Next.visible = level + 1 < lessons.size()
		"compiling":
			color = Palette.DIM
			text = "`rustc` 가 꽃게의 두뇌를 컴파일하는 중..."
		"ready":
			color = Palette.DIM
			text = "재생 중이에요. 결과는 타임라인 끝에서 나와요."
		_:
			text = _goal_text()
	%Banner.add_theme_stylebox_override("panel", UiTheme.box(bg, 10, 14, 10))
	%BannerText.base_color = color
	%BannerText.markdown = text


# ── 실행 ─────────────────────────────────────────────────────────

func run() -> void:
	if phase == "compiling" or lessons.is_empty():
		return
	_save_code()
	playing = false
	_set_phase("compiling")
	_clear_console()
	_log("컴파일 중...", "dim")
	Sfx.play("compile")
	_check_token = -1
	_run_level = level
	_run_token = core.start_pool_run(code.text, level, randi() + 1)


func _process(delta: float) -> void:
	var out := core.poll_run()
	if out != "":
		_on_run_result(JsonUtil.parse(out))
	out = core.poll_check()
	if out != "":
		var r: Dictionary = JsonUtil.parse(out)
		if r.token == _check_token and code.text == _check_version and phase != "compiling":
			code.set_diagnostics(r.diagnostics)

	if playing and not result.is_empty():
		var end := float(pool.last_tick())
		pool.time = minf(pool.time + delta * TICKS_PER_SECOND * SPEEDS[speed_idx], end)
		_sync_timeline()
		if pool.time >= end:
			playing = false
			%Play.text = "다시"
			_finish_run()


func _on_run_result(r: Dictionary) -> void:
	if r.token != _run_token or _run_level != level:
		return
	code.set_diagnostics(r.diagnostics)
	_clear_console()
	if not r.compiled:
		result = {}
		_log_compiler(r.output, false)
		console.scroll_to_line(0)
		var line := code.first_error_line()
		if line >= 0:
			code.set_caret_line(line)
			code.center_viewport_to_caret()
		_set_phase("failed", "컴파일 에러! 아래 컴파일러 메시지를 읽어 보세요.")
		Sfx.play("fail")
		return
	result = r
	_compiler_note = r.output
	pool.setup(r.map, r.frames)
	_shown_tick = -1
	playing = true
	%Play.text = "일시정지"
	_set_phase("ready")
	_sync_timeline()


func _finish_run() -> void:
	if phase != "ready":
		return
	var stats: Dictionary = result.stats
	if not result.won:
		_set_phase("failed", result.message)
		Sfx.play("fail")
		return
	var l := lesson()
	var note := ""
	var prev: Dictionary = best.get(l.id, {})
	if prev.is_empty() or stats.ticks < prev.ticks or (stats.ticks == prev.ticks and stats.lines < prev.lines):
		if not prev.is_empty():
			note = " · **최고 기록 경신!** (전 %d틱)" % prev.ticks
		best[l.id] = {"ticks": stats.ticks, "lines": stats.lines}
	else:
		note = " · 최고 %d틱, %d줄" % [prev.ticks, prev.lines]
	cleared[l.id] = true
	_write_save()
	var mishaps := ""
	if stats.swept + stats.caught > 0:
		mishaps = " · 휩쓸림 %d · 잡힘 %d" % [stats.swept, stats.caught]
	_set_phase("won", "성공! **%d틱** · 코드 **%d줄**%s%s" % [stats.ticks, stats.lines, mishaps, note])
	_render_lesson()
	pool.celebrate()
	Sfx.play("win")


func reset_code() -> void:
	code.replace_all(lesson().start)
	_show_preview()


func show_solution() -> void:
	if not confirm_solution:
		confirm_solution = true
		%Solution.text = "정말 볼까요?"
		return
	code.replace_all(lesson().solution)


## Shift+Alt+F: rustfmt 로 코드를 정리한다.
func _format_code() -> void:
	var r: Dictionary = JsonUtil.parse(core.format(code.text))
	if r.ok:
		if r.code != code.text:
			code.replace_all(r.code, true)
		return
	_log("rustfmt 가 코드를 정리하지 못했어요. 문법 에러부터 고쳐 보세요.", "warn")


func _toggle_sound() -> void:
	Sfx.enabled = not Sfx.enabled
	%Sound.text = "소리 켜짐" if Sfx.enabled else "소리 꺼짐"


# ── 타임라인 ─────────────────────────────────────────────────────

func _toggle_play() -> void:
	if result.is_empty():
		return
	if not playing and pool.time >= pool.last_tick():
		pool.time = 0.0
		_shown_tick = -1
	playing = not playing
	%Play.text = "일시정지" if playing else "재생"


func _step_by(n: int) -> void:
	if result.is_empty():
		return
	playing = false
	%Play.text = "재생"
	pool.time = clampf(round(pool.time) + n, 0.0, pool.last_tick())
	_sync_timeline()


func _on_slider(value: float) -> void:
	if _slider_lock or result.is_empty():
		return
	playing = false
	%Play.text = "재생"
	pool.time = value
	_sync_timeline()


func _sync_timeline() -> void:
	var has := not result.is_empty()
	for b in [%Back, %Play, %Forward, %Slider, %Speed]:
		if b is BaseButton:
			b.disabled = not has
		else:
			b.editable = has
	_slider_lock = true
	%Slider.max_value = maxf(pool.last_tick(), 0.001)
	%Slider.value = pool.time
	_slider_lock = false
	var tick := int(floor(pool.time + 0.0001))
	%TickLabel.text = "%d / %d" % [tick, pool.last_tick()]
	if tick != _shown_tick:
		# 재생 중에 앞으로 넘어간 틱마다 효과를 낸다.
		if playing and tick > _shown_tick:
			for t in range(maxi(_shown_tick + 1, 1), tick + 1):
				pool.play_effects(t)
		_shown_tick = tick
		_render_console(tick)
		_render_inspector()


# ── 출력 창과 꽃게 살펴보기 ───────────────────────────────────────

const TONES := {"bad": "#ff6b7a", "warn": "#f2c94c", "good": "#5fd38d", "dim": "#717791", "": "#e4e6f0"}


static func _escape(s: String) -> String:
	return s.replace("[", "[lb]")


func _log(text: String, tone := "") -> void:
	console.append_text("[color=%s]%s[/color]\n" % [TONES.get(tone, TONES[""]), _escape(text)])


func _clear_console() -> void:
	console.clear()


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


func _crab_tag(tick: int, crab: int) -> String:
	var hex: String = CRAB_HEX[crab % CRAB_HEX.size()]
	return "[color=#5d627a]t%-3d[/color][color=%s]#%d[/color] " % [tick, hex, crab]


## 지금 틱까지 일어난 출력·사건을 시간 순서로 보여 준다.
func _render_console(tick: int) -> void:
	if result.is_empty():
		return
	console.clear()
	if _compiler_note != "":
		_log_compiler(_compiler_note, true)
	var entries: Array = []
	for log in result.logs:
		# tick t 의 출력은 t 에서 t+1 로 넘어가는 동안 나온 것이다.
		if log.tick < tick or (log.kind == "stderr" and tick >= pool.last_tick()):
			entries.append([log.tick, 0, log.crab, log.text, "bad" if log.kind == "stderr" else ""])
	for t in range(1, tick + 1):
		for ev in pool.frame_at(t).events:
			entries.append([t - 1, 1, ev.crab, ev.text, "dim"])
	entries.sort_custom(func(a, b): return a[0] < b[0] or (a[0] == b[0] and a[1] < b[1]))
	for e in entries:
		var color: String = TONES.get(e[4], TONES[""])
		if e[3].begins_with("배달 완료"):
			color = TONES.good
		elif e[3].begins_with("밀물") or e[3].begins_with("갈매기"):
			color = TONES.warn
		console.append_text(_crab_tag(e[0], e[2]) + "[color=%s]%s[/color]\n" % [color, _escape(e[3])])
	if tick >= pool.last_tick() and phase != "ready":
		var tone := "good" if result.won else "bad"
		_log(result.message, tone)


func _render_inspector() -> void:
	var i := pool.selected
	var hex: String = CRAB_HEX[i % CRAB_HEX.size()]
	if result.is_empty():
		%InspectorText.text = "[color=#8a8fa8]꽃게를 누르면 그 틱의 상태와 행동이 여기 보여요.[/color]"
		return
	var tick := int(floor(pool.time + 0.0001))
	var fr := pool.frame_at(tick)
	if fr.is_empty() or i >= fr.crabs.size():
		return
	var c: Dictionary = fr.crabs[i]
	var lines := PackedStringArray()
	var where := "(%d, %d)%s" % [c.x, c.y, " · 먹이 들고 있음" if c.carrying else ""]
	lines.append("[color=%s]꽃게 #%d[/color]  [color=#8a8fa8]틱 %d · %s[/color]" % [hex, i, tick, where])
	# 다음에 할 일: 이 틱에 tick() 이 고른 행동과, 그 뒤의 상태
	var acts: Array = result.actions[tick] if tick < result.actions.size() else []
	var states: Array = result.states[tick] if tick < result.states.size() else []
	if i < acts.size():
		lines.append("[color=#8a8fa8]tick() →[/color] [color=#ffc66d]%s[/color]" % _escape(acts[i]))
	if i < states.size():
		lines.append("[color=#8a8fa8]self   =[/color] [color=#6fe0c0]%s[/color]" % _escape(states[i]))
	elif tick >= pool.last_tick():
		lines.append("[color=#8a8fa8]시뮬레이션 끝[/color]")
	%InspectorText.text = "\n".join(lines)
