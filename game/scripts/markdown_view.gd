class_name MarkdownView
## 마크다운 미리 보기.
##
## 블록: 문단(줄바꿈은 그대로), 제목(#), 목록(- / 1.), 인용(>), 코드 블록(```), 구분선(---),
##       "목표:" 로 시작하는 문단은 콜아웃 상자.
## 인라인: `코드`(``여러 개`` 도), **굵게**, *기울임*, [링크](주소), \ 이스케이프.
##
## 인라인 코드는 줄 중간에서 끊기지 않는 둥근 상자로 그리고, 누르면 클립보드에 복사한다.
## 한글은 낱자 사이에서 줄이 바뀌지 않게 어절 단위로 줄을 바꾼다 (CSS 의 word-break: keep-all).
extends Control

## 그릴 마크다운.
@export_multiline var markdown := "":
	set(value):
		if value == markdown:
			return
		markdown = value
		_blocks = _parse_blocks(value)
		_dirty = true
		queue_redraw()
		update_minimum_size()

## 본문 글자색.
var base_color := Palette.TEXT:
	set(value):
		base_color = value
		_dirty = true
		queue_redraw()
## 본문 글자 크기.
var font_size := 15
## "목표:" 문단을 콜아웃 상자로 그릴지 (아니면 "목표:" 만 강조).
var callouts := true
## 줄을 바꾸지 않고 내용 너비만큼만 차지할지 (가방 옆 정보 칩 같은 곳).
var fit_width := false
## 인라인 코드가 정의로 갈 수 있는 이름인지 알려 주는 함수 (text -> bool).
## 그런 코드는 누르면 복사 대신 code_link_clicked 를 보낸다.
var code_link := Callable()

## 정의로 갈 수 있는 인라인 코드를 눌렀을 때.
signal code_link_clicked(text: String)

const LINE_GAP := 5.0
const BLOCK_GAP := 12.0
const ITEM_GAP := 5.0
const CODE_PAD := Vector2(5, 1)
const CODE_SCALE := 0.87
const LIST_INDENT := 22.0
const BOX_PAD := Vector2(14, 11)
const WJ := "⁠" # 줄바꿈을 막는 보이지 않는 글자 (WORD JOINER)

var _blocks: Array = []
var _items: Array = [] # 배치를 마친 블록들
var _hits: Array = [] # 누를 수 있는 곳: {rect, kind: "code" | "link" | "copy", value}
var _hover := -1
var _toast := {} # 복사했다는 표시: {pos, time}
var _dirty := true
var _laid_width := -1.0
var _height := 0.0
var _natural_width := 0.0

static var _fonts := {}


func _init() -> void:
	mouse_filter = Control.MOUSE_FILTER_PASS
	size_flags_horizontal = Control.SIZE_EXPAND_FILL


func _notification(what: int) -> void:
	if what == NOTIFICATION_MOUSE_EXIT and _hover >= 0:
		_hover = -1
		queue_redraw()
	if what == NOTIFICATION_RESIZED and not fit_width and absf(size.x - _laid_width) > 0.5:
		_dirty = true
		queue_redraw()
		# 너비가 바뀌면 높이도 바뀌므로 다시 잰다.
		_relayout()
		update_minimum_size()


func _get_minimum_size() -> Vector2:
	if _dirty:
		_relayout()
	return Vector2(_natural_width if fit_width else 0.0, _height)


# ── 글꼴 ──────────────────────────────────────────────────────────

static func _font(bold: bool, italic: bool) -> Font:
	var key := int(bold) * 2 + int(italic)
	if not _fonts.has(key):
		if not bold and not italic:
			_fonts[key] = Palette.text_font()
		else:
			var f := FontVariation.new()
			f.base_font = Palette.text_font()
			if bold:
				f.variation_embolden = 0.65
			_fonts[key] = f
	return _fonts[key]


func _code_size() -> int:
	return int(round(font_size * CODE_SCALE))


# ── 블록 나누기 ───────────────────────────────────────────────────

static func _parse_blocks(src: String) -> Array:
	var blocks: Array = []
	var lines := src.replace("\r", "").split("\n")
	var para: Array = []
	var quote: Array = []
	var flush := func() -> void:
		if not para.is_empty():
			var text := "\n".join(PackedStringArray(para))
			var kind := "callout" if text.begins_with("목표:") else "para"
			blocks.append({"type": kind, "text": text})
			para.clear()
		if not quote.is_empty():
			blocks.append({"type": "quote", "text": "\n".join(PackedStringArray(quote))})
			quote.clear()
	var heading := RegEx.create_from_string("^(#{1,6})\\s+(.*)$")
	var bullet := RegEx.create_from_string("^\\s*[-*+]\\s+(.*)$")
	var ordered := RegEx.create_from_string("^\\s*(\\d+)[.)]\\s+(.*)$")
	var i := 0
	while i < lines.size():
		var line: String = lines[i]
		var t := line.strip_edges()
		if t.begins_with("```"):
			flush.call()
			var lang := t.substr(3).strip_edges()
			var body: Array = []
			i += 1
			while i < lines.size() and not lines[i].strip_edges().begins_with("```"):
				body.append(lines[i])
				i += 1
			blocks.append({"type": "code", "lang": lang, "text": "\n".join(PackedStringArray(body))})
			i += 1
			continue
		if t == "":
			flush.call()
		elif t.begins_with(">"):
			if not para.is_empty():
				flush.call()
			quote.append(t.substr(1).strip_edges(true, false))
		elif t == "---" or t == "***" or t == "___":
			flush.call()
			blocks.append({"type": "hr"})
		elif heading.search(t):
			flush.call()
			var m := heading.search(t)
			blocks.append({"type": "heading", "level": m.get_string(1).length(), "text": m.get_string(2)})
		elif bullet.search(line):
			flush.call()
			blocks.append({"type": "item", "marker": "•", "text": bullet.search(line).get_string(1)})
		elif ordered.search(line):
			flush.call()
			var m := ordered.search(line)
			blocks.append({"type": "item", "marker": m.get_string(1) + ".", "text": m.get_string(2)})
		else:
			if not quote.is_empty():
				flush.call()
			para.append(line)
		i += 1
	flush.call()
	return blocks


# ── 인라인 ───────────────────────────────────────────────────────

const ESCAPABLE := "\\`*_{}[]()#+-.!<>|~"


## 인라인 마크다운을 조각 목록으로: {text, code, bold, italic, link}
static func _parse_inline(s: String) -> Array:
	var runs: Array = []
	# 람다는 지역 변수를 값으로 잡으므로 상태는 딕셔너리(참조)에 둔다.
	var st := {"buf": "", "bold": false, "italic": false}
	var link_re := RegEx.create_from_string("^\\[([^\\]]+)\\]\\(([^)\\s]+)\\)")
	var push := func() -> void:
		if st.buf != "":
			runs.append({"text": st.buf, "code": false, "bold": st.bold, "italic": st.italic, "link": ""})
			st.buf = ""
	var i := 0
	var n := s.length()
	while i < n:
		var c := s[i]
		if c == "\\" and i + 1 < n and ESCAPABLE.contains(s[i + 1]):
			st.buf += s[i + 1]
			i += 2
			continue
		if c == "`":
			var ticks := 0
			while i + ticks < n and s[i + ticks] == "`":
				ticks += 1
			var close := _find_ticks(s, i + ticks, ticks)
			if close < 0:
				st.buf += "`".repeat(ticks)
				i += ticks
				continue
			push.call()
			var code := s.substr(i + ticks, close - i - ticks).replace("\n", " ")
			# CommonMark: 양쪽에 공백이 하나씩 있으면 하나씩 걷어 낸다 (`` `a` `` 처럼 쓰려고).
			if code.length() >= 2 and code.begins_with(" ") and code.ends_with(" ") and code.strip_edges() != "":
				code = code.substr(1, code.length() - 2)
			runs.append({"text": code, "code": true, "bold": st.bold, "italic": st.italic, "link": ""})
			i = close + ticks
			continue
		if c == "*":
			if s.substr(i, 2) == "**" and (st.bold or s.find("**", i + 2) >= 0):
				push.call()
				st.bold = not st.bold
				i += 2
				continue
			if st.italic or (i + 1 < n and s[i + 1] != " " and s.find("*", i + 1) >= 0):
				push.call()
				st.italic = not st.italic
				i += 1
				continue
		if c == "[":
			var m := link_re.search(s.substr(i))
			if m:
				push.call()
				runs.append({"text": m.get_string(1), "code": false, "bold": st.bold, "italic": st.italic, "link": m.get_string(2)})
				i += m.get_end()
				continue
		st.buf += c
		i += 1
	push.call()
	return runs


static func _find_ticks(s: String, from: int, count: int) -> int:
	var i := from
	while i < s.length():
		if s[i] == "`":
			var j := i
			while j < s.length() and s[j] == "`":
				j += 1
			if j - i == count:
				return i
			i = j
		else:
			i += 1
	return -1


static func _is_hangul(c: String) -> bool:
	if c == "":
		return false
	var u := c.unicode_at(0)
	return (u >= 0xAC00 and u <= 0xD7A3) or (u >= 0x3131 and u <= 0x318E)


## 한글 낱자 사이에 WORD JOINER 를 넣어 어절 중간에서 줄이 바뀌지 않게 한다.
static func _keep_all(text: String) -> String:
	var out := ""
	for i in text.length():
		if i > 0 and _is_hangul(text[i]) and _is_hangul(text[i - 1]):
			out += WJ
		out += text[i]
	return out


# ── 배치 ─────────────────────────────────────────────────────────

func _relayout() -> void:
	_dirty = false
	_laid_width = size.x
	_items.clear()
	_hits.clear()
	var width := -1.0 if fit_width else maxf(size.x, 40.0)
	var y := 0.0
	var prev := ""
	for b in _blocks:
		if not _items.is_empty():
			y += ITEM_GAP if prev == "item" and b.type == "item" else BLOCK_GAP
		var item := {"type": b.type, "y": y}
		match b.type:
			"para":
				item.para = _make_para(_inline_runs(b.text), width, font_size, base_color)
				item.x = 0.0
				item.h = _para_height(item.para)
			"callout":
				if callouts and width > 0:
					item.para = _make_para(_inline_runs(b.text), width - BOX_PAD.x * 2.0 - 3.0, font_size, base_color)
					item.x = BOX_PAD.x + 3.0
					item.pad_y = BOX_PAD.y
					item.h = _para_height(item.para) + BOX_PAD.y * 2.0
				else:
					item.type = "para"
					item.para = _make_para(_inline_runs(b.text), width, font_size, base_color)
					item.x = 0.0
					item.h = _para_height(item.para)
			"heading":
				var sizes := [24, 20, 17, 16, 15, 15]
				var runs := _inline_runs(b.text)
				for r in runs:
					r.bold = true
				item.para = _make_para(runs, width, sizes[b.level - 1], base_color)
				item.x = 0.0
				item.h = _para_height(item.para) + (6.0 if b.level <= 2 else 0.0)
			"item":
				item.para = _make_para(_inline_runs(b.text), width - LIST_INDENT if width > 0 else -1.0, font_size, base_color)
				item.x = LIST_INDENT
				item.marker = b.marker
				item.h = _para_height(item.para)
			"quote":
				item.para = _make_para(_inline_runs(b.text), width - 16.0 if width > 0 else -1.0, font_size, Palette.DIM)
				item.x = 16.0
				item.h = _para_height(item.para)
			"code":
				item.para = _make_code_block(b.text, width - BOX_PAD.x * 2.0 if width > 0 else -1.0)
				item.x = BOX_PAD.x
				item.pad_y = BOX_PAD.y
				item.h = _para_height(item.para) + BOX_PAD.y * 2.0
				item.text = b.text
			"hr":
				item.h = 1.0
		if item.has("para"):
			var top: float = y + item.get("pad_y", 0.0)
			item.lines = _place_lines(item.para, Vector2(item.x, top))
			_collect_hits(item)
		if b.type == "code":
			var copy := Rect2(Vector2(maxf(width, 0.0) - 54.0, y + 6.0), Vector2(46, 22))
			item.copy_rect = copy
			_hits.append({"rect": copy, "kind": "copy", "value": b.text})
		_items.append(item)
		y += item.h
		prev = b.type
	_height = ceilf(y)
	_natural_width = 0.0
	for it in _items:
		if it.has("para"):
			_natural_width = maxf(_natural_width, it.x + it.para.p.get_size().x + (BOX_PAD.x if it.type in ["callout", "code"] else 0.0))


func _inline_runs(text: String) -> Array:
	var runs := _parse_inline(text)
	# "목표:" 는 강조색 굵은 글씨로.
	if not runs.is_empty() and not runs[0].code and runs[0].text.begins_with("목표:"):
		var first: Dictionary = runs[0]
		runs[0] = {"text": first.text.substr(3), "code": false, "bold": first.bold, "italic": first.italic, "link": ""}
		runs.insert(0, {"text": "목표:", "code": false, "bold": true, "italic": false, "link": "", "color": Palette.ACCENT})
	return runs


## 조각들을 TextParagraph 하나로. 인라인 코드는 끊기지 않는 객체로 넣는다.
func _make_para(runs: Array, width: float, size_px: int, color: Color) -> Dictionary:
	var p := TextParagraph.new()
	p.width = width
	p.break_flags = TextServer.BREAK_MANDATORY | TextServer.BREAK_WORD_BOUND | TextServer.BREAK_ADAPTIVE
	var code_font := Palette.code_font()
	var code_size := int(round(size_px * CODE_SCALE))
	var ranges: Array = [] # [시작, 끝, 색, 링크 주소, 기울임]
	var objects: Array = []
	var pos := 0
	var prev_char := ""
	for idx in runs.size():
		var r: Dictionary = runs[idx]
		if r.code:
			# 코드 상자 바로 앞뒤에 붙은 한글(조사 등)과 떨어지지 않게 한다.
			if _is_hangul(prev_char):
				p.add_string(WJ, _font(false, false), size_px)
				pos += 1
			var w := code_font.get_string_size(r.text, HORIZONTAL_ALIGNMENT_LEFT, -1, code_size).x
			var box := Vector2(ceilf(w + CODE_PAD.x * 2.0), ceilf(code_font.get_height(code_size) + CODE_PAD.y * 2.0))
			p.add_object(objects.size(), box, INLINE_ALIGNMENT_BASELINE_TO | INLINE_ALIGNMENT_TO_BASELINE, 1, CODE_PAD.y + code_font.get_ascent(code_size))
			objects.append({"text": r.text, "size": code_size})
			pos += 1
			prev_char = "`"
			if idx + 1 < runs.size() and not runs[idx + 1].code and _is_hangul(runs[idx + 1].text.left(1)):
				p.add_string(WJ, _font(false, false), size_px)
				pos += 1
		else:
			var text := _keep_all(r.text)
			if text == "":
				continue
			var c: Color = r.get("color", Palette.SKY if r.link != "" else color)
			p.add_string(text, _font(r.bold, r.italic), size_px)
			ranges.append([pos, pos + text.length(), c, r.link, r.italic])
			pos += text.length()
			prev_char = r.text.right(1)
	return {"p": p, "ranges": ranges, "objects": objects, "mono": false, "base": color}


## 코드 블록: 고정폭 글꼴에 간단한 Rust 문법 색칠.
func _make_code_block(text: String, width: float) -> Dictionary:
	var p := TextParagraph.new()
	p.width = width
	p.break_flags = TextServer.BREAK_MANDATORY | TextServer.BREAK_WORD_BOUND | TextServer.BREAK_ADAPTIVE
	var size_px := _code_size()
	p.add_string(text, Palette.code_font(), size_px)
	return {"p": p, "ranges": _rust_tokens(text), "objects": [], "mono": true, "base": Palette.TEXT}


const RUST_KEYWORDS := [
	"as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for", "if", "impl", "in",
	"let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
	"super", "trait", "type", "unsafe", "use", "where", "while", "dyn", "true", "false",
]
const RUST_TYPES := [
	"i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64",
	"bool", "char", "str", "String", "Vec", "Option", "Result", "Some", "None", "Ok", "Err", "Box",
]
const COL_KEYWORD := Color("#ff8f5a")
const COL_TYPE := Color("#6fe0c0")
const COL_COMMENT := Color("#717791")
const COL_STRING := Color("#b5e07a")
const COL_NUMBER := Color("#d9a8ff")
const COL_FUNC := Color("#8ec5ff")


static func _rust_tokens(text: String) -> Array:
	var ranges: Array = []
	var re := RegEx.create_from_string("//[^\\n]*|\"(?:[^\"\\\\]|\\\\.)*\"|\\b\\d[\\d_]*(?:\\.\\d+)?\\b|[A-Za-z_][A-Za-z0-9_]*!?")
	for m in re.search_all(text):
		var tok := m.get_string()
		var c := Color(0, 0, 0, 0)
		if tok.begins_with("//"):
			c = COL_COMMENT
		elif tok.begins_with("\""):
			c = COL_STRING
		elif tok[0].is_valid_int():
			c = COL_NUMBER
		elif tok in RUST_KEYWORDS:
			c = COL_KEYWORD
		elif tok in RUST_TYPES or tok[0] == tok[0].to_upper():
			c = COL_TYPE
		elif tok.ends_with("!") or text.substr(m.get_end(), 1) == "(":
			c = COL_FUNC
		if c.a > 0:
			ranges.append([m.get_start(), m.get_end(), c, "", false])
	return ranges


func _para_height(para: Dictionary) -> float:
	var p: TextParagraph = para.p
	var h := 0.0
	for i in p.get_line_count():
		h += p.get_line_size(i).y
	return h + LINE_GAP * maxf(p.get_line_count() - 1, 0)


## 줄마다 위치와 줄 안의 코드 상자 위치를 미리 계산해 둔다.
func _place_lines(para: Dictionary, origin: Vector2) -> Array:
	var ts := TextServerManager.get_primary_interface()
	var p: TextParagraph = para.p
	var lines: Array = []
	var y := origin.y
	for i in p.get_line_count():
		var rid := p.get_line_rid(i)
		var asc := p.get_line_ascent(i)
		var boxes: Array = []
		for key in p.get_line_objects(i):
			var r := ts.shaped_text_get_object_rect(rid, key)
			r.position += Vector2(origin.x, y + asc)
			boxes.append({"rect": r, "key": key})
		lines.append({"rid": rid, "pos": Vector2(origin.x, y), "asc": asc, "boxes": boxes})
		y += p.get_line_size(i).y + LINE_GAP
	return lines


func _collect_hits(item: Dictionary) -> void:
	var ts := TextServerManager.get_primary_interface()
	for line in item.lines:
		for b in line.boxes:
			_hits.append({"rect": b.rect, "kind": "code", "value": item.para.objects[b.key].text})
		for r in item.para.ranges:
			if r[3] == "":
				continue
			var sel := ts.shaped_text_get_selection(line.rid, r[0], r[1])
			for k in range(0, sel.size()):
				var span: Vector2 = sel[k]
				var rect := Rect2(line.pos + Vector2(span.x, 0), Vector2(span.y - span.x, line.asc + 4.0))
				_hits.append({"rect": rect, "kind": "link", "value": r[3]})


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	if _dirty or (not fit_width and absf(size.x - _laid_width) > 0.5):
		_relayout()
	for item in _items:
		match item.type:
			"callout":
				var rect := Rect2(0, item.y, size.x, item.h)
				draw_style_box(UiTheme.box(Color(Palette.ACCENT, 0.09), 8), rect)
				draw_rect(Rect2(0, item.y, 3, item.h), Palette.ACCENT)
			"code":
				var rect := Rect2(0, item.y, size.x, item.h)
				var sb := UiTheme.box(Palette.EDITOR, 8)
				sb.border_color = Color("#2d3150")
				sb.set_border_width_all(1)
				draw_style_box(sb, rect)
				var hovered: bool = _hover >= 0 and _hits[_hover].kind == "copy" and _hits[_hover].rect == item.copy_rect
				draw_style_box(UiTheme.box(Palette.PANEL_HI.lightened(0.1 if hovered else 0.0), 5), item.copy_rect)
				_draw_centered("복사", item.copy_rect, 12, Palette.DIM if not hovered else Palette.TEXT)
			"quote":
				draw_rect(Rect2(0, item.y, 3, item.h), Palette.FAINT)
			"hr":
				draw_rect(Rect2(0, item.y, size.x, 1), Palette.PANEL_HI)
			"item":
				var first: Dictionary = item.lines[0]
				var marker_color := Palette.ACCENT if item.marker == "•" else Palette.DIM
				draw_string(_font(false, false), Vector2(4, first.pos.y + first.asc), item.marker, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, marker_color)
		if item.has("para"):
			_draw_para(item)
	if not _toast.is_empty():
		var t: float = (Time.get_ticks_msec() - _toast.time) / 1000.0
		var a := clampf(1.6 - t * 1.6, 0.0, 1.0)
		var r := Rect2(_toast.pos - Vector2(36, 30 + t * 8.0), Vector2(72, 22))
		draw_style_box(UiTheme.box(Color(Palette.GREEN_BG, a), 5), r)
		_draw_centered("복사했어요", r, 12, Color(Palette.GREEN, a))


func _draw_centered(text: String, rect: Rect2, size_px: int, color: Color) -> void:
	var f := _font(false, false)
	var w := f.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, size_px).x
	var base := rect.position.y + (rect.size.y + f.get_ascent(size_px) - f.get_descent(size_px)) / 2.0
	draw_string(f, Vector2(rect.position.x + (rect.size.x - w) / 2.0, base), text, HORIZONTAL_ALIGNMENT_LEFT, -1, size_px, color)


func _draw_para(item: Dictionary) -> void:
	var ts := TextServerManager.get_primary_interface()
	var para: Dictionary = item.para
	var ci := get_canvas_item()
	for line in item.lines:
		# 코드 상자 먼저
		for b in line.boxes:
			var obj: Dictionary = para.objects[b.key]
			var hovered: bool = _hover >= 0 and _hits[_hover].kind == "code" and _hits[_hover].rect == b.rect
			var sb := UiTheme.box(Color("#3a3f5c") if hovered else Color("#2f3349"), 5)
			sb.border_color = Color("#454a6b") if hovered else Color("#3a3e58")
			sb.set_border_width_all(1)
			draw_style_box(sb, b.rect)
			var cf := Palette.code_font()
			var base: float = b.rect.position.y + CODE_PAD.y + cf.get_ascent(obj.size)
			draw_string(cf, Vector2(b.rect.position.x + CODE_PAD.x, base), obj.text, HORIZONTAL_ALIGNMENT_LEFT, -1, obj.size, Palette.CODE)
			if _is_code_link(obj.text):
				# 정의로 갈 수 있는 코드는 점선 밑줄
				var uy := base + 3.0
				var ux: float = b.rect.position.x + CODE_PAD.x
				while ux < b.rect.end.x - CODE_PAD.x:
					draw_line(Vector2(ux, uy), Vector2(minf(ux + 3.0, b.rect.end.x - CODE_PAD.x), uy), Color(Palette.SKY, 0.8 if hovered else 0.45), 1.0)
					ux += 5.0
		# 글자
		var x: float = line.pos.x
		var baseline: float = line.pos.y + line.asc
		# 기울임: 기준선을 축으로 글자를 비스듬히 민다 (x' = x + 0.2 * (기준선 - y)).
		var skew := Transform2D(Vector2(1, 0), Vector2(-0.2, 1), Vector2(0.2 * baseline, 0))
		for g in ts.shaped_text_get_glyphs(line.rid):
			var r = _range_at(para, g.start)
			var color: Color = r[2] if r != null else para.base
			var italic: bool = r != null and r[4]
			for _i in g.repeat:
				if g.font_rid.is_valid() and (g.flags & TextServer.GRAPHEME_IS_VIRTUAL) == 0:
					if italic:
						draw_set_transform_matrix(skew)
					ts.font_draw_glyph(g.font_rid, ci, g.font_size, Vector2(x, baseline) + g.offset, g.index, color)
					if italic:
						draw_set_transform_matrix(Transform2D.IDENTITY)
				x += g.advance
		# 링크 밑줄
		for r in para.ranges:
			if r[3] == "":
				continue
			for span in ts.shaped_text_get_selection(line.rid, r[0], r[1]):
				draw_line(Vector2(line.pos.x + span.x, baseline + 3), Vector2(line.pos.x + span.y, baseline + 3), Color(Palette.SKY, 0.6), 1.0)


func _is_code_link(text: String) -> bool:
	return code_link.is_valid() and code_link.call(text)


func _range_at(para: Dictionary, index: int):
	for r in para.ranges:
		if index >= r[0] and index < r[1]:
			return r
	return null


# ── 마우스 ───────────────────────────────────────────────────────

func _hit_at(pos: Vector2) -> int:
	for i in _hits.size():
		if _hits[i].rect.grow(1.0).has_point(pos):
			return i
	return -1


func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		var h := _hit_at(event.position)
		if h != _hover:
			_hover = h
			mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND if h >= 0 else Control.CURSOR_ARROW
			queue_redraw()
	elif event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		var h := _hit_at(event.position)
		if h < 0:
			return
		var hit: Dictionary = _hits[h]
		if hit.kind == "link":
			OS.shell_open(hit.value)
		elif hit.kind == "code" and _is_code_link(hit.value):
			code_link_clicked.emit(hit.value)
		else:
			DisplayServer.clipboard_set(hit.value)
			_toast = {"pos": hit.rect.get_center() if hit.kind == "code" else hit.rect.position + Vector2(hit.rect.size.x / 2.0, 0), "time": Time.get_ticks_msec()}
			set_process(true)
		accept_event()


func _process(_delta: float) -> void:
	if _toast.is_empty():
		set_process(false)
		return
	if Time.get_ticks_msec() - _toast.time > 1100:
		_toast = {}
	queue_redraw()


func _get_tooltip(at: Vector2) -> String:
	var h := _hit_at(at)
	if h < 0:
		return ""
	match _hits[h].kind:
		"code": return "눌러서 정의 보기 (crab.rs)" if _is_code_link(_hits[h].value) else "눌러서 복사"
		"link": return _hits[h].value
		"copy": return "코드 블록 복사"
	return ""
