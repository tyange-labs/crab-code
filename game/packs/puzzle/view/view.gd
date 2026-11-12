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
##
## 그림은 assets/sheet.png (../art/sprites.py 가 만든다). 16픽셀 격자에 그린 뒤 정수 배율로 키운다.
extends Control

# HUD 글자색: 어두운 세계 위에 그리므로 UI 테마(Palette)와 따로 둔다.
const HUD_TEXT := Color("#e4e6f0")
const HUD_DIM := Color("#9aa0ba")
const HUD_FAINT := Color("#7f85a3")
const HUD_CODE := Color("#ffc66d")
const HUD_WARN := Color("#f2c94c")
const HUD_SKY := Color("#5cb8ff")
const HUD_GREEN := Color("#5fd38d")


signal agent_selected(index: int)

const DX := [1, 0, -1, 0]
const DY := [0, 1, 0, -1]
const DIR_NAME := ["동", "남", "서", "북"]
const RING := {"collect": Color("#f2c94c"), "deposit": Color("#5fd38d"), "polish": Color.WHITE, "unlock": Color("#5cb8ff")}
const T := 16
# sheet.png 안의 순서
enum Sprite { FLOOR0, FLOOR1, GEM_RED, GEM_BLUE, GEM_DULL, PED, PED_RED, PED_BLUE, DOOR, DOOR_OPEN, FLAG0, FLAG1, SIGN_LEFT, SIGN_RIGHT, NUMBER, CRAB0, CRAB1 }

var map: Dictionary = {}
var frames: Array = []
var time := 0.0
var selected := 0

var _clock := 0.0
var _shake := 0.0
var _particles: Array = []
var _tile := 1.0 # 화면에서 한 칸의 크기 (= T * _scale)
var _scale := 1
var _origin := Vector2.ZERO
var _sheet: Texture2D = load("res://packs/puzzle/view/assets/sheet.png")


func _init() -> void:
	mouse_filter = Control.MOUSE_FILTER_STOP
	clip_contents = true
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST


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
			_burst(here, HUD_WARN, 18)
		"deposit":
			Sfx.play("deposit")
			_burst(here, HUD_GREEN, 22)
		"polish":
			Sfx.play("polish")
			_burst(here, Color.WHITE, 14)
		"unlock":
			Sfx.play("unlock")
			if w.door != null:
				_burst(_cell_center(Vector2(w.door.x, w.door.y)), HUD_SKY, 26)


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
	# 위쪽 정보 줄은 지도 모서리 위에 겹쳐 그린다 (그래야 정수 배율을 한 단계 크게 쓸 수 있다).
	var top := 8.0
	_scale = maxi(1, int(floor(minf((size.x - 24.0) / (cols * T), (size.y - top - 8.0) / (rows * T)))))
	_tile = T * _scale
	_origin = Vector2(floor((size.x - _tile * cols) / 2.0 - lo.x * _tile), floor(top + (size.y - top - _tile * rows) / 2.0 - lo.y * _tile))


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
	draw_string(font, Vector2(round(c.x - w / 2.0), round(baseline)), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color("#141625"))
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
	var shake := (Vector2(sin(_clock * 90.0), cos(_clock * 77.0)) * _shake * 2.0).round() * _scale
	draw_set_transform(shake)

	for y in w.h:
		for x in w.w:
			if not _is_wall(w, x, y):
				_sprite(Sprite.FLOOR0 + (x + y) % 2, Vector2(x, y) * T)

	var digits := Palette.pixel_font(true)
	var fs := Palette.PIXEL_SIZE * maxi(1, _scale / 2)
	for n in w.numbers:
		_sprite(Sprite.NUMBER, Vector2(n.x, n.y) * T)
		_label(str(n.n), _cell_center(Vector2(n.x, n.y)), fs, HUD_SKY, digits)

	for s in w.signs:
		if s.sign == "left":
			_sprite(Sprite.SIGN_LEFT, Vector2(s.x, s.y) * T)
		elif s.sign == "right":
			_sprite(Sprite.SIGN_RIGHT, Vector2(s.x, s.y) * T)

	if w.flag != null:
		_sprite(Sprite.FLAG0 + int(_clock * 3.0) % 2, Vector2(w.flag.x, w.flag.y) * T)

	for p in w.pedestals:
		var kind := Sprite.PED
		if p.want == "red":
			kind = Sprite.PED_RED
		elif p.want == "blue":
			kind = Sprite.PED_BLUE
		_sprite(kind, Vector2(p.x, p.y) * T)

	if w.door != null:
		_sprite(Sprite.DOOR_OPEN if w.door.open else Sprite.DOOR, Vector2(w.door.x, w.door.y) * T)

	for g in w.gems:
		if g.held:
			continue
		var bob: float = 0.0 if g.placed else roundf(sin(_clock * 2.6 + g.x))
		_sprite(_gem_sprite(g), Vector2(g.x, g.y) * T + Vector2(0, bob))
		if w.showWeights:
			_label(str(g.weight), _cell_center(Vector2(g.x, g.y)) + Vector2(0, bob * _scale), fs, Color("#1a1a22"), digits)

	_draw_crab(a, b, f, moving)

	for p in _particles:
		var lk := clampf(p.life, 0.0, 1.0)
		var px := float(_scale) * (2.0 if p.size > _tile * 0.06 else 1.0)
		draw_rect(Rect2(p.pos.round(), Vector2(px, px)), Color(p.color, lk))
	draw_set_transform(Vector2.ZERO)
	_draw_hud(w, k if not moving or f < 0.5 else k + 1)


func _gem_sprite(g: Dictionary) -> int:
	if not g.shiny:
		return Sprite.GEM_DULL
	return Sprite.GEM_RED if g.color == "red" else Sprite.GEM_BLUE


## 시트의 그림 하나를 그림 픽셀 좌표(지도 왼쪽 위 기준)에 그린다. quarter 만큼 90도씩 돌린다.
func _sprite(index: int, art_px: Vector2, quarter := 0, color := Color.WHITE) -> void:
	var size_px := Vector2(T, T) * _scale
	var pos := _origin + art_px.round() * _scale
	var src := Rect2(index * T, 0, T, T)
	if quarter == 0:
		draw_texture_rect_region(_sheet, Rect2(pos, size_px), src, color)
		return
	var shake := (Vector2(sin(_clock * 90.0), cos(_clock * 77.0)) * _shake * 2.0).round() * _scale
	draw_set_transform(pos + size_px / 2.0 + shake, quarter * PI / 2.0)
	draw_texture_rect_region(_sheet, Rect2(-size_px / 2.0, size_px), src, color)
	draw_set_transform(shake)


func _draw_crab(a: Dictionary, b: Dictionary, f: float, moving: bool) -> void:
	var wa: Dictionary = a.world
	var wb: Dictionary = b.world
	var cell := Vector2(wa.x, wa.y)
	var dir: int = wa.dir
	var walk := 0
	var e := f * f * (3.0 - 2.0 * f)
	if moving and not b.bump:
		cell = cell.lerp(Vector2(wb.x, wb.y), e)
		# 도는 동안 반쯤 지나면 새 방향으로 (픽셀 그림은 90도씩만 돌린다)
		if f >= 0.5:
			dir = wb.dir
		if b.action == "forward":
			walk = 1 if f >= 0.5 else 0
	var art := cell * T
	if moving and b.bump and b.action == "forward":
		art += Vector2(DX[wa.dir], DY[wa.dir]) * round(sin(PI * f) * 4.0)
	if moving and b.action == "forward" and not b.bump:
		art.y -= round(sin(f * PI) * 1.0)
	# 그림은 집게가 북쪽을 본다. dir: 0 동, 1 남, 2 서, 3 북
	var quarter := (dir + 1) % 4
	draw_rect(Rect2(_origin + (art + Vector2(3, 13)).round() * _scale, Vector2(10, 2) * _scale), Color(0, 0, 0, 0.3))
	_sprite(Sprite.CRAB0 + walk, art, quarter)
	if moving and RING.has(b.action) and not b.bump:
		var c := _cell_center(cell)
		draw_arc(c, _tile * (0.3 + 0.35 * f), 0.0, TAU, 32, Color(RING[b.action], 1.0 - f), float(_scale), false)
	elif not moving:
		# 꽃게가 받침대 위에 서 있으면 올려 둔 보석이 가려지므로 위에 다시 그린다.
		for g in wa.gems:
			if g.placed and g.x == wa.x and g.y == wa.y:
				_sprite(_gem_sprite(g), art + Vector2(0, -8))


## 위쪽 정보 줄: 행동 수, 가방, 다이얼·두루마리·쪽지.
func _draw_hud(w: Dictionary, step: int) -> void:
	var font := Palette.pixel_font()
	var x := 12.0
	x = _chip(x, "행동 %d / %d" % [step, last_tick()], HUD_TEXT, font)
	var held: Array = w.gems.filter(func(g): return g.held)
	var bag_w := 52.0 + maxf(held.size(), 1) * 18.0
	_chip_box(x, bag_w)
	_text(x + 10, "가방", HUD_DIM, font)
	if held.is_empty():
		_text(x + 42, "-", HUD_FAINT, font)
	for i in held.size():
		draw_texture_rect_region(_sheet, Rect2(x + 40 + i * 18.0, 12, 16, 16), Rect2(_gem_sprite(held[i]) * T, 0, T, T))
	x += bag_w + 6.0
	if w.dial != null:
		x = _chip(x, "다이얼 %d" % w.dial, HUD_CODE, font)
	if not w.scroll.is_empty():
		x = _chip(x, "두루마리 [%s]" % ", ".join(PackedStringArray(w.scroll.map(func(n): return str(n)))), HUD_CODE, font)
	if w.note != null:
		x = _chip(x, "쪽지 %s" % JSON.stringify(w.note), HUD_CODE, font)
	if w.note2 != null:
		x = _chip(x, "쪽지 2 %s" % JSON.stringify(w.note2), HUD_CODE, font)


func _chip_box(x: float, w: float) -> void:
	draw_rect(Rect2(x, 8, w, 24), Color(0.02, 0.03, 0.06, 0.75))
	draw_rect(Rect2(x, 8, w, 24), Color(1, 1, 1, 0.12), false, 2.0)


func _text(x: float, text: String, color: Color, font: Font) -> void:
	var fs := Palette.PIXEL_SIZE
	draw_string(font, Vector2(x, round(8 + 12 + (font.get_ascent(fs) - font.get_descent(fs)) / 2.0)), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)


func _chip(x: float, text: String, color: Color, font: Font) -> float:
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, Palette.PIXEL_SIZE).x + 20.0
	_chip_box(x, w)
	_text(x + 10, text, color, font)
	return x + w + 6.0


func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		agent_selected.emit(0)
		accept_event()
