class_name FindBar
## 에디터 오른쪽 위에 뜨는 찾기 / 바꾸기 / 줄 이동 상자 (VS Code 의 찾기 위젯과 같은 키).
##
## 찾기: Enter 다음, Shift+Enter 이전, Alt+Enter 찾은 것 모두 선택, Esc 닫기,
##       Alt+C 대소문자 구분, Alt+W 단어 단위.
## 바꾸기: Enter 하나 바꾸기, Cmd+Alt+Enter 모두 바꾸기.
## 줄 이동: 줄 번호(또는 줄:칸)를 쓰고 Enter.
extends PanelContainer

var editor: CodeEdit
var mode := "find" # find | replace | goto

var find_edit := LineEdit.new()
var replace_edit := LineEdit.new()
var count_label := Label.new()
var case_button := Button.new()
var word_button := Button.new()
var _replace_row := HBoxContainer.new()
var _find_row := HBoxContainer.new()
var _nav_buttons: Array[Button] = []


func _init() -> void:
	visible = false
	mouse_filter = Control.MOUSE_FILTER_STOP
	add_theme_stylebox_override("panel", _panel_box())
	var col := VBoxContainer.new()
	col.add_theme_constant_override("separation", 6)
	add_child(col)

	_find_row.add_theme_constant_override("separation", 4)
	col.add_child(_find_row)
	_setup_edit(find_edit, "찾기")
	find_edit.text_changed.connect(func(_t): _refresh(true))
	_find_row.add_child(find_edit)
	_setup_toggle(case_button, "Aa", "대소문자 구분 (Alt+C)")
	_setup_toggle(word_button, "ab", "단어 단위 (Alt+W)")
	_find_row.add_child(case_button)
	_find_row.add_child(word_button)
	count_label.custom_minimum_size = Vector2(64, 0)
	count_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	count_label.add_theme_font_size_override("font_size", 12)
	count_label.add_theme_color_override("font_color", Palette.DIM)
	_find_row.add_child(count_label)
	_find_row.add_child(_small_button("↑", "이전 (Shift+Enter)", func(): find_next(true)))
	_find_row.add_child(_small_button("↓", "다음 (Enter)", func(): find_next(false)))
	_find_row.add_child(_small_button("✕", "닫기 (Esc)", close))

	_replace_row.add_theme_constant_override("separation", 4)
	col.add_child(_replace_row)
	_setup_edit(replace_edit, "바꿀 내용")
	_replace_row.add_child(replace_edit)
	_replace_row.add_child(_small_button("바꾸기", "하나 바꾸기 (Enter)", replace_one))
	_replace_row.add_child(_small_button("모두", "모두 바꾸기 (Cmd+Alt+Enter)", replace_all))

	for b in [case_button, word_button]:
		b.toggled.connect(func(_on): _refresh(true))


static func _panel_box() -> StyleBoxFlat:
	var sb := UiTheme.box(Color("#23263b"), 8, 8, 6)
	sb.border_color = Color("#454a70")
	sb.set_border_width_all(1)
	sb.shadow_color = Color(0, 0, 0, 0.35)
	sb.shadow_size = 8
	return sb


func _setup_edit(e: LineEdit, placeholder: String) -> void:
	e.placeholder_text = placeholder
	e.custom_minimum_size = Vector2(190, 28)
	e.add_theme_font_override("font", Palette.code_font())
	e.add_theme_font_size_override("font_size", 13)
	e.add_theme_stylebox_override("normal", UiTheme.box(Palette.EDITOR, 5, 8, 3))
	var focus := UiTheme.box(Palette.EDITOR, 5, 8, 3)
	focus.border_color = Palette.SKY
	focus.set_border_width_all(1)
	e.add_theme_stylebox_override("focus", focus)
	e.gui_input.connect(_on_edit_key.bind(e))


func _setup_toggle(b: Button, text: String, tip: String) -> void:
	b.text = text
	b.toggle_mode = true
	b.tooltip_text = tip
	b.focus_mode = Control.FOCUS_NONE
	b.custom_minimum_size = Vector2(30, 28)
	b.add_theme_font_size_override("font_size", 12)
	b.add_theme_stylebox_override("normal", UiTheme.box(Color(0, 0, 0, 0), 5, 4, 2))
	b.add_theme_stylebox_override("hover", UiTheme.box(Palette.PANEL_HI, 5, 4, 2))
	b.add_theme_stylebox_override("pressed", UiTheme.box(Color(Palette.SKY, 0.3), 5, 4, 2))


func _small_button(text: String, tip: String, action: Callable) -> Button:
	var b := Button.new()
	b.text = text
	b.tooltip_text = tip
	b.focus_mode = Control.FOCUS_NONE
	b.custom_minimum_size = Vector2(28, 28)
	b.add_theme_font_size_override("font_size", 12)
	b.add_theme_stylebox_override("normal", UiTheme.box(Color(0, 0, 0, 0), 5, 6, 2))
	b.add_theme_stylebox_override("hover", UiTheme.box(Palette.PANEL_HI, 5, 6, 2))
	b.add_theme_stylebox_override("pressed", UiTheme.box(Palette.PANEL_HI.darkened(0.1), 5, 6, 2))
	b.pressed.connect(action)
	_nav_buttons.append(b)
	return b


# ── 열고 닫기 ────────────────────────────────────────────────────

func open(new_mode: String) -> void:
	mode = new_mode
	visible = true
	var goto := mode == "goto"
	if mode == "replace" and not editor.editable:
		mode = "find"
	_replace_row.visible = mode == "replace"
	for c in _find_row.get_children():
		c.visible = not goto or c == find_edit
	if goto:
		find_edit.placeholder_text = "줄 번호 (1 - %d), 줄:칸" % editor.get_line_count()
		find_edit.text = ""
	else:
		find_edit.placeholder_text = "찾기"
		# 선택한 글이 한 줄이면 그걸로 찾기를 시작한다.
		var sel := editor.get_selected_text()
		if sel != "" and not sel.contains("\n"):
			find_edit.text = sel
	reset_size()
	_reposition()
	find_edit.grab_focus()
	find_edit.select_all()
	_refresh(false)


func close() -> void:
	if not visible:
		return
	visible = false
	editor.set_search_text("")
	editor.queue_redraw()
	editor.grab_focus()


func _reposition() -> void:
	var w := get_combined_minimum_size().x
	position = Vector2(editor.size.x - w - 22.0, 6.0)


# ── 찾기 ─────────────────────────────────────────────────────────

func _flags() -> int:
	var f := 0
	if case_button.button_pressed:
		f |= TextEdit.SEARCH_MATCH_CASE
	if word_button.button_pressed:
		f |= TextEdit.SEARCH_WHOLE_WORDS
	return f


## 찾은 곳 전부: [줄, 칸] 목록.
func matches() -> Array:
	var out: Array = []
	var needle := find_edit.text
	if needle == "" or mode == "goto":
		return out
	var cs := case_button.button_pressed
	if not cs:
		needle = needle.to_lower()
	for line in editor.get_line_count():
		var hay := editor.get_line(line)
		if not cs:
			hay = hay.to_lower()
		var from := 0
		while true:
			var at := hay.find(needle, from)
			if at < 0:
				break
			if not word_button.button_pressed or _is_whole_word(hay, at, needle.length()):
				out.append([line, at])
			from = at + maxi(needle.length(), 1)
	return out


static func _is_whole_word(s: String, at: int, n: int) -> bool:
	var before := s[at - 1] if at > 0 else " "
	var after := s[at + n] if at + n < s.length() else " "
	return not RustEditor.is_word_char(before) and not RustEditor.is_word_char(after)


func _refresh(jump: bool) -> void:
	if mode == "goto":
		count_label.text = ""
		return
	editor.set_search_flags(_flags())
	editor.set_search_text(find_edit.text)
	editor.queue_redraw()
	var all := matches()
	if all.is_empty():
		count_label.text = "결과 없음" if find_edit.text != "" else ""
		count_label.add_theme_color_override("font_color", Palette.RED if find_edit.text != "" else Palette.DIM)
		return
	count_label.add_theme_color_override("font_color", Palette.DIM)
	if jump:
		# 입력하는 동안에는 캐럿 자리부터 가장 가까운 결과로 옮긴다.
		var line := editor.get_selection_from_line() if editor.has_selection() else editor.get_caret_line()
		var col := editor.get_selection_from_column() if editor.has_selection() else editor.get_caret_column()
		_select_match(_first_at_or_after(all, line, col))
	_update_count(all)


func _first_at_or_after(all: Array, line: int, col: int) -> Array:
	for m in all:
		if m[0] > line or (m[0] == line and m[1] >= col):
			return m
	return all[0]


func _update_count(all: Array) -> void:
	var idx := 0
	for i in all.size():
		if all[i][0] == editor.get_caret_line() and all[i][1] + find_edit.text.length() == editor.get_caret_column():
			idx = i + 1
	count_label.text = "%s / %d" % [str(idx) if idx > 0 else "?", all.size()]


func _select_match(m: Array) -> void:
	editor.remove_secondary_carets()
	editor.select(m[0], m[1], m[0], m[1] + find_edit.text.length())
	editor.center_viewport_to_caret()


func find_next(backwards: bool) -> void:
	if find_edit.text == "":
		var sel := editor.get_selected_text()
		if sel == "":
			return
		find_edit.text = sel
	editor.set_search_flags(_flags())
	editor.set_search_text(find_edit.text)
	var all := matches()
	if all.is_empty():
		_update_count(all)
		return
	var line := editor.get_caret_line()
	var col := editor.get_caret_column()
	var pick: Array = []
	if backwards:
		var start := editor.get_selection_from_column() if editor.has_selection() else col
		pick = all[-1]
		for i in range(all.size() - 1, -1, -1):
			var m: Array = all[i]
			if m[0] < line or (m[0] == line and m[1] < start):
				pick = m
				break
	else:
		pick = all[0]
		for m in all:
			if m[0] > line or (m[0] == line and m[1] >= col):
				pick = m
				break
	_select_match(pick)
	_update_count(all)


## Alt+Enter: 찾은 곳 전부를 여러 캐럿으로 선택한다.
func select_all_matches() -> void:
	var all := matches()
	if all.is_empty():
		return
	var n := find_edit.text.length()
	editor.remove_secondary_carets()
	editor.select(all[0][0], all[0][1], all[0][0], all[0][1] + n)
	for i in range(1, all.size()):
		var c := editor.add_caret(all[i][0], all[i][1] + n)
		if c >= 0:
			editor.select(all[i][0], all[i][1], all[i][0], all[i][1] + n, c)
	close()


# ── 바꾸기 ───────────────────────────────────────────────────────

func _selection_is_match() -> bool:
	var sel := editor.get_selected_text()
	if sel == "" or editor.get_caret_count() > 1:
		return false
	return sel == find_edit.text if case_button.button_pressed else sel.to_lower() == find_edit.text.to_lower()


func replace_one() -> void:
	if find_edit.text == "" or not editor.editable:
		return
	if not _selection_is_match():
		find_next(false)
		if not _selection_is_match():
			return
	editor.begin_complex_operation()
	editor.delete_selection()
	editor.insert_text_at_caret(replace_edit.text)
	editor.end_complex_operation()
	find_next(false)


func replace_all() -> void:
	if not editor.editable:
		return
	var all := matches()
	if all.is_empty():
		return
	var n := find_edit.text.length()
	editor.begin_complex_operation()
	editor.remove_secondary_carets()
	editor.deselect()
	# 뒤에서부터 바꿔야 앞쪽 위치가 밀리지 않는다.
	for i in range(all.size() - 1, -1, -1):
		var m: Array = all[i]
		editor.remove_text(m[0], m[1], m[0], m[1] + n)
		editor.insert_text(replace_edit.text, m[0], m[1])
	editor.end_complex_operation()
	_refresh(false)
	count_label.text = "%d개 바꿈" % all.size()


# ── 줄 이동 ──────────────────────────────────────────────────────

func goto_line() -> void:
	var parts := find_edit.text.strip_edges().split(":")
	if parts.is_empty() or not parts[0].is_valid_int():
		return
	var line := clampi(parts[0].to_int(), 1, editor.get_line_count()) - 1
	var col := 0
	if parts.size() > 1 and parts[1].is_valid_int():
		col = clampi(parts[1].to_int() - 1, 0, editor.get_line(line).length())
	editor.remove_secondary_carets()
	editor.deselect()
	editor.set_caret_line(line)
	editor.set_caret_column(col)
	editor.center_viewport_to_caret()
	close()


# ── 입력 상자 키 ─────────────────────────────────────────────────

func _on_edit_key(event: InputEvent, edit: LineEdit) -> void:
	if not (event is InputEventKey) or not event.pressed:
		return
	var e := event as InputEventKey
	var cmd := e.is_command_or_control_pressed()
	var handled := true
	match e.keycode:
		KEY_ESCAPE:
			close()
		KEY_ENTER, KEY_KP_ENTER:
			if mode == "goto":
				goto_line()
			elif edit == replace_edit:
				if cmd and e.alt_pressed:
					replace_all()
				else:
					replace_one()
			elif e.alt_pressed:
				select_all_matches()
			else:
				find_next(e.shift_pressed)
		KEY_F3:
			find_next(e.shift_pressed)
		KEY_C when e.alt_pressed and not cmd:
			case_button.button_pressed = not case_button.button_pressed
		KEY_W when e.alt_pressed and not cmd:
			word_button.button_pressed = not word_button.button_pressed
		KEY_G when cmd and OS.has_feature("macos"):
			find_next(e.shift_pressed)
		KEY_F when cmd and not e.alt_pressed:
			if mode == "goto":
				open("find")
			else:
				edit.select_all()
		KEY_H when cmd and not OS.has_feature("macos"):
			open("replace")
		KEY_F when cmd and e.alt_pressed:
			open("replace")
		_:
			handled = false
	if handled:
		edit.accept_event()
