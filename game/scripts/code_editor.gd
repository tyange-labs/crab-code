class_name RustEditor
## Rust 코드 에디터.
## 문법 색칠, 게임 함수 자동 완성, 컴파일 에러 물결 밑줄, 마우스를 올리면 나오는 설명을 맡는다.
extends CodeEdit

## 입력이 잠깐 멈췄을 때 (저장하고 컴파일 에러를 확인할 때).
signal idle_edited
## Shift+Alt+F: rustfmt 로 문서를 정리해 달라는 요청.
signal format_requested

# [이름, 넣을 글, 시그니처, 설명]
const API := [
	["move_forward", "move_forward()", "fn move_forward()", "앞으로 한 칸 이동합니다. 벽이나 잠긴 문에 부딪히면 프로그램이 멈춥니다."],
	["turn_left", "turn_left()", "fn turn_left()", "제자리에서 왼쪽으로 90도 돕니다."],
	["turn_right", "turn_right()", "fn turn_right()", "제자리에서 오른쪽으로 90도 돕니다."],
	["collect", "collect()", "fn collect() -> Gem", "발밑의 보석을 줍습니다. 주운 보석의 소유권을 돌려줍니다."],
	["deposit", "deposit(", "fn deposit(gem: Gem)", "발밑 받침대에 보석을 올립니다. 보석의 소유권을 가져갑니다."],
	["color_of", "color_of(&", "fn color_of(gem: &Gem) -> Color", "보석의 색을 확인합니다. 빌려서 보기만 합니다."],
	["polish", "polish(&mut ", "fn polish(gem: &mut Gem)", "보석에 광을 냅니다. 보석을 바꾸므로 가변 레퍼런스가 필요합니다."],
	["inspect", "inspect(&", "fn inspect(gem: &Gem) -> GemInfo", "보석을 살펴봅니다. GemInfo { color, weight, shiny } 를 돌려줍니다."],
	["unlock", "unlock(", "fn unlock(code: u32)", "바로 앞의 문에 비밀번호를 입력합니다."],
	["front_is_clear", "front_is_clear()", "fn front_is_clear() -> bool", "앞 칸으로 갈 수 있으면 true."],
	["on_gem", "on_gem()", "fn on_gem() -> bool", "발밑에 보석이 있으면 true."],
	["read_sign", "read_sign()", "fn read_sign() -> Sign", "발밑의 표지판을 읽습니다. 표지판이 없으면 Sign::Straight."],
	["read_number", "read_number()", "fn read_number() -> Option<u32>", "발밑의 숫자 타일을 읽습니다. 숫자가 없으면 None."],
	["read_dial", "read_dial()", "fn read_dial() -> u32", "다이얼에 적힌 숫자를 읽습니다."],
	["read_scroll", "read_scroll()", "fn read_scroll() -> Vec<u32>", "두루마리에 적힌 숫자 목록을 읽습니다."],
	["read_note", "read_note()", "fn read_note() -> String", "쪽지에 적힌 글을 읽습니다."],
	["read_two_notes", "read_two_notes()", "fn read_two_notes() -> (String, String)", "쪽지 두 장을 한꺼번에 읽습니다."],
	["gem_offset", "gem_offset()", "fn gem_offset() -> (u32, u32)", "보석까지의 거리: (앞으로 몇 칸, 오른쪽으로 몇 칸)."],
]

# [이름, 시그니처, 설명]
const TYPES := [
	["Color", "enum Color { Red, Blue }", "보석의 색."],
	["Sign", "enum Sign { Straight, Left, Right, Finish }", "바닥 표지판."],
	["Gem", "struct Gem", "보석. Copy 도 Clone 도 아니라서 주인은 언제나 하나뿐입니다."],
	["GemInfo", "struct GemInfo { color: Color, weight: u32, shiny: bool }", "inspect 가 돌려주는 보석 정보."],
]

const VARIANTS := ["Color::Red", "Color::Blue", "Sign::Straight", "Sign::Left", "Sign::Right", "Sign::Finish"]

const KEYWORDS := [
	"as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for", "if", "impl", "in",
	"let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
	"super", "trait", "type", "unsafe", "use", "where", "while", "dyn", "true", "false",
]
const PRIMITIVES := [
	"i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64",
	"bool", "char", "str", "String", "Vec", "Option", "Result", "Some", "None", "Ok", "Err", "Box",
	"HashMap", "HashSet", "Copy", "Clone", "Debug", "PartialEq", "Default", "Add", "From", "Display",
]
const MACROS := ["println!", "print!", "format!", "vec!", "panic!", "assert!", "assert_eq!"]

const COL_KEYWORD := Color("#ff8f5a")
const COL_TYPE := Color("#6fe0c0")
const COL_COMMENT := Color("#717791")
const COL_STRING := Color("#b5e07a")
const COL_NUMBER := Color("#d9a8ff")
const COL_FUNC := Color("#8ec5ff")
const COL_SYMBOL := Color("#b4b9d0")

var _diagnostics: Array = []
var _idle_timer := Timer.new()
var _complete_timer := Timer.new()


func _ready() -> void:
	_setup_look()
	_setup_highlighter()
	code_completion_enabled = true
	code_completion_prefixes = [".", ":"]
	code_completion_requested.connect(_on_completion_requested)

	_idle_timer.one_shot = true
	_idle_timer.wait_time = 0.6
	_idle_timer.timeout.connect(func(): idle_edited.emit())
	add_child(_idle_timer)
	_complete_timer.one_shot = true
	_complete_timer.wait_time = 0.15
	_complete_timer.timeout.connect(_maybe_complete)
	add_child(_complete_timer)
	text_changed.connect(_on_text_changed)
	_setup_overlays()


func _setup_look() -> void:
	add_theme_font_override("font", Palette.code_font())
	add_theme_font_size_override("font_size", 14)
	add_theme_constant_override("line_spacing", 7)
	add_theme_stylebox_override("normal", UiTheme.box(Palette.EDITOR, 0, 6, 8))
	add_theme_stylebox_override("focus", StyleBoxEmpty.new())
	add_theme_color_override("font_color", Palette.TEXT)
	add_theme_color_override("current_line_color", Color("#24273a"))
	add_theme_color_override("line_number_color", Palette.FAINT)
	add_theme_color_override("caret_color", Palette.TEXT)
	add_theme_color_override("selection_color", Color("#2c4a7c"))
	add_theme_color_override("word_highlighted_color", Color("#3a3f5c"))
	add_theme_color_override("brace_mismatch_color", Palette.RED)
	add_theme_color_override("completion_background_color", Color("#23263b"))
	add_theme_color_override("completion_selected_color", Color("#2f4a7a"))
	add_theme_color_override("completion_existing_color", Color(Palette.ACCENT, 0.25))
	add_theme_color_override("completion_font_color", Palette.DIM)
	gutters_draw_line_numbers = true
	gutters_zero_pad_line_numbers = false
	highlight_current_line = true
	highlight_all_occurrences = true
	draw_tabs = false
	indent_use_spaces = true
	indent_size = 4
	indent_automatic = true
	auto_brace_completion_enabled = true
	auto_brace_completion_highlight_matching = true
	scroll_smooth = true
	caret_blink = true
	minimap_draw = false
	line_folding = true
	gutters_draw_fold_gutter = true
	add_theme_color_override("search_result_color", Color(Palette.WARN, 0.22))
	add_theme_color_override("search_result_border_color", Color(Palette.WARN, 0.6))
	clear_comment_delimiters()
	add_comment_delimiter("//", "", true)
	add_comment_delimiter("/*", "*/", false)
	clear_string_delimiters()
	add_string_delimiter("\"", "\"", false)


func _setup_highlighter() -> void:
	var h := CodeHighlighter.new()
	h.number_color = COL_NUMBER
	h.symbol_color = COL_SYMBOL
	h.function_color = COL_FUNC
	h.member_variable_color = Palette.TEXT
	for k in KEYWORDS:
		h.add_keyword_color(k, COL_KEYWORD)
	for t in PRIMITIVES:
		h.add_keyword_color(t, COL_TYPE)
	for t in TYPES:
		h.add_keyword_color(t[0], COL_TYPE)
	h.add_color_region("//", "", COL_COMMENT, true)
	h.add_color_region("/*", "*/", COL_COMMENT, false)
	h.add_color_region("\"", "\"", COL_STRING, false)
	syntax_highlighter = h


# ── VS Code 기본 단축키 ───────────────────────────────────────────
#
# macOS 는 Cmd, 다른 OS 는 Ctrl (VS Code 와 같다). Godot 가 기본으로 처리하는 키
# (캐럿 이동, 단어 단위 이동·삭제, Cmd+Z/X/C/V, Tab 들여쓰기, 괄호 자동 닫기, Alt+클릭 캐럿 추가 등)
# 는 그대로 두고, 나머지 VS Code 기본 단축키를 여기서 처리한다.
#
#   Cmd+/            줄 주석           Shift+Alt+A       블록 주석
#   Cmd+K Cmd+C/U    주석 달기/풀기      Cmd+K Cmd+0/J     모두 접기/펼치기
#   Alt+↑/↓          줄 옮기기          Shift+Alt+↑/↓     줄 복사
#   Cmd+Shift+K      줄 지우기          Cmd+Enter / Cmd+Shift+Enter  아래/위에 새 줄
#   Cmd+] / Cmd+[    들여쓰기/내어쓰기    Cmd+L             줄 선택
#   Cmd+D            다음 같은 단어 선택   Cmd+Shift+L       같은 단어 모두 선택
#   Alt+Cmd+↑/↓      위/아래에 캐럿 추가  Esc               캐럿 하나로 / 선택 해제
#   Cmd+Shift+\      짝 괄호로 이동      Shift+Alt+→/←     선택 넓히기/좁히기
#   Alt+Cmd+[ / ]    접기/펼치기 (Windows·Linux: Ctrl+Shift+[ / ])
#   Cmd+F 찾기  Alt+Cmd+F (Ctrl+H) 바꾸기  F3·Cmd+G 다음  Shift+F3·Cmd+Shift+G 이전
#   Ctrl+G 줄 이동   F8 / Shift+F8 다음/이전 문제   Ctrl+Space·Cmd+I 자동 완성
#   Cmd+Shift+Space  매개변수 힌트      Shift+Alt+F       문서 정리 (rustfmt)
#   Alt+Z 줄 바꿈 켜기/끄기   Cmd+= / Cmd+- / Cmd+0 글자 크기

const BASE_FONT_SIZE := 14

var _chord := "" # Cmd+K 를 누르고 다음 키를 기다리는 중이면 "k"
var _find_bar: FindBar
var _peek: PanelContainer
var _peek_label: RichTextLabel
var _expand_stack: Array = [] # Shift+Alt+→ 로 넓히기 전 선택들
var _font_size := BASE_FONT_SIZE


func _setup_overlays() -> void:
	_find_bar = FindBar.new()
	_find_bar.editor = self
	add_child(_find_bar)
	_peek = PanelContainer.new()
	_peek.visible = false
	_peek.mouse_filter = Control.MOUSE_FILTER_IGNORE
	var sb := UiTheme.box(Color("#23263b"), 6, 10, 7)
	sb.border_color = Color("#454a70")
	sb.set_border_width_all(1)
	_peek.add_theme_stylebox_override("panel", sb)
	_peek_label = RichTextLabel.new()
	_peek_label.fit_content = true
	_peek_label.bbcode_enabled = true
	_peek_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	_peek_label.custom_minimum_size = Vector2(420, 0)
	_peek_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_peek_label.add_theme_font_override("normal_font", Palette.code_font())
	_peek_label.add_theme_font_size_override("normal_font_size", 13)
	_peek.add_child(_peek_label)
	add_child(_peek)
	caret_changed.connect(func():
		if not _expanding:
			_expand_stack.clear())


## 내용을 통째로 바꾸되 Cmd+Z 로 되돌릴 수 있게 한다. keep_caret 이면 캐럿 줄을 지킨다.
func replace_all(new_text: String, keep_caret := false) -> void:
	var line := get_caret_line()
	var col := get_caret_column()
	var scroll := scroll_vertical
	begin_complex_operation()
	remove_secondary_carets()
	select_all()
	delete_selection()
	insert_text_at_caret(new_text)
	end_complex_operation()
	if keep_caret:
		set_caret_line(mini(line, get_line_count() - 1))
		set_caret_column(mini(col, get_line(get_caret_line()).length()))
		scroll_vertical = scroll
	else:
		set_caret_line(0)
		set_caret_column(0)
	grab_focus()


func _gui_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed:
		if _peek.visible and event.keycode not in [KEY_SHIFT, KEY_CTRL, KEY_ALT, KEY_META]:
			_peek.visible = false
		if handle_key(event):
			accept_event()


static func _is_modifier(k: int) -> bool:
	return k in [KEY_SHIFT, KEY_CTRL, KEY_ALT, KEY_META]


## VS Code 단축키를 처리했으면 true. (테스트에서도 부른다.)
func handle_key(e: InputEventKey) -> bool:
	var mac := OS.has_feature("macos")
	var cmd := e.is_command_or_control_pressed()
	var shift := e.shift_pressed
	var alt := e.alt_pressed
	# macOS 에서 Cmd 와 따로 누르는 진짜 Ctrl
	var mac_ctrl := mac and e.ctrl_pressed and not e.meta_pressed
	var k := e.keycode
	if _is_modifier(k):
		return false

	if _chord == "k":
		_chord = ""
		if cmd:
			match k:
				KEY_C: _set_line_comments(true)
				KEY_U: _set_line_comments(false)
				KEY_0, KEY_KP_0: fold_all_lines()
				KEY_J: unfold_all_lines()
		return true

	# 줄 주석 / 블록 주석
	if cmd and not alt and k == KEY_SLASH:
		toggle_comment()
		return true
	if shift and alt and not cmd and k == KEY_A:
		toggle_block_comment()
		return true
	if cmd and not shift and not alt and k == KEY_K:
		_chord = "k"
		return true

	# 줄 옮기기 / 복사 / 지우기 / 새 줄
	if alt and not cmd and not mac_ctrl and (k == KEY_UP or k == KEY_DOWN):
		if shift:
			copy_lines(k == KEY_UP)
		elif k == KEY_UP:
			move_lines_up()
		else:
			move_lines_down()
		return true
	if cmd and shift and k == KEY_K:
		delete_lines()
		return true
	if cmd and not alt and (k == KEY_ENTER or k == KEY_KP_ENTER):
		insert_line(shift)
		return true

	# 들여쓰기
	if cmd and not shift and not alt and k == KEY_BRACKETRIGHT:
		indent_lines()
		return true
	if cmd and not shift and not alt and k == KEY_BRACKETLEFT:
		unindent_lines()
		return true

	# 접기 (macOS: Alt+Cmd+[ ], 다른 OS: Ctrl+Shift+[ ])
	var fold_combo := (mac and cmd and alt and not shift) or (not mac and cmd and shift and not alt)
	if fold_combo and (k == KEY_BRACKETLEFT or k == KEY_BRACKETRIGHT):
		var line := get_caret_line()
		if k == KEY_BRACKETLEFT:
			_fold_at(line)
		else:
			unfold_line(line)
		return true

	# 선택 / 여러 캐럿
	if cmd and not shift and not alt and k == KEY_D:
		add_selection_for_next_occurrence()
		return true
	if cmd and shift and not alt and k == KEY_L:
		select_all_occurrences()
		return true
	if cmd and not shift and not alt and k == KEY_L:
		expand_line_selection()
		return true
	if cmd and alt and not shift and (k == KEY_UP or k == KEY_DOWN):
		add_caret_at_carets(k == KEY_DOWN)
		return true
	if k == KEY_ESCAPE and not cmd and not alt:
		if _find_bar.visible:
			_find_bar.close()
		elif get_caret_count() > 1:
			remove_secondary_carets()
		elif has_selection():
			deselect()
		else:
			return false
		return true
	# 선택 넓히기/좁히기 (Windows·Linux: Shift+Alt+→/←, macOS: Ctrl+Shift+Cmd+→/←)
	var expand_combo := (shift and alt and not cmd) or (mac and shift and e.meta_pressed and e.ctrl_pressed)
	if expand_combo and (k == KEY_RIGHT or k == KEY_LEFT):
		if k == KEY_RIGHT:
			expand_selection()
		else:
			shrink_selection()
		return true

	# 이동
	if cmd and shift and not alt and k == KEY_BACKSLASH:
		jump_to_bracket()
		return true
	if (mac_ctrl or (not mac and cmd)) and not shift and not alt and k == KEY_G:
		_find_bar.open("goto")
		return true
	if k == KEY_F8 and not cmd and not alt:
		goto_problem(shift)
		return true

	# 찾기 / 바꾸기
	if cmd and not shift and not alt and k == KEY_F:
		_find_bar.open("find")
		return true
	if (cmd and alt and not shift and k == KEY_F) or (not mac and cmd and not shift and not alt and k == KEY_H):
		_find_bar.open("replace")
		return true
	if k == KEY_F3 and not cmd and not alt:
		_find_bar.find_next(shift)
		return true
	if mac and cmd and not alt and k == KEY_G:
		_find_bar.find_next(shift)
		return true

	# 자동 완성 / 힌트 / 정리
	if (k == KEY_SPACE and (mac_ctrl or (not mac and cmd)) and not shift) or (mac and cmd and not shift and not alt and k == KEY_I):
		request_code_completion(true)
		return true
	if cmd and shift and k == KEY_SPACE:
		show_parameter_hint()
		return true
	if shift and alt and not cmd and k == KEY_F:
		format_requested.emit()
		return true

	# 보기
	if alt and not cmd and not shift and k == KEY_Z:
		wrap_mode = TextEdit.LINE_WRAPPING_NONE if wrap_mode == TextEdit.LINE_WRAPPING_BOUNDARY else TextEdit.LINE_WRAPPING_BOUNDARY
		return true
	if cmd and not alt and (k == KEY_EQUAL or k == KEY_PLUS or k == KEY_KP_ADD):
		set_font_size(_font_size + 1)
		return true
	if cmd and not alt and (k == KEY_MINUS or k == KEY_KP_SUBTRACT):
		set_font_size(_font_size - 1)
		return true
	if cmd and not alt and not shift and (k == KEY_0 or k == KEY_KP_0):
		set_font_size(BASE_FONT_SIZE)
		return true

	# 되돌리기: Windows·Linux 는 Ctrl+Y 도 다시 실행
	if not mac and cmd and not shift and k == KEY_Y:
		redo()
		return true
	if cmd and shift and k == KEY_Z:
		redo()
		return true
	return false


# ── 주석 ─────────────────────────────────────────────────────────

func _selected_line_range(caret := 0) -> Vector2i:
	if has_selection(caret):
		var to := get_selection_to_line(caret)
		# 다음 줄 맨 앞까지 선택했으면 그 줄은 빼고 센다 (VS Code 와 같다).
		if to > get_selection_from_line(caret) and get_selection_to_column(caret) == 0:
			to -= 1
		return Vector2i(get_selection_from_line(caret), to)
	return Vector2i(get_caret_line(caret), get_caret_line(caret))


## 캐럿마다의 줄 범위를 합친 줄 목록 (오름차순, 중복 없음).
func _all_selected_lines() -> Array:
	var lines := {}
	for c in get_caret_count():
		var r := _selected_line_range(c)
		for i in range(r.x, r.y + 1):
			lines[i] = true
	var out := lines.keys()
	out.sort()
	return out


## 선택한 줄(또는 지금 줄)을 // 주석으로 묶거나 푼다.
func toggle_comment() -> void:
	var all_commented := true
	for i in _all_selected_lines():
		var t := get_line(i).strip_edges()
		if t != "" and not t.begins_with("//"):
			all_commented = false
	_set_line_comments(not all_commented)


func _set_line_comments(add: bool) -> void:
	var lines := _all_selected_lines()
	# 주석 표시는 고른 줄들 중 가장 얕은 들여쓰기에 맞춘다.
	var indent := 1 << 20
	for i in lines:
		var line := get_line(i)
		if line.strip_edges() != "":
			indent = mini(indent, line.length() - line.strip_edges(true, false).length())
	begin_complex_operation()
	for i in lines:
		var line := get_line(i)
		if line.strip_edges() == "":
			continue
		if add:
			set_line(i, line.substr(0, indent) + "// " + line.substr(indent))
		else:
			var lead := line.length() - line.strip_edges(true, false).length()
			var rest := line.substr(lead)
			if rest.begins_with("//"):
				rest = rest.substr(2)
				if rest.begins_with(" "):
					rest = rest.substr(1)
				set_line(i, line.substr(0, lead) + rest)
	end_complex_operation()


## Shift+Alt+A: 선택을 /* */ 로 감싸거나 푼다. 선택이 없으면 빈 블록 주석을 넣는다.
func toggle_block_comment() -> void:
	begin_complex_operation()
	if has_selection():
		var sel := get_selected_text()
		var t := sel.strip_edges()
		if t.begins_with("/*") and t.ends_with("*/"):
			var inner := t.substr(2, t.length() - 4)
			if inner.begins_with(" "):
				inner = inner.substr(1)
			if inner.ends_with(" "):
				inner = inner.left(inner.length() - 1)
			insert_text_at_caret(inner)
		else:
			insert_text_at_caret("/* " + sel + " */")
	else:
		insert_text_at_caret("/*  */")
		set_caret_column(get_caret_column() - 3)
	end_complex_operation()


# ── 줄 편집 ──────────────────────────────────────────────────────

## Shift+Alt+↑/↓: 고른 줄들을 위/아래로 복사한다. 캐럿은 위로 복사하면 위 사본에 남는다.
func copy_lines(up: bool) -> void:
	var r := _selected_line_range()
	var block: Array = []
	for i in range(r.x, r.y + 1):
		block.append(get_line(i))
	var text := "\n".join(PackedStringArray(block))
	var count := r.y - r.x + 1
	var had_sel := has_selection()
	var sel := [get_selection_origin_line(), get_selection_origin_column(), get_caret_line(), get_caret_column()]
	remove_secondary_carets()
	begin_complex_operation()
	insert_text(text + "\n", r.x, 0, false, false)
	end_complex_operation()
	var shift_lines := 0 if up else count
	if had_sel:
		select(sel[0] + shift_lines, sel[1], sel[2] + shift_lines, sel[3])
	else:
		deselect()
		set_caret_line(sel[2] + shift_lines)
		set_caret_column(sel[3])


## Cmd+Enter / Cmd+Shift+Enter: 줄 중간에서도 지금 줄은 그대로 두고 아래/위에 새 줄을 연다.
func insert_line(above: bool) -> void:
	remove_secondary_carets()
	deselect()
	var line := get_caret_line()
	var indent_text := _line_indent(line)
	begin_complex_operation()
	if above:
		insert_text(indent_text + "\n", line, 0, false, false)
		set_caret_line(line)
	else:
		# 줄이 { 로 끝나면 한 단계 더 들여 쓴다.
		if get_line(line).strip_edges().ends_with("{"):
			indent_text += " ".repeat(indent_size)
		insert_text("\n" + indent_text, line, get_line(line).length(), false, false)
		set_caret_line(line + 1)
	set_caret_column(indent_text.length())
	end_complex_operation()


func _line_indent(line: int) -> String:
	var s := get_line(line)
	return s.substr(0, s.length() - s.strip_edges(true, false).length())


## Cmd+L: 지금 줄 전체를 선택한다. 다시 누르면 다음 줄까지 넓힌다.
func expand_line_selection() -> void:
	for c in get_caret_count():
		var from := get_caret_line(c)
		var to := from
		if has_selection(c):
			from = get_selection_from_line(c)
			to = get_selection_to_line(c)
			if get_selection_to_column(c) != 0 or to == from:
				to += 1
			else:
				to += 1
		else:
			to = from + 1
		if to >= get_line_count():
			select(from, 0, get_line_count() - 1, get_line(get_line_count() - 1).length(), c)
		else:
			select(from, 0, to, 0, c)


## Cmd+Shift+L: 선택한 글(없으면 캐럿 밑 단어)이 나오는 곳을 모두 선택한다.
func select_all_occurrences() -> void:
	# 선택이 없으면 첫 호출은 캐럿 밑 단어를 고르기만 한다.
	if not has_selection():
		add_selection_for_next_occurrence()
	var guard := 0
	var before := -1
	while get_caret_count() != before and guard < 1000:
		before = get_caret_count()
		add_selection_for_next_occurrence()
		guard += 1


func _fold_at(line: int) -> void:
	# 지금 줄이 접을 수 없으면 감싸고 있는 블록(들여쓰기가 더 얕은 위쪽 줄)을 접는다.
	var l := line
	while l >= 0:
		if can_fold_line(l) and _block_end(l) >= line:
			fold_line(l)
			set_caret_line(l)
			return
		l -= 1


func _block_end(line: int) -> int:
	var base := get_indent_level(line)
	var i := line + 1
	while i < get_line_count():
		if get_line(i).strip_edges() != "" and get_indent_level(i) <= base:
			return i
		i += 1
	return get_line_count() - 1


func set_font_size(px: int) -> void:
	_font_size = clampi(px, 9, 32)
	add_theme_font_size_override("font_size", _font_size)


# ── 괄호 ─────────────────────────────────────────────────────────

const OPEN := "([{"
const CLOSE := ")]}"


func _offset_of(line: int, col: int) -> int:
	var off := 0
	for i in line:
		off += get_line(i).length() + 1
	return off + col


func _pos_of(offset: int) -> Vector2i:
	var line := 0
	while line < get_line_count() - 1 and offset > get_line(line).length():
		offset -= get_line(line).length() + 1
		line += 1
	return Vector2i(offset, line)


## 문자열·주석 안의 괄호는 빼고, offset 에 있는 괄호의 짝 위치 (-1 이면 없음).
func _match_bracket(src: String, offset: int) -> int:
	var c := src[offset]
	var mask := _code_mask(src)
	if not mask[offset]:
		return -1
	var depth := 0
	if OPEN.contains(c):
		var close := CLOSE[OPEN.find(c)]
		for i in range(offset, src.length()):
			if not mask[i]:
				continue
			if src[i] == c:
				depth += 1
			elif src[i] == close:
				depth -= 1
				if depth == 0:
					return i
	elif CLOSE.contains(c):
		var open := OPEN[CLOSE.find(c)]
		for i in range(offset, -1, -1):
			if not mask[i]:
				continue
			if src[i] == c:
				depth += 1
			elif src[i] == open:
				depth -= 1
				if depth == 0:
					return i
	return -1


## 글자마다 "코드인가" (문자열·주석이면 false).
static func _code_mask(src: String) -> PackedByteArray:
	var mask := PackedByteArray()
	mask.resize(src.length())
	mask.fill(1)
	var i := 0
	while i < src.length():
		var two := src.substr(i, 2)
		if two == "//":
			while i < src.length() and src[i] != "\n":
				mask[i] = 0
				i += 1
		elif two == "/*":
			var end := src.find("*/", i + 2)
			end = src.length() if end < 0 else end + 2
			for j in range(i, end):
				mask[j] = 0
			i = end
		elif src[i] == "\"":
			mask[i] = 0
			i += 1
			while i < src.length() and src[i] != "\"":
				if src[i] == "\\":
					mask[i] = 0
					i += 1
				if i < src.length():
					mask[i] = 0
				i += 1
			if i < src.length():
				mask[i] = 0
			i += 1
		else:
			i += 1
	return mask


## Cmd+Shift+\: 캐럿 옆 괄호의 짝으로 간다. 옆에 괄호가 없으면 감싸고 있는 여는 괄호로.
func jump_to_bracket() -> void:
	var src := text
	var off := _offset_of(get_caret_line(), get_caret_column())
	var target := -1
	for at in [off, off - 1]:
		if at >= 0 and at < src.length() and (OPEN + CLOSE).contains(src[at]):
			var m := _match_bracket(src, at)
			if m >= 0:
				target = m
				break
	if target < 0:
		target = _enclosing_open(src, off)
	if target < 0:
		return
	var p := _pos_of(target)
	deselect()
	set_caret_line(p.y)
	set_caret_column(p.x)


func _enclosing_open(src: String, off: int) -> int:
	var mask := _code_mask(src)
	var depth := 0
	for i in range(off - 1, -1, -1):
		if not mask[i]:
			continue
		if CLOSE.contains(src[i]):
			depth += 1
		elif OPEN.contains(src[i]):
			if depth == 0:
				return i
			depth -= 1
	return -1


# ── 선택 넓히기 ───────────────────────────────────────────────────

var _expanding := false


func _current_range() -> Vector2i:
	if has_selection():
		return Vector2i(_offset_of(get_selection_from_line(), get_selection_from_column()), _offset_of(get_selection_to_line(), get_selection_to_column()))
	var off := _offset_of(get_caret_line(), get_caret_column())
	return Vector2i(off, off)


## Shift+Alt+→: 단어 → 괄호 안 → 괄호까지 → 줄 → 전체 순으로 선택을 넓힌다.
func expand_selection() -> void:
	var src := text
	var cur := _current_range()
	var cands: Array = []
	# 단어
	var a := cur.x
	var b := cur.y
	while a > 0 and is_word_char(src[a - 1]):
		a -= 1
	while b < src.length() and is_word_char(src[b]):
		b += 1
	cands.append(Vector2i(a, b))
	# 감싸는 괄호들 (안쪽, 괄호 포함)
	var mask := _code_mask(src)
	var start := cur.x
	for _guard in 50:
		var open := _enclosing_open(src, start)
		if open < 0:
			break
		var close := _match_bracket(src, open)
		if close < 0:
			break
		if mask[open]:
			cands.append(Vector2i(open + 1, close))
			cands.append(Vector2i(open, close + 1))
		start = open
	# 줄
	var lf := _pos_of(cur.x).y
	var lt := _pos_of(cur.y).y
	var line_from := _offset_of(lf, 0) + _line_indent(lf).length()
	cands.append(Vector2i(line_from, _offset_of(lt, get_line(lt).length())))
	cands.append(Vector2i(0, src.length()))
	cands.sort_custom(func(p, q): return (p.y - p.x) < (q.y - q.x))
	for c in cands:
		if c.x <= cur.x and c.y >= cur.y and (c.x < cur.x or c.y > cur.y):
			_expand_stack.append(cur)
			_select_range(c)
			return


func shrink_selection() -> void:
	if _expand_stack.is_empty():
		return
	_select_range(_expand_stack.pop_back())


func _select_range(r: Vector2i) -> void:
	_expanding = true
	remove_secondary_carets()
	var a := _pos_of(r.x)
	var b := _pos_of(r.y)
	if r.x == r.y:
		deselect()
		set_caret_line(a.y)
		set_caret_column(a.x)
	else:
		select(a.y, a.x, b.y, b.x)
	_expanding = false


# ── 문제 / 힌트 ───────────────────────────────────────────────────

## F8 / Shift+F8: 다음/이전 컴파일러 문제로 가서 메시지를 보여 준다.
func goto_problem(backwards: bool) -> void:
	if _diagnostics.is_empty():
		_show_peek("[color=#8a8fa8]문제가 없어요.[/color]", get_caret_line())
		return
	var sorted := _diagnostics.duplicate()
	sorted.sort_custom(func(p, q): return p.line < q.line or (p.line == q.line and p.col < q.col))
	var here := Vector2i(get_caret_column() + 1, get_caret_line() + 1)
	var pick = null
	if backwards:
		pick = sorted[-1]
		for i in range(sorted.size() - 1, -1, -1):
			var d = sorted[i]
			if d.line < here.y or (d.line == here.y and d.col < here.x):
				pick = d
				break
	else:
		pick = sorted[0]
		for d in sorted:
			if d.line > here.y or (d.line == here.y and d.col > here.x):
				pick = d
				break
	remove_secondary_carets()
	deselect()
	set_caret_line(pick.line - 1)
	set_caret_column(pick.col - 1)
	center_viewport_to_caret()
	var idx := sorted.find(pick) + 1
	var color := "#ff6b7a" if pick.error else "#f2c94c"
	_show_peek("[color=%s]%s %d/%d[/color]\n%s" % [color, "에러" if pick.error else "경고", idx, sorted.size(), pick.message.replace("[", "[lb]")], pick.line - 1)


## Cmd+Shift+Space: 캐럿이 들어 있는 함수 호출의 시그니처를 보여 준다.
func show_parameter_hint() -> void:
	var line := get_line(get_caret_line()).left(get_caret_column())
	var depth := 0
	for i in range(line.length() - 1, -1, -1):
		if line[i] == ")":
			depth += 1
		elif line[i] == "(":
			if depth == 0:
				var name_end := i
				var s := i
				while s > 0 and is_word_char(line[s - 1]):
					s -= 1
				var fn_name := line.substr(s, name_end - s)
				for api in API:
					if api[0] == fn_name:
						_show_peek("[color=#8ec5ff]%s[/color]\n[color=#8a8fa8]%s[/color]" % [api[2], api[3]], get_caret_line())
						return
				return
			depth -= 1


func _show_peek(bbcode: String, line: int) -> void:
	_peek_label.text = bbcode
	_peek.visible = true
	_peek.reset_size()
	var r := get_rect_at_line_column(line, 0)
	var y := float(r.end.y) + 4.0 if r.position.y >= 0 else 8.0
	if y + _peek.size.y > size.y:
		y = maxf(float(r.position.y) - _peek.size.y - 4.0, 4.0)
	_peek.position = Vector2(48, y)


# ── 자동 완성 ────────────────────────────────────────────────────

func _on_text_changed() -> void:
	_idle_timer.start()
	_complete_timer.start()
	if not _diagnostics.is_empty():
		# 고치는 중에는 예전 밑줄이 엉뚱한 곳에 남지 않게 지운다.
		set_diagnostics([])


func _word_before_caret() -> String:
	var line := get_line(get_caret_line())
	var col := get_caret_column()
	var start := col
	while start > 0 and is_word_char(line[start - 1]):
		start -= 1
	return line.substr(start, col - start)


static func is_word_char(c: String) -> bool:
	return c == "_" or (c >= "a" and c <= "z") or (c >= "A" and c <= "Z") or (c >= "0" and c <= "9")


func _maybe_complete() -> void:
	var word := _word_before_caret()
	if word.length() >= 2 and not word[0].is_valid_int() and has_focus():
		request_code_completion()


func _on_completion_requested() -> void:
	for a in API:
		add_code_completion_option(CodeEdit.KIND_FUNCTION, a[0], a[1], COL_FUNC, null, a[2])
	for t in TYPES:
		add_code_completion_option(CodeEdit.KIND_CLASS, t[0], t[0], COL_TYPE, null, t[1])
	for v in VARIANTS:
		add_code_completion_option(CodeEdit.KIND_ENUM, v, v, COL_TYPE)
	for k in KEYWORDS:
		add_code_completion_option(CodeEdit.KIND_PLAIN_TEXT, k, k, COL_KEYWORD)
	for m in MACROS:
		add_code_completion_option(CodeEdit.KIND_FUNCTION, m, m.trim_suffix("!") + "!(", COL_FUNC)
	for t in PRIMITIVES:
		add_code_completion_option(CodeEdit.KIND_CLASS, t, t, COL_TYPE)
	update_code_completion_options(false)


# ── 컴파일러 진단 ─────────────────────────────────────────────────

## rustc 진단 목록 (줄·칸은 1부터). 에러 줄에 배경색을 깔고 _draw 에서 물결 밑줄을 긋는다.
func set_diagnostics(list: Array) -> void:
	for i in get_line_count():
		set_line_background_color(i, Color(0, 0, 0, 0))
	_diagnostics = list
	for d in list:
		var line: int = d.line - 1
		if line >= 0 and line < get_line_count():
			set_line_background_color(line, Color(Palette.RED if d.error else Palette.WARN, 0.08))
	queue_redraw()


func first_error_line() -> int:
	for d in _diagnostics:
		if d.error:
			return d.line - 1
	return -1


func _draw() -> void:
	for d in _diagnostics:
		var color: Color = Palette.RED if d.error else Palette.WARN
		for line in range(d.line - 1, d.endLine):
			if line < 0 or line >= get_line_count():
				continue
			var from_col: int = d.col - 1 if line == d.line - 1 else 0
			var to_col: int = d.endCol - 1 if line == d.endLine - 1 else get_line(line).length()
			if to_col <= from_col:
				to_col = from_col + 1
			_squiggle(line, from_col, mini(to_col, get_line(line).length()), color)


func _squiggle(line: int, from_col: int, to_col: int, color: Color) -> void:
	var a := get_rect_at_line_column(line, from_col)
	var b := get_rect_at_line_column(line, maxi(to_col, from_col))
	if a.position.x < 0 or b.position.x < 0:
		return
	var x0 := float(a.position.x)
	var x1 := maxf(float(b.position.x), x0 + 6.0)
	var y := float(a.end.y) - 2.0
	var pts := PackedVector2Array()
	var x := x0
	var up := true
	while x <= x1:
		pts.append(Vector2(x, y + (-1.5 if up else 1.5)))
		x += 3.0
		up = not up
	if pts.size() >= 2:
		draw_polyline(pts, color, 1.2, true)


# ── 마우스를 올렸을 때 ────────────────────────────────────────────

func _get_tooltip(at: Vector2) -> String:
	var pos := get_line_column_at_pos(Vector2i(at), false)
	if pos.x >= 0 and pos.y >= 0:
		for d in _diagnostics:
			var line := pos.y + 1
			var col := pos.x + 1
			var after_start: bool = line > d.line or (line == d.line and col >= d.col)
			var before_end: bool = line < d.endLine or (line == d.endLine and col <= d.endCol)
			if after_start and before_end:
				return "rustc: " + d.message
	var word := get_word_at_pos(at)
	for a in API:
		if a[0] == word:
			return a[2] + "\n\n" + a[3]
	for t in TYPES:
		if t[0] == word:
			return t[1] + "\n\n" + t[2]
	return ""


func _make_custom_tooltip(for_text: String) -> Object:
	var label := RichTextLabel.new()
	label.fit_content = true
	label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	label.custom_minimum_size = Vector2(minf(460.0, 40.0 + for_text.length() * 8.0), 0)
	label.add_theme_font_override("normal_font", Palette.code_font())
	label.add_theme_font_size_override("normal_font_size", 13)
	label.add_theme_color_override("default_color", Palette.TEXT)
	label.add_text(for_text)
	return label
