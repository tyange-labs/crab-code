class_name WorldView
## 월드(지도·꽃게·보석)를 그리고, 실행 결과로 받은 행동들을 차례로 재생한다.
extends Control

## println! 출력이나 패닉 메시지가 나왔을 때. tone 은 "" / "bad".
signal printed(text: String, tone: String)
## 월드 상태가 바뀌었을 때 (가방·정보 표시 갱신용).
signal world_changed(world: Dictionary)
## 재생이 끝났을 때.
signal finished

const DX := [1, 0, -1, 0]
const DY := [0, 1, 0, -1]
const SPEEDS := [1.0, 2.0, 4.0]
const STEP_TIME := {"forward": 0.24, "left": 0.18, "right": 0.18}
const RING := {"collect": Color("#f2c94c"), "deposit": Color("#5fd38d"), "polish": Color.WHITE, "unlock": Color("#5cb8ff")}

var world: Dictionary = {}
var speed_idx := 0
var playing := false

var _queue: Array = []
var _anim: Dictionary = {}
var _bump := 0.0
var _bump_forward := false
var _shake := 0.0
var _squash := 0.0
var _time := 0.0
var _particles: Array = []
var _layout := {"tile": 1.0, "origin": Vector2.ZERO}


func set_world(w: Dictionary) -> void:
	world = w
	_queue.clear()
	_anim = {}
	_bump = 0.0
	_particles.clear()
	playing = false
	world_changed.emit(world)
	queue_redraw()


## 실행 결과의 단계 목록을 재생하기 시작한다.
func play(steps: Array) -> void:
	_queue = steps.duplicate()
	_anim = {}
	playing = true


func stop() -> void:
	_queue.clear()
	_anim = {}
	playing = false


func cycle_speed() -> float:
	speed_idx = (speed_idx + 1) % SPEEDS.size()
	return SPEEDS[speed_idx]


func celebrate() -> void:
	var colors := [Color("#f46a25"), Color("#5fd38d"), Color("#5cb8ff"), Color("#f2c94c"), Color("#ff5c7a")]
	var c := Vector2(size.x / 2.0, size.y * 0.45)
	for i in 110:
		var a := i * 2.399
		var sp := 140.0 + float((i * 37) % 100) * 2.8
		_particles.append({
			"pos": c, "vel": Vector2(cos(a), sin(a)) * sp + Vector2(0, -200),
			"life": 1.3 + float((i * 13) % 10) * 0.08, "color": colors[i % 5],
			"size": 8.0, "gravity": 560.0, "spin": float(i % 7) - 3.0, "kind": "confetti",
		})


func _process(delta: float) -> void:
	delta = minf(delta, 0.05)
	_time += delta
	if playing:
		_advance(delta)
	_bump = maxf(_bump - delta, 0.0)
	_shake = maxf(_shake - delta * 2.5, 0.0)
	_squash = maxf(_squash - delta * 6.0, 0.0)
	for p in _particles:
		p.vel.y += p.gravity * delta
		p.pos += p.vel * delta
		p.life -= delta
	_particles = _particles.filter(func(p): return p.life > 0.0)
	queue_redraw()


func _advance(delta: float) -> void:
	if not _anim.is_empty():
		var base: float = STEP_TIME.get(_anim.action, 0.22)
		_anim.t += delta * SPEEDS[speed_idx] / base
		if _anim.t < 1.0:
			return
		if _anim.action == "forward":
			_squash = 1.0
		_anim = {}
	while true:
		if _queue.is_empty():
			playing = false
			finished.emit()
			return
		var step: Dictionary = _queue.pop_front()
		match step.type:
			"print":
				printed.emit(step.text, "")
			"stderr":
				printed.emit(step.text, "bad")
			"bump":
				_bump = 0.3
				_bump_forward = step.forward
				_shake = 1.0
				Sfx.play("bump")
			_:
				_start_step(step)
				return


func _start_step(step: Dictionary) -> void:
	_anim = {"from": Vector2(world.x, world.y), "from_dir": world.dir, "t": 0.0, "action": step.action}
	var before: Dictionary = world
	world = step.world
	world_changed.emit(world)
	var here := _cell_center(Vector2(world.x, world.y))
	match step.action:
		"forward":
			Sfx.play("step", randf_range(0.95, 1.05))
		"left", "right":
			Sfx.play("turn")
		"collect":
			Sfx.play("collect")
			var gem = _changed_gem(before, "held")
			_burst(here, Palette.GEM[gem.color] if gem else Palette.WARN, 18)
		"deposit":
			Sfx.play("deposit")
			_burst(here, Palette.GREEN, 22)
		"polish":
			Sfx.play("polish")
			_burst(here, Color.WHITE, 14)
		"unlock":
			Sfx.play("unlock")
			var d: Dictionary = world.door
			_burst(_cell_center(Vector2(d.x, d.y)), Palette.SKY, 26)


## 이번 행동으로 상태(held 등)가 바뀐 보석.
func _changed_gem(before: Dictionary, key: String):
	for i in world.gems.size():
		if world.gems[i][key] and not before.gems[i][key]:
			return world.gems[i]
	return null


func _burst(at: Vector2, color: Color, count: int) -> void:
	var tile: float = _layout.tile
	for i in count:
		var a := randf() * TAU
		var sp := randf_range(0.6, 2.2) * tile
		_particles.append({
			"pos": at, "vel": Vector2(cos(a), sin(a)) * sp, "life": randf_range(0.35, 0.7),
			"color": color, "size": tile * randf_range(0.04, 0.08), "gravity": tile * 2.0, "spin": 0.0, "kind": "spark",
		})


# ── 그리기 ──────────────────────────────────────────────────────

func _is_wall(x: int, y: int) -> bool:
	return x < 0 or y < 0 or x >= world.w or y >= world.h or world.walls[y * world.w + x]


func _update_layout() -> void:
	# 바닥이 있는 범위만 화면에 맞춘다.
	var lo := Vector2i(world.w, world.h)
	var hi := Vector2i(0, 0)
	for y in world.h:
		for x in world.w:
			if not _is_wall(x, y):
				lo = Vector2i(mini(lo.x, x), mini(lo.y, y))
				hi = Vector2i(maxi(hi.x, x), maxi(hi.y, y))
	var cols := hi.x - lo.x + 1
	var rows := hi.y - lo.y + 1
	var tile := minf(minf((size.x - 24.0) / cols, (size.y - 20.0) / rows), 92.0)
	_layout.tile = tile
	_layout.origin = Vector2((size.x - tile * cols) / 2.0 - lo.x * tile, (size.y - tile * rows) / 2.0 - lo.y * tile)


func _cell_center(cell: Vector2) -> Vector2:
	return _layout.origin + (cell + Vector2(0.5, 0.5)) * _layout.tile


func _rect(c: Vector2, half: float, radius: float, color: Color) -> void:
	var sb := StyleBoxFlat.new()
	sb.bg_color = color
	sb.set_corner_radius_all(int(radius))
	sb.anti_aliasing = true
	draw_style_box(sb, Rect2(c - Vector2(half, half), Vector2(half, half) * 2.0))


func _label(text: String, c: Vector2, font_size: float, color: Color, font: Font = null) -> void:
	if font == null:
		font = Palette.text_font()
	var fs := int(font_size)
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
	var baseline := c.y + (font.get_ascent(fs) - font.get_descent(fs)) / 2.0
	draw_string(font, Vector2(c.x - w / 2.0, baseline), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)


func _draw() -> void:
	if world.is_empty() or size.x < 10 or size.y < 10:
		return
	_update_layout()
	var tile: float = _layout.tile
	var shake := Vector2(sin(_time * 90.0), cos(_time * 77.0)) * _shake * tile * 0.06
	draw_set_transform(shake)

	# 바닥
	for y in world.h:
		for x in world.w:
			if not _is_wall(x, y):
				var c := _cell_center(Vector2(x, y))
				_rect(c, tile / 2.0 - 2.0, tile * 0.14, Color("#363b58") if (x + y) % 2 == 0 else Color("#323651"))

	for n in world.numbers:
		var c := _cell_center(Vector2(n.x, n.y))
		_rect(c, tile * 0.3, tile * 0.1, Color("#27405e"))
		_label(str(n.n), c, tile * 0.4, Palette.SKY, Palette.code_font())

	for s in world.signs:
		if s.sign != "left" and s.sign != "right":
			continue
		var c := _cell_center(Vector2(s.x, s.y))
		var sb := StyleBoxFlat.new()
		sb.bg_color = Palette.WARN
		sb.set_corner_radius_all(int(tile * 0.08))
		draw_style_box(sb, Rect2(c - Vector2(tile * 0.4, tile * 0.2), Vector2(tile * 0.8, tile * 0.4)))
		_label("왼쪽" if s.sign == "left" else "오른쪽", c, tile * 0.23, Color("#3a2c00"))

	if world.get("flag") != null:
		var f := _cell_center(Vector2(world.flag.x, world.flag.y))
		var px := f.x - tile * 0.16
		draw_line(Vector2(px, f.y - tile * 0.32), Vector2(px, f.y + tile * 0.32), Palette.TEXT, tile * 0.05, true)
		var wave := sin(_time * 4.0) * tile * 0.03
		draw_colored_polygon(PackedVector2Array([
			Vector2(px, f.y - tile * 0.32), Vector2(px + tile * 0.4, f.y - tile * 0.17 + wave), Vector2(px, f.y - tile * 0.02),
		]), Palette.GREEN)

	for p in world.pedestals:
		var c := _cell_center(Vector2(p.x, p.y))
		var ring: Color = Palette.GEM[p.want] if p.want != null else Color("#c9cde0")
		_rect(c, tile * 0.36, tile * 0.12, ring.lerp(Color.BLACK, 0.2))
		_rect(c, tile * 0.29, tile * 0.09, Color("#23263a"))

	if world.get("door") != null:
		var d := _cell_center(Vector2(world.door.x, world.door.y))
		if world.door.open:
			draw_rect(Rect2(d - Vector2(tile * 0.38, tile * 0.38), Vector2(tile * 0.76, tile * 0.76)), Color("#8a5a2b"), false, tile * 0.07)
		else:
			_rect(d, tile * 0.42, tile * 0.1, Color("#8a5a2b"))
			_rect(d, tile * 0.34, tile * 0.07, Color("#a9713a"))
			draw_circle(d + Vector2(0, -tile * 0.04), tile * 0.11, Palette.WARN, true, -1.0, true)
			draw_rect(Rect2(d + Vector2(-tile * 0.03, 0), Vector2(tile * 0.06, tile * 0.16)), Palette.WARN)

	for g in world.gems:
		if g.held:
			continue
		var bob := 0.0 if g.placed else sin(_time * 2.6 + g.x) * tile * 0.03
		var c := _cell_center(Vector2(g.x, g.y)) + Vector2(0, bob)
		CrabArt.draw_gem(self, c, tile * 0.24, g)
		if world.showWeights:
			_label(str(g.weight), c + Vector2(0, tile * 0.01), tile * 0.2, Color("#1a1a22"), Palette.code_font())

	_draw_crab(tile)

	# 불꽃·색종이
	for p in _particles:
		var k := clampf(p.life, 0.0, 1.0)
		var col: Color = p.color
		if p.kind == "confetti":
			var s: float = p.size * k
			draw_set_transform(shake + p.pos, p.spin * _time, Vector2.ONE)
			draw_rect(Rect2(Vector2(-s / 2.0, -s / 2.0), Vector2(s, s * 0.6)), col)
			draw_set_transform(shake)
		else:
			draw_circle(p.pos, p.size * k * 1.4, Color(col, k), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)


func _draw_crab(tile: float) -> void:
	var pos := Vector2(world.x, world.y)
	var angle: float = world.dir * PI / 2.0
	var walk := 0.0
	var hop := 0.0
	if not _anim.is_empty():
		var t := clampf(_anim.t, 0.0, 1.0)
		var e := t * t * (3.0 - 2.0 * t)
		pos = _anim.from.lerp(pos, e)
		var delta: int = ((world.dir - _anim.from_dir + 6) % 4) - 2
		angle = (_anim.from_dir + delta * e) * PI / 2.0
		if _anim.action == "forward":
			walk = t
			hop = sin(t * PI) * 0.06
	if _bump > 0.0 and _bump_forward:
		var k := sin(PI * (1.0 - _bump / 0.3)) * 0.22
		pos += Vector2(DX[world.dir], DY[world.dir]) * k
	var c := _cell_center(pos) - Vector2(0, hop * tile)

	# 그림자
	draw_set_transform(c + Vector2(0, tile * 0.3), 0.0, Vector2(1.0, 0.35))
	draw_circle(Vector2.ZERO, tile * 0.3, Color(0, 0, 0, 0.25), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)
	var shake := Vector2(sin(_time * 90.0), cos(_time * 77.0)) * _shake * tile * 0.06
	CrabArt.draw_crab(self, c + shake, tile, angle, walk, _squash)
	draw_set_transform(shake)

	if not _anim.is_empty() and RING.has(_anim.action):
		var t := clampf(_anim.t, 0.0, 1.0)
		draw_arc(c, tile * (0.3 + 0.35 * t), 0.0, TAU, 40, Color(RING[_anim.action], 1.0 - t), tile * 0.05, true)
	elif _anim.is_empty():
		# 꽃게가 받침대 위에 서 있으면 올려 둔 보석이 가려지므로 위에 다시 그린다.
		for g in world.gems:
			if g.placed and g.x == world.x and g.y == world.y:
				CrabArt.draw_gem(self, c - Vector2(0, tile * 0.3), tile * 0.17, g)
