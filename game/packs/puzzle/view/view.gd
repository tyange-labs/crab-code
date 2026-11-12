## 퍼즐 게임팩의 화면: 격자 위의 꽃게 한 마리와 보석·받침대·문·표지판을 그린다.
##
## 게임 본체(host.gd)와의 약속 (모든 게임팩 화면이 같다):
##   setup(map, frames)     게임팩 프로그램이 출력한 지도와 행동마다의 모습
##   time                   틱(행동) 단위 실수. 2.5 면 두 번째와 세 번째 행동 사이 (보간한다)
##   selected               고른 행위자 번호
##   last_tick(), frame_at(t)
##   play_effects(t)        재생 중 t 번째 행동에 들어설 때 효과음·불꽃
##   celebrate()            성공했을 때
##   agent_count()          행위자 수
##   describe_agent(i, t)   t 의 i 번 행위자를 한 줄로 (살펴보기 창에 쓴다)
##   signal agent_selected  행위자를 눌렀을 때
extends Control

signal agent_selected(index: int)

const DX := [1, 0, -1, 0]
const DY := [0, 1, 0, -1]
const DIR_NAME := ["동", "남", "서", "북"]
const RING := {"collect": Color("#f2c94c"), "deposit": Color("#5fd38d"), "polish": Color.WHITE, "unlock": Color("#5cb8ff")}

var map: Dictionary = {}
var frames: Array = []
var time := 0.0
var selected := 0

var _clock := 0.0
var _shake := 0.0
var _particles: Array = []
var _tile := 1.0
var _origin := Vector2.ZERO


func _init() -> void:
	mouse_filter = Control.MOUSE_FILTER_STOP
	clip_contents = true


func setup(new_map: Dictionary, new_frames: Array) -> void:
	map = new_map
	frames = new_frames
	time = 0.0
	_particles.clear()
	queue_redraw()


func last_tick() -> int:
	return maxi(frames.size() - 1, 0)


func frame_at(t: int) -> Dictionary:
	if frames.is_empty():
		return {}
	return frames[clampi(t, 0, frames.size() - 1)]


func agent_count() -> int:
	return 1


func describe_agent(_i: int, t: int) -> String:
	var f := frame_at(t)
	if f.is_empty():
		return ""
	var w: Dictionary = f.world
	var held: int = w.gems.filter(func(g): return g.held).size()
	return "(%d, %d) · %s쪽을 봄%s" % [w.x, w.y, DIR_NAME[w.dir], " · 가방에 보석 %d개" % held if held > 0 else ""]


func play_effects(t: int) -> void:
	var f := frame_at(t)
	if f.is_empty():
		return
	var w: Dictionary = f.world
	var here := _cell_center(Vector2(w.x, w.y))
	if f.bump:
		_shake = 1.0
		Sfx.play("bump")
		return
	match f.action:
		"forward":
			Sfx.play("step", randf_range(0.95, 1.05))
		"left", "right":
			Sfx.play("turn")
		"collect":
			Sfx.play("collect")
			_burst(here, Palette.WARN, 18)
		"deposit":
			Sfx.play("deposit")
			_burst(here, Palette.GREEN, 22)
		"polish":
			Sfx.play("polish")
			_burst(here, Color.WHITE, 14)
		"unlock":
			Sfx.play("unlock")
			if w.door != null:
				_burst(_cell_center(Vector2(w.door.x, w.door.y)), Palette.SKY, 26)


func celebrate() -> void:
	var colors := [Color("#f46a25"), Color("#5fd38d"), Color("#5cb8ff"), Color("#f2c94c"), Color("#ff5c7a")]
	var c := Vector2(size.x / 2.0, size.y * 0.45)
	for i in 110:
		var a := i * 2.399
		var sp := 140.0 + float((i * 37) % 100) * 2.8
		_particles.append({
			"pos": c, "vel": Vector2(cos(a), sin(a)) * sp + Vector2(0, -200),
			"life": 1.3 + float((i * 13) % 10) * 0.08, "color": colors[i % 5], "size": 7.0, "gravity": 560.0,
		})


func _burst(at: Vector2, color: Color, count: int) -> void:
	for i in count:
		var a := randf() * TAU
		var sp := randf_range(0.6, 2.2) * _tile
		_particles.append({
			"pos": at, "vel": Vector2(cos(a), sin(a)) * sp, "life": randf_range(0.35, 0.7),
			"color": color, "size": _tile * randf_range(0.04, 0.08), "gravity": _tile * 2.0,
		})


func _process(delta: float) -> void:
	delta = minf(delta, 0.05)
	_clock += delta
	_shake = maxf(_shake - delta * 2.5, 0.0)
	for p in _particles:
		p.vel.y += p.gravity * delta
		p.pos += p.vel * delta
		p.life -= delta
	_particles = _particles.filter(func(p): return p.life > 0.0)
	queue_redraw()


# ── 좌표 ─────────────────────────────────────────────────────────

func _is_wall(w: Dictionary, x: int, y: int) -> bool:
	return x < 0 or y < 0 or x >= w.w or y >= w.h or w.walls[y * w.w + x]


func _layout(w: Dictionary) -> void:
	# 바닥이 있는 범위만 화면에 맞춘다.
	var lo := Vector2i(w.w, w.h)
	var hi := Vector2i(0, 0)
	for y in w.h:
		for x in w.w:
			if not _is_wall(w, x, y):
				lo = Vector2i(mini(lo.x, x), mini(lo.y, y))
				hi = Vector2i(maxi(hi.x, x), maxi(hi.y, y))
	var cols := hi.x - lo.x + 1
	var rows := hi.y - lo.y + 1
	var top := 40.0
	_tile = minf(minf((size.x - 24.0) / cols, (size.y - top - 16.0) / rows), 92.0)
	_origin = Vector2((size.x - _tile * cols) / 2.0 - lo.x * _tile, top + (size.y - top - _tile * rows) / 2.0 - lo.y * _tile)


func _cell_center(cell: Vector2) -> Vector2:
	return _origin + (cell + Vector2(0.5, 0.5)) * _tile


func _rect(c: Vector2, half: float, radius: float, color: Color) -> void:
	var sb := StyleBoxFlat.new()
	sb.bg_color = color
	sb.set_corner_radius_all(int(radius))
	sb.anti_aliasing = true
	draw_style_box(sb, Rect2(c - Vector2(half, half), Vector2(half, half) * 2.0))


func _label(text: String, c: Vector2, font_size: float, color: Color, font: Font = null) -> void:
	if font == null:
		font = Palette.text_font()
	var fs := maxi(int(font_size), 1)
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
	var baseline := c.y + (font.get_ascent(fs) - font.get_descent(fs)) / 2.0
	draw_string(font, Vector2(c.x - w / 2.0, baseline), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color("#1d2031"))
	if frames.is_empty() or size.x < 60.0 or size.y < 60.0:
		return
	var k := int(floor(time))
	var f := clampf(time - k, 0.0, 1.0)
	var a: Dictionary = frame_at(k)
	var b: Dictionary = frame_at(k + 1) if k + 1 < frames.size() else a
	var moving := b != a and f > 0.0
	# 물건(보석·문)은 행동 중간쯤에 바뀐 모습으로 넘어간다.
	var w: Dictionary = b.world if moving and f >= 0.5 else a.world
	_layout(a.world)
	if _tile < 6.0:
		return
	var tile := _tile
	var shake := Vector2(sin(_clock * 90.0), cos(_clock * 77.0)) * _shake * tile * 0.06
	draw_set_transform(shake)

	for y in w.h:
		for x in w.w:
			if not _is_wall(w, x, y):
				_rect(_cell_center(Vector2(x, y)), tile / 2.0 - 2.0, tile * 0.14, Color("#363b58") if (x + y) % 2 == 0 else Color("#323651"))

	for n in w.numbers:
		var c := _cell_center(Vector2(n.x, n.y))
		_rect(c, tile * 0.3, tile * 0.1, Color("#27405e"))
		_label(str(n.n), c, tile * 0.4, Palette.SKY, Palette.code_font())

	for s in w.signs:
		if s.sign != "left" and s.sign != "right":
			continue
		var c := _cell_center(Vector2(s.x, s.y))
		var sb := StyleBoxFlat.new()
		sb.bg_color = Palette.WARN
		sb.set_corner_radius_all(int(tile * 0.08))
		draw_style_box(sb, Rect2(c - Vector2(tile * 0.4, tile * 0.2), Vector2(tile * 0.8, tile * 0.4)))
		_label("왼쪽" if s.sign == "left" else "오른쪽", c, tile * 0.23, Color("#3a2c00"))

	if w.flag != null:
		var fc := _cell_center(Vector2(w.flag.x, w.flag.y))
		var px := fc.x - tile * 0.16
		draw_line(Vector2(px, fc.y - tile * 0.32), Vector2(px, fc.y + tile * 0.32), Palette.TEXT, tile * 0.05, true)
		var wave := sin(_clock * 4.0) * tile * 0.03
		draw_colored_polygon(PackedVector2Array([
			Vector2(px, fc.y - tile * 0.32), Vector2(px + tile * 0.4, fc.y - tile * 0.17 + wave), Vector2(px, fc.y - tile * 0.02),
		]), Palette.GREEN)

	for p in w.pedestals:
		var c := _cell_center(Vector2(p.x, p.y))
		var ring: Color = Palette.GEM[p.want] if p.want != null else Color("#c9cde0")
		_rect(c, tile * 0.36, tile * 0.12, ring.lerp(Color.BLACK, 0.2))
		_rect(c, tile * 0.29, tile * 0.09, Color("#23263a"))

	if w.door != null:
		var d := _cell_center(Vector2(w.door.x, w.door.y))
		if w.door.open:
			draw_rect(Rect2(d - Vector2(tile * 0.38, tile * 0.38), Vector2(tile * 0.76, tile * 0.76)), Color("#8a5a2b"), false, tile * 0.07)
		else:
			_rect(d, tile * 0.42, tile * 0.1, Color("#8a5a2b"))
			_rect(d, tile * 0.34, tile * 0.07, Color("#a9713a"))
			draw_circle(d + Vector2(0, -tile * 0.04), tile * 0.11, Palette.WARN, true, -1.0, true)
			draw_rect(Rect2(d + Vector2(-tile * 0.03, 0), Vector2(tile * 0.06, tile * 0.16)), Palette.WARN)

	for g in w.gems:
		if g.held:
			continue
		var bob := 0.0 if g.placed else sin(_clock * 2.6 + g.x) * tile * 0.03
		var c := _cell_center(Vector2(g.x, g.y)) + Vector2(0, bob)
		CrabArt.draw_gem(self, c, tile * 0.24, g)
		if w.showWeights:
			_label(str(g.weight), c + Vector2(0, tile * 0.01), tile * 0.2, Color("#1a1a22"), Palette.code_font())

	_draw_crab(a, b, f, moving)

	for p in _particles:
		var lk := clampf(p.life, 0.0, 1.0)
		draw_circle(p.pos, p.size * (0.6 + lk), Color(p.color, lk), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)
	_draw_hud(w, k if not moving or f < 0.5 else k + 1)


func _draw_crab(a: Dictionary, b: Dictionary, f: float, moving: bool) -> void:
	var tile := _tile
	var wa: Dictionary = a.world
	var wb: Dictionary = b.world
	var pos := Vector2(wa.x, wa.y)
	var angle: float = wa.dir * PI / 2.0
	var walk := 0.0
	var hop := 0.0
	var e := f * f * (3.0 - 2.0 * f)
	if moving and not b.bump:
		pos = pos.lerp(Vector2(wb.x, wb.y), e)
		var delta: int = ((wb.dir - wa.dir + 6) % 4) - 2
		angle = (wa.dir + delta * e) * PI / 2.0
		if b.action == "forward":
			walk = f
			hop = sin(f * PI) * 0.06
	elif moving and b.bump and b.action == "forward":
		pos += Vector2(DX[wa.dir], DY[wa.dir]) * sin(PI * f) * 0.22
	var c := _cell_center(pos) - Vector2(0, hop * tile)
	draw_set_transform(c + Vector2(0, tile * 0.3), 0.0, Vector2(1.0, 0.35))
	draw_circle(Vector2.ZERO, tile * 0.3, Color(0, 0, 0, 0.25), true, -1.0, true)
	draw_set_transform(Vector2(sin(_clock * 90.0), cos(_clock * 77.0)) * _shake * tile * 0.06)
	CrabArt.draw_crab(self, c, tile, angle, walk, 0.0)
	if moving and RING.has(b.action) and not b.bump:
		draw_arc(c, tile * (0.3 + 0.35 * f), 0.0, TAU, 40, Color(RING[b.action], 1.0 - f), tile * 0.05, true)
	elif not moving:
		# 꽃게가 받침대 위에 서 있으면 올려 둔 보석이 가려지므로 위에 다시 그린다.
		for g in wa.gems:
			if g.placed and g.x == wa.x and g.y == wa.y:
				CrabArt.draw_gem(self, c - Vector2(0, tile * 0.3), tile * 0.17, g)


## 위쪽 정보 줄: 행동 수, 가방, 다이얼·두루마리·쪽지.
func _draw_hud(w: Dictionary, step: int) -> void:
	var font := Palette.text_font()
	var x := 12.0
	x = _chip(x, "행동 %d / %d" % [step, last_tick()], Palette.TEXT, font)
	var held: Array = w.gems.filter(func(g): return g.held)
	var bag_w := 52.0 + maxf(held.size(), 1) * 18.0
	_chip_box(x, bag_w)
	_text(x + 10, "가방", Palette.DIM, font)
	if held.is_empty():
		_text(x + 42, "-", Palette.FAINT, font)
	for i in held.size():
		CrabArt.draw_gem(self, Vector2(x + 50 + i * 18.0, 20), 7.0, held[i])
	x += bag_w + 6.0
	if w.dial != null:
		x = _chip(x, "다이얼 %d" % w.dial, Palette.CODE, font)
	if not w.scroll.is_empty():
		x = _chip(x, "두루마리 [%s]" % ", ".join(PackedStringArray(w.scroll.map(func(n): return str(n)))), Palette.CODE, font)
	if w.note != null:
		x = _chip(x, "쪽지 %s" % JSON.stringify(w.note), Palette.CODE, font)
	if w.note2 != null:
		x = _chip(x, "쪽지 2 %s" % JSON.stringify(w.note2), Palette.CODE, font)


func _chip_box(x: float, w: float) -> void:
	var sb := StyleBoxFlat.new()
	sb.bg_color = Color(0, 0, 0, 0.3)
	sb.set_corner_radius_all(12)
	draw_style_box(sb, Rect2(x, 8, w, 24))


func _text(x: float, text: String, color: Color, font: Font) -> void:
	draw_string(font, Vector2(x, 8 + 12 + (font.get_ascent(13) - font.get_descent(13)) / 2.0), text, HORIZONTAL_ALIGNMENT_LEFT, -1, 13, color)


func _chip(x: float, text: String, color: Color, font: Font) -> float:
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, 13).x + 20.0
	_chip_box(x, w)
	_text(x + 10, text, color, font)
	return x + w + 6.0


func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		agent_selected.emit(0)
		accept_event()
