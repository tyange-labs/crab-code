## 게임 전체에서 쓰는 색과 글꼴.
class_name Palette

## 밝은 테마: 따뜻한 종이색 바탕에 짙은 갈색 글자.
const BG := Color("#efe6d2")
const PANEL := Color("#fffaf0")
const PANEL_HI := Color("#f1e7d3")
const EDITOR := Color("#f8f1e3")
const CHROME := Color("#e8dcc4")
const TEXT := Color("#2b2520")
## 보조 글자. PANEL 위 7.0:1, PANEL_HI 위 6.1:1.
const TEXT2 := Color("#5c5246")
## 라벨·경로. PANEL 위 4.9:1, EDITOR 위 4.6:1.
const TEXT3 := Color("#776b5c")
## 예전 이름 (TEXT2·TEXT3 와 같다).
const DIM := TEXT2
const FAINT := TEXT3
## 1px·2px 선.
const BORDER := Color("#d8c9ad")
## 주황(ACCENT) 버튼 위 글자. 6.2:1.
const ON_ACCENT := Color("#1b1005")
const TOOLTIP := Color("#fffaf0")
const ACCENT := Color("#f46a25")
## 글자로 쓰는 주황 (ACCENT 는 밝은 바탕 위 글자로는 흐리다). PANEL 위 4.6:1.
const ACCENT_TEXT := Color("#c2470e")
const GREEN := Color("#1e7f46")
const GREEN_BG := Color("#d9f0e0")
const RED := Color("#c2283c")
const RED_BG := Color("#f9dcdf")
const WARN := Color("#9a6700")
const SKY := Color("#1f6fbf")
const CODE := Color("#9a3f0c")
const CODE_BG := Color("#efe3cc")

const GEM := {"red": Color("#ff5c7a"), "blue": Color("#4fb3ff")}

# 코드 색칠 (레슨의 코드, 살펴보기 창)
const SYN_KEYWORD := Color("#c2410c")
const SYN_TYPE := Color("#0f766e")
const SYN_COMMENT := Color("#857a6a")
const SYN_STRING := Color("#4d7c0f")
const SYN_NUMBER := Color("#7c3aed")
const SYN_FUNC := Color("#1d4ed8")

# 인라인 코드 상자
const CODE_BOX := Color("#f0e5cf")
const CODE_BOX_HOVER := Color("#e7d9bd")
const CODE_BOX_LINE := Color("#ddcdae")
const CODE_BOX_LINE_HOVER := Color("#cfbd9a")

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
