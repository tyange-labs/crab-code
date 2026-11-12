## 게임 전체에서 쓰는 색과 글꼴.
class_name Palette

const BG := Color("#151722")
const PANEL := Color("#202334")
const PANEL_HI := Color("#2c3049")
const EDITOR := Color("#1b1d2c")
const CHROME := Color("#171927")
const TEXT := Color("#e4e6f0")
## 보조 글자. PANEL 위 6.0:1, PANEL_HI 위 5.0:1.
const TEXT2 := Color("#9aa0ba")
## 라벨·경로. PANEL 위 4.3:1, EDITOR 위 4.6:1. PANEL_HI 위에는 쓰지 않는다.
const TEXT3 := Color("#7f85a3")
## 예전 이름 (대비를 높인 TEXT2·TEXT3 와 같다).
const DIM := TEXT2
const FAINT := TEXT3
## 1px·2px 선.
const BORDER := Color("#2f334d")
## 주황(ACCENT) 버튼 위 글자. 6.2:1.
const ON_ACCENT := Color("#1b1005")
const TOOLTIP := Color("#23263b")
const ACCENT := Color("#f46a25")
const GREEN := Color("#5fd38d")
const GREEN_BG := Color("#1f4a35")
const RED := Color("#ff6b7a")
const RED_BG := Color("#4b2233")
const WARN := Color("#f2c94c")
const SKY := Color("#5cb8ff")
const CODE := Color("#ffc66d")
const CODE_BG := Color("#33374f")

const GEM := {"red": Color("#ff5c7a"), "blue": Color("#4fb3ff")}

# 코드 색칠 (레슨의 코드, 살펴보기 창)
const SYN_KEYWORD := Color("#ff8f5a")
const SYN_TYPE := Color("#6fe0c0")
const SYN_COMMENT := Color("#7f85a3")
const SYN_STRING := Color("#b5e07a")
const SYN_NUMBER := Color("#d9a8ff")
const SYN_FUNC := Color("#8ec5ff")

# 인라인 코드 상자
const CODE_BOX := Color("#2f3349")
const CODE_BOX_HOVER := Color("#3a3f5c")
const CODE_BOX_LINE := Color("#3a3e58")
const CODE_BOX_LINE_HOVER := Color("#454a6b")

# 글자 크기 (본문 Pretendard / 코드 JetBrains Mono / 픽셀 갈무리11)
const SIZE_BODY := 15
const SIZE_SMALL := 13
const SIZE_CODE := 13
const SIZE_PIXEL_TITLE := 24

# 간격 (4의 배수)
const GAP := 8
const PAD := 16


## BBCode [color=…] 에 넣을 "#rrggbb".
static func hex(c: Color) -> String:
	return "#" + c.to_html(false)

## 픽셀 폰트(갈무리11)는 12px 격자로 만들어졌다. 이 크기의 배수로 써야 선명하다.
const PIXEL_SIZE := 12

static var _text_font: Font
static var _pixel_font: Font
static var _pixel_bold: Font
static var _code_font: Font


## 본문 글꼴 (Pretendard).
static func text_font() -> Font:
	if _text_font == null:
		_text_font = load("res://fonts/Pretendard-Regular.ttf")
	return _text_font


## 코드 글꼴 (JetBrains Mono, 한글은 Pretendard 로).
static func code_font() -> Font:
	if _code_font == null:
		var mono: FontFile = load("res://fonts/JetBrainsMono-Regular.ttf").duplicate()
		mono.fallbacks = [text_font()]
		_code_font = mono
	return _code_font


## 픽셀 폰트 (갈무리11, OFL). 제목·HUD·버튼에 쓴다. 긴 글에는 text_font 를 쓴다.
static func pixel_font(bold := false) -> Font:
	if bold:
		if _pixel_bold == null:
			_pixel_bold = _make_pixel("res://fonts/Galmuri11-Bold.ttf")
		return _pixel_bold
	if _pixel_font == null:
		_pixel_font = _make_pixel("res://fonts/Galmuri11.ttf")
	return _pixel_font


static func _make_pixel(path: String) -> Font:
	var f: FontFile = load(path).duplicate()
	f.antialiasing = TextServer.FONT_ANTIALIASING_NONE
	f.hinting = TextServer.HINTING_NONE
	f.subpixel_positioning = TextServer.SUBPIXEL_POSITIONING_DISABLED
	f.fixed_size = 0
	f.fallbacks = [text_font()]
	return f
