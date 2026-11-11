## 화면 전체의 테마를 코드로 만든다. (색은 Palette 에 모여 있다.)
class_name UiTheme


static func box(color: Color, radius := 8, pad_x := 0.0, pad_y := 0.0) -> StyleBoxFlat:
	var s := StyleBoxFlat.new()
	s.bg_color = color
	s.set_corner_radius_all(radius)
	s.content_margin_left = pad_x
	s.content_margin_right = pad_x
	s.content_margin_top = pad_y
	s.content_margin_bottom = pad_y
	s.anti_aliasing = true
	return s


static func build() -> Theme:
	var t := Theme.new()
	t.default_font = Palette.text_font()
	t.default_font_size = 15

	# 버튼
	t.set_stylebox("normal", "Button", box(Palette.PANEL_HI, 8, 16, 8))
	t.set_stylebox("hover", "Button", box(Palette.PANEL_HI.lightened(0.12), 8, 16, 8))
	t.set_stylebox("pressed", "Button", box(Palette.PANEL_HI.darkened(0.15), 8, 16, 8))
	t.set_stylebox("disabled", "Button", box(Palette.PANEL_HI.darkened(0.2), 8, 16, 8))
	t.set_stylebox("focus", "Button", StyleBoxEmpty.new())
	t.set_color("font_color", "Button", Palette.TEXT)
	t.set_color("font_hover_color", "Button", Palette.TEXT)
	t.set_color("font_pressed_color", "Button", Palette.TEXT)
	t.set_color("font_focus_color", "Button", Palette.TEXT)
	t.set_color("font_disabled_color", "Button", Palette.DIM)

	# 실행 버튼
	t.set_type_variation("PrimaryButton", "Button")
	t.set_stylebox("normal", "PrimaryButton", box(Palette.ACCENT, 8, 16, 8))
	t.set_stylebox("hover", "PrimaryButton", box(Palette.ACCENT.lightened(0.12), 8, 16, 8))
	t.set_stylebox("pressed", "PrimaryButton", box(Palette.ACCENT.darkened(0.15), 8, 16, 8))
	t.set_stylebox("disabled", "PrimaryButton", box(Palette.ACCENT.darkened(0.35), 8, 16, 8))
	t.set_color("font_color", "PrimaryButton", Color.WHITE)

	# 장·레벨 칩
	for kind in ["Chip", "ChipCurrent", "ChipDone"]:
		t.set_type_variation(kind, "Button")
	var chip := func(kind: String, bg: Color, fg: Color) -> void:
		t.set_stylebox("normal", kind, box(bg, 6, 9, 3))
		t.set_stylebox("hover", kind, box(bg.lightened(0.12), 6, 9, 3))
		t.set_stylebox("pressed", kind, box(bg.darkened(0.1), 6, 9, 3))
		t.set_color("font_color", kind, fg)
		t.set_color("font_hover_color", kind, fg)
		t.set_color("font_pressed_color", kind, fg)
		t.set_font_size("font_size", kind, 13)
	chip.call("Chip", Palette.PANEL_HI, Palette.DIM)
	chip.call("ChipCurrent", Palette.ACCENT, Color.WHITE)
	chip.call("ChipDone", Palette.GREEN_BG, Palette.GREEN)

	# 패널
	t.set_stylebox("panel", "PanelContainer", box(Palette.PANEL, 12, 16, 14))
	t.set_type_variation("EditorPanel", "PanelContainer")
	t.set_stylebox("panel", "EditorPanel", box(Palette.EDITOR, 12, 0, 0))

	# 글
	t.set_color("font_color", "Label", Palette.TEXT)
	t.set_color("default_color", "RichTextLabel", Palette.TEXT)
	t.set_font("mono_font", "RichTextLabel", Palette.code_font())
	t.set_font_size("mono_font_size", "RichTextLabel", 13)
	# [bgcolor] 상자(인라인 코드)의 여백: 옆은 살짝 띄우고, 위아래는 글자에 붙인다.
	t.set_constant("text_highlight_h_padding", "RichTextLabel", 3)
	t.set_constant("text_highlight_v_padding", "RichTextLabel", 2)
	t.set_stylebox("normal", "RichTextLabel", StyleBoxEmpty.new())
	t.set_stylebox("focus", "RichTextLabel", StyleBoxEmpty.new())

	# 스크롤바
	var grabber := box(Color(Palette.FAINT, 0.6), 4)
	for bar in ["VScrollBar", "HScrollBar"]:
		t.set_stylebox("scroll", bar, box(Color(0, 0, 0, 0), 4, 3, 3))
		t.set_stylebox("grabber", bar, grabber)
		t.set_stylebox("grabber_highlight", bar, box(Palette.FAINT, 4))
		t.set_stylebox("grabber_pressed", bar, box(Palette.DIM, 4))

	# 툴팁
	t.set_stylebox("panel", "TooltipPanel", box(Color("#23263b"), 6, 10, 7))
	t.set_color("font_color", "TooltipLabel", Palette.TEXT)
	return t
