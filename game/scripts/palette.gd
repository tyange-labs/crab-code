## 게임 전체에서 쓰는 색과 글꼴.
class_name Palette

## 어두운 SF 테마: 깊은 남색 바탕에 푸른 흰 글자, 청록 강조.
const BG := Color("#070a14")
const PANEL := Color("#0e1424")
const PANEL_HI := Color("#1a2442")
const EDITOR := Color("#0b101e")
const CHROME := Color("#0a0e1a")
const TEXT := Color("#dbe8ff")
## 보조 글자. PANEL 위 8.4:1, PANEL_HI 위 6.9:1.
const TEXT2 := Color("#9fb0d8")
## 라벨·경로. PANEL 위 5.3:1, EDITOR 위 5.6:1.
const TEXT3 := Color("#7f90ba")
## 예전 이름 (TEXT2·TEXT3 와 같다).
const DIM := TEXT2
const FAINT := TEXT3
## 1px·2px 선.
const BORDER := Color("#2b3a5e")
## 강조색(ACCENT) 버튼 위 글자.
const ON_ACCENT := Color("#04121a")
const TOOLTIP := Color("#121a30")
const ACCENT := Color("#35e0ff")
## 글자로 쓰는 강조색 (어두운 바탕에서는 ACCENT 와 같다).
const ACCENT_TEXT := Color("#35e0ff")
const GREEN := Color("#52f59a")
const GREEN_BG := Color("#0f3a2a")
const RED := Color("#ff6b7a")
const RED_BG := Color("#3a1520")
const WARN := Color("#ffb347")
const SKY := Color("#6cb8ff")
const CODE := Color("#ffcf7a")
const CODE_BG := Color("#1b2440")

const GEM := {"red": Color("#ff5c7a"), "blue": Color("#4fb3ff")}

# 코드 색칠 (레슨의 코드, 살펴보기 창)
const SYN_KEYWORD := Color("#ffa86b")
const SYN_TYPE := Color("#6fe0c0")
const SYN_COMMENT := Color("#7f90ba")
const SYN_STRING := Color("#b5e07a")
const SYN_NUMBER := Color("#d9a8ff")
const SYN_FUNC := Color("#8ec5ff")

# 인라인 코드 상자
const CODE_BOX := Color("#1a2442")
const CODE_BOX_HOVER := Color("#243258")
const CODE_BOX_LINE := Color("#2b3a5e")
const CODE_BOX_LINE_HOVER := Color("#3b4f80")

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
