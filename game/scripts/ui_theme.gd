## 화면 전체의 테마를 코드로 만든다. (색은 Palette 에 모여 있다.)
class_name UiTheme


## 픽셀 아트에 어울리게 모서리는 거의 각지게 (radius 는 넘겨받은 값의 절반, 최대 4).
static func box(color: Color, radius := 8, pad_x := 0.0, pad_y := 0.0) -> StyleBoxFlat:
	var s := StyleBoxFlat.new()
	s.bg_color = color
	s.set_corner_radius_all(mini(radius / 2, 4))
	s.content_margin_left = pad_x
	s.content_margin_right = pad_x
	s.content_margin_top = pad_y
	s.content_margin_bottom = pad_y
	s.anti_aliasing = true
	return s


## 키보드 포커스 링: SKY 2px (버튼 면 위에 겹쳐 그린다).
static func focus_ring() -> StyleBoxFlat:
	var s := box(Color(0, 0, 0, 0), 8, 16, 8)
	s.draw_center = false
	s.border_color = Palette.SKY
	s.set_border_width_all(2)
	s.expand_margin_left = 2
	s.expand_margin_right = 2
	s.expand_margin_top = 2
	s.expand_margin_bottom = 2
	return s


## Sunnyside 의 9칸 상자 그림으로 만든 버튼 면 (모서리 3px).
static func sprite_box(path: String, tint := Color.WHITE) -> StyleBoxTexture:
	var s := StyleBoxTexture.new()
	s.texture = load(path)
	s.set_texture_margin_all(3)
	s.content_margin_left = 16
	s.content_margin_right = 16
	s.content_margin_top = 8
	s.content_margin_bottom = 8
	s.modulate_color = tint
	return s


static func _pressable(color: Color) -> StyleBoxFlat:
	var s := box(color, 8, 16, 8)
	s.border_width_bottom = 3
	s.border_color = color.darkened(0.35)
	return s


static func build() -> Theme:
	var t := Theme.new()
	t.default_font = Palette.text_font()
	t.default_font_size = 15
	# 버튼·칩·제목은 픽셀 폰트, 긴 글(레슨 본문·출력)은 읽기 편한 글꼴
	t.set_font("font", "Button", Palette.pixel_font())
	t.set_font_size("font_size", "Button", Palette.PIXEL_SIZE)
	t.set_font("font", "OptionButton", Palette.pixel_font())
	t.set_font_size("font_size", "OptionButton", Palette.PIXEL_SIZE)
	t.set_font("font", "PopupMenu", Palette.pixel_font())
	t.set_font_size("font_size", "PopupMenu", Palette.PIXEL_SIZE)
	t.set_type_variation("PixelTitle", "Label")
	t.set_font("font", "PixelTitle", Palette.pixel_font(true))
	t.set_font_size("font_size", "PixelTitle", Palette.PIXEL_SIZE * 2)

	# 버튼 (아래쪽에 2px 그림자 테두리를 둬서 눌리는 느낌)
	t.set_stylebox("normal", "Button", sprite_box("res://assets/ui_btn.png"))
	t.set_stylebox("hover", "Button", sprite_box("res://assets/ui_btn.png", Color(1.08, 1.08, 1.08)))
	t.set_stylebox("pressed", "Button", sprite_box("res://assets/ui_btn_down.png"))
	t.set_stylebox("disabled", "Button", box(Palette.PANEL_HI.lightened(0.3), 8, 16, 8))
	t.set_stylebox("focus", "Button", focus_ring())
	t.set_color("font_color", "Button", Palette.TEXT)
	t.set_color("font_hover_color", "Button", Palette.TEXT)
	t.set_color("font_pressed_color", "Button", Color("#fff3e0"))
	t.set_color("font_focus_color", "Button", Palette.TEXT)
	t.set_color("font_disabled_color", "Button", Palette.DIM)

	# 실행 버튼
	t.set_type_variation("PrimaryButton", "Button")
	t.set_stylebox("normal", "PrimaryButton", _pressable(Palette.ACCENT))
	t.set_stylebox("hover", "PrimaryButton", _pressable(Palette.ACCENT.lightened(0.12)))
	t.set_stylebox("pressed", "PrimaryButton", box(Palette.ACCENT.darkened(0.15), 8, 16, 8))
	t.set_stylebox("disabled", "PrimaryButton", box(Palette.ACCENT.lightened(0.45), 8, 16, 8))
	for c in ["font_color", "font_hover_color", "font_pressed_color", "font_focus_color"]:
		t.set_color(c, "PrimaryButton", Palette.ON_ACCENT)
	t.set_color("font_disabled_color", "PrimaryButton", Palette.ON_ACCENT.lightened(0.3))

	# 테두리 버튼 (보조 동작)
	t.set_type_variation("OutlineButton", "Button")
	var outline := box(Color(0, 0, 0, 0), 8, 14, 7)
	outline.border_color = Palette.BORDER
	outline.set_border_width_all(2)
	var outline_hover := outline.duplicate()
	outline_hover.bg_color = Palette.PANEL_HI
	t.set_stylebox("normal", "OutlineButton", outline)
	t.set_stylebox("hover", "OutlineButton", outline_hover)
	t.set_stylebox("pressed", "OutlineButton", outline_hover)
	t.set_stylebox("disabled", "OutlineButton", outline)
	t.set_color("font_color", "OutlineButton", Palette.TEXT2)
	t.set_color("font_hover_color", "OutlineButton", Palette.TEXT)

	# 위험한 동작 (되돌리기 등)
	t.set_type_variation("DangerButton", "Button")
	t.set_stylebox("normal", "DangerButton", _pressable(Palette.RED_BG))
	t.set_stylebox("hover", "DangerButton", _pressable(Palette.RED_BG.darkened(0.05)))
	t.set_stylebox("pressed", "DangerButton", box(Palette.RED_BG.darkened(0.15), 8, 16, 8))
	t.set_color("font_color", "DangerButton", Palette.RED)
	t.set_color("font_hover_color", "DangerButton", Palette.RED)

	# 장·레벨 칩
	for kind in ["Chip", "ChipCurrent", "ChipDone"]:
		t.set_type_variation(kind, "Button")
	var chip := func(kind: String, bg: Color, fg: Color) -> void:
		t.set_stylebox("normal", kind, box(bg, 6, 9, 3))
		t.set_stylebox("hover", kind, box(bg.darkened(0.06), 6, 9, 3))
		t.set_stylebox("pressed", kind, box(bg.darkened(0.1), 6, 9, 3))
		t.set_color("font_color", kind, fg)
		t.set_color("font_hover_color", kind, fg)
		t.set_color("font_pressed_color", kind, fg)
		t.set_font_size("font_size", kind, Palette.PIXEL_SIZE)
	chip.call("Chip", Palette.PANEL_HI, Palette.DIM)
	chip.call("ChipCurrent", Palette.ACCENT, Palette.ON_ACCENT)
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
	var tip := box(Palette.TOOLTIP, 6, 10, 7)
	tip.border_color = Palette.BORDER
	tip.set_border_width_all(1)
	t.set_stylebox("panel", "TooltipPanel", tip)
	t.set_color("font_color", "TooltipLabel", Palette.TEXT)
	return t
