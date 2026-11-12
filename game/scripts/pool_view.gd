class_name PoolView
## 조수 웅덩이를 그린다. 틱 사이를 보간하므로 타임라인을 끌어 아무 시점이나 볼 수 있다.
##
## time 은 틱 단위 실수다. 2.5 면 2틱 모습과 3틱 모습의 중간.
extends Control

## 꽃게를 눌렀을 때.
signal crab_selected(index: int)

const CRAB_COLORS := [
	[Color("#f4511e"), Color("#b93a12")],
	[Color("#3fb6a8"), Color("#23766d")],
	[Color("#e2b33c"), Color("#9c7a1c")],
	[Color("#b07cf0"), Color("#6f45a8")],
]
const DIR_ANGLE := {"N": -PI / 2.0, "E": 0.0, "S": PI / 2.0, "W": PI}
const DIR_VEC := {"N": Vector2(0, -1), "E": Vector2(1, 0), "S": Vector2(0, 1), "W": Vector2(-1, 0)}

var map: Dictionary = {}
var frames: Array = []
var time := 0.0
var selected := 0

var _clock := 0.0
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
	selected = clampi(selected, 0, maxi(map.get("crabs", 1) - 1, 0))
	queue_redraw()


func last_tick() -> int:
	return maxi(frames.size() - 1, 0)


func frame_at(t: int) -> Dictionary:
	if frames.is_empty():
		return {}
	return frames[clampi(t, 0, frames.size() - 1)]


## 틱 하나를 지나갈 때 효과를 터뜨린다 (재생 중에만 부른다).
func play_effects(tick: int) -> void:
	var f := frame_at(tick)
	if f.is_empty():
		return
	var prev := frame_at(tick - 1)
	for i in f.crabs.size():
		var c: Dictionary = f.crabs[i]
		var at := _cell_center(Vector2(c.x, c.y))
		match c.mishap:
			"swept":
				var from: Dictionary = prev.crabs[i] if not prev.is_empty() else c
				_burst(_cell_center(Vector2(from.x, from.y)), Color("#7fc8ff"), 26, 1.6)
				Sfx.play("bump", 0.7)
			"caught":
				var from: Dictionary = prev.crabs[i] if not prev.is_empty() else c
				_burst(_cell_center(Vector2(from.x, from.y)), Color.WHITE, 22, 1.2)
				Sfx.play("fail", 1.4)
			"bump":
				Sfx.play("bump", 1.3)
		if not prev.is_empty():
			var was: Dictionary = prev.crabs[i]
			if c.carrying and not was.carrying:
				_burst(at, Color("#6fe0c0"), 14, 0.9)
				Sfx.play("collect", 1.0 + i * 0.08)
	if not prev.is_empty() and f.delivered > prev.delivered:
		_burst(_cell_center(Vector2(map.nest[0], map.nest[1])), Palette.ACCENT, 30, 1.4)
		Sfx.play("deposit")


func _process(delta: float) -> void:
	_clock += delta
	for p in _particles:
		p.vel.y += p.gravity * delta
		p.pos += p.vel * delta
		p.life -= delta
	_particles = _particles.filter(func(p): return p.life > 0.0)
	queue_redraw()


func _burst(at: Vector2, color: Color, count: int, power: float) -> void:
	for i in count:
		var a := randf() * TAU
		var sp := randf_range(0.5, 2.0) * _tile * power
		_particles.append({
			"pos": at, "vel": Vector2(cos(a), sin(a)) * sp, "life": randf_range(0.35, 0.8),
			"color": color, "size": _tile * randf_range(0.03, 0.07), "gravity": _tile * 2.5,
		})


func celebrate() -> void:
	var colors := [Palette.ACCENT, Palette.GREEN, Palette.SKY, Palette.WARN, Color("#6fe0c0")]
	var c := Vector2(size.x / 2.0, size.y * 0.4)
	for i in 110:
		var a := i * 2.399
		var sp := 140.0 + float((i * 37) % 100) * 2.8
		_particles.append({
			"pos": c, "vel": Vector2(cos(a), sin(a)) * sp + Vector2(0, -200),
			"life": 1.3 + float((i * 13) % 10) * 0.08, "color": colors[i % 5], "size": 7.0, "gravity": 560.0,
		})


# ── 좌표 ─────────────────────────────────────────────────────────

func _ground(x: int, y: int) -> String:
	if x < 0 or y < 0 or x >= map.w or y >= map.h:
		return "#"
	return map.ground[y * map.w + x]


func _layout() -> void:
	var cols: float = map.w
	var rows: float = map.h
	_tile = minf(minf((size.x - 16.0) / cols, (size.y - 44.0) / rows), 72.0)
	_origin = Vector2((size.x - _tile * cols) / 2.0, 36.0 + (size.y - 36.0 - _tile * rows) / 2.0)


func _cell_center(cell: Vector2) -> Vector2:
	return _origin + (cell + Vector2(0.5, 0.5)) * _tile


func _cell_rect(x: int, y: int) -> Rect2:
	return Rect2(_origin + Vector2(x, y) * _tile, Vector2(_tile, _tile))


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	# 밤바다
	draw_rect(Rect2(Vector2.ZERO, size), Color("#0e1e30"))
	if map.is_empty() or frames.is_empty() or size.x < 60.0 or size.y < 60.0:
		return
	_layout()
	if _tile < 6.0:
		return
	_draw_waves()

	var k := int(floor(time))
	var f := clampf(time - k, 0.0, 1.0)
	var a := frame_at(k)
	var b := frame_at(k + 1)
	var e := f * f * (3.0 - 2.0 * f)

	# 밀물 정도 (0 = 다 빠짐, 1 = 다 참)
	var flood_a := 1.0 if a.tide.kind == "high" else 0.0
	var flood_b := 1.0 if b.tide.kind == "high" else 0.0
	var flood: float = lerp(flood_a, flood_b, e)

	_draw_ground(flood)

	var foods: Array = a.foods if f < 0.5 else b.foods
	for food in foods:
		_draw_food(_cell_center(Vector2(food.x, food.y)), 1.0)

	for i in a.gulls.size():
		var ga: Dictionary = a.gulls[i]
		var gb: Dictionary = b.gulls[i] if i < b.gulls.size() else ga
		var pos := Vector2(ga.x, ga.y).lerp(Vector2(gb.x, gb.y), e)
		_draw_gull(_cell_center(pos), gb.dir, i)

	var order := range(a.crabs.size())
	# 고른 꽃게를 맨 위에 그린다.
	order.erase(selected)
	order.append(selected)
	for i in order:
		if i < a.crabs.size():
			_draw_crab(i, a.crabs[i], b.crabs[i] if i < b.crabs.size() else a.crabs[i], f, e)

	for p in _particles:
		var lk := clampf(p.life, 0.0, 1.0)
		draw_circle(p.pos, p.size * (0.6 + lk), Color(p.color, lk), true, -1.0, true)

	_draw_hud(a if f < 0.5 else b, flood)


func _draw_waves() -> void:
	var c := Color("#16314c")
	for i in 7:
		var y := fmod(i * 61.0 + _clock * 6.0, size.y + 40.0) - 20.0
		var pts := PackedVector2Array()
		var x := -10.0
		while x < size.x + 20.0:
			pts.append(Vector2(x, y + sin(x * 0.03 + _clock * 0.8 + i) * 4.0))
			x += 16.0
		draw_polyline(pts, c, 1.5, true)


func _rounded(rect: Rect2, color: Color, radius: float) -> void:
	var sb := StyleBoxFlat.new()
	sb.bg_color = color
	sb.set_corner_radius_all(int(radius))
	sb.anti_aliasing = true
	draw_style_box(sb, rect)


func _draw_ground(flood: float) -> void:
	var t := _tile
	for y in map.h:
		for x in map.w:
			var g := _ground(x, y)
			var r := _cell_rect(x, y)
			match g:
				"#":
					if _near_open(x, y):
						_draw_rock(r, x, y)
				"w":
					_draw_water(r, 1.0)
				_:
					var sand := Color("#6e604f") if (x + y) % 2 == 0 else Color("#6a5c4b")
					if g == "~":
						sand = Color("#4f5560") if (x + y) % 2 == 0 else Color("#4b515c")
					_rounded(r.grow(-1.0), sand, t * 0.12)
					if g == "~":
						# 젖은 갯벌 무늬
						draw_line(r.position + Vector2(t * 0.2, t * 0.7), r.position + Vector2(t * 0.45, t * 0.62), Color(1, 1, 1, 0.08), 1.5, true)
						draw_line(r.position + Vector2(t * 0.55, t * 0.35), r.position + Vector2(t * 0.8, t * 0.3), Color(1, 1, 1, 0.08), 1.5, true)
						if flood > 0.0:
							_draw_water(r, flood)
					if g == "N":
						var c := r.get_center()
						draw_circle(c, t * 0.36, Color("#3a2a20"), true, -1.0, true)
						draw_circle(c, t * 0.27, Color("#1e1510"), true, -1.0, true)
						draw_arc(c, t * 0.36, 0.0, TAU, 40, Color(Palette.ACCENT, 0.7), 2.0, true)


func _near_open(x: int, y: int) -> bool:
	for dy in range(-1, 2):
		for dx in range(-1, 2):
			var g := _ground(x + dx, y + dy)
			if g != "#" and not (x + dx < 0 or y + dy < 0 or x + dx >= map.w or y + dy >= map.h):
				return true
	return false


## 해안 바위: 칸마다 조금씩 다른 둥근 돌 두세 개.
func _draw_rock(r: Rect2, x: int, y: int) -> void:
	var t := _tile
	var h := absi(hash(Vector2i(x, y)))
	var c := r.get_center()
	var stones := 2 + (h % 2)
	for i in stones:
		var k := (h >> (i * 5)) & 31
		var off := Vector2(((k % 7) - 3) * t * 0.06, (((k >> 3) % 4) - 1.5) * t * 0.08)
		var rad := t * (0.34 - i * 0.06 + (k % 3) * 0.02)
		var base := Color("#2a2d40").lerp(Color("#33374d"), float(k % 5) / 5.0)
		draw_circle(c + off + Vector2(0, t * 0.04), rad, Color(0, 0, 0, 0.25), true, -1.0, true)
		draw_circle(c + off, rad, base, true, -1.0, true)
		draw_circle(c + off - Vector2(rad * 0.25, rad * 0.3), rad * 0.45, base.lightened(0.12), true, -1.0, true)


func _draw_water(r: Rect2, alpha: float) -> void:
	_rounded(r, Color(0.12, 0.33, 0.52, 0.92 * alpha), _tile * 0.1)
	var y := r.position.y + r.size.y * (0.35 + 0.1 * sin(_clock * 2.0 + r.position.x * 0.05))
	draw_line(Vector2(r.position.x + 4, y), Vector2(r.end.x - 4, y), Color(0.6, 0.85, 1.0, 0.25 * alpha), 1.5, true)
	var y2 := r.position.y + r.size.y * (0.7 + 0.08 * sin(_clock * 2.4 + r.position.y * 0.05))
	draw_line(Vector2(r.position.x + r.size.x * 0.3, y2), Vector2(r.end.x - 6, y2), Color(0.6, 0.85, 1.0, 0.18 * alpha), 1.5, true)


func _draw_food(c: Vector2, alpha: float) -> void:
	var pulse := 1.0 + 0.12 * sin(_clock * 4.0 + c.x * 0.1)
	var t := _tile
	draw_circle(c, t * 0.26 * pulse, Color(0.44, 0.88, 0.75, 0.12 * alpha), true, -1.0, true)
	draw_circle(c, t * 0.17 * pulse, Color(0.44, 0.88, 0.75, 0.25 * alpha), true, -1.0, true)
	draw_circle(c, t * 0.1, Color(0.62, 1.0, 0.88, alpha), true, -1.0, true)
	draw_circle(c + Vector2(-t * 0.03, -t * 0.03), t * 0.035, Color(1, 1, 1, 0.9 * alpha), true, -1.0, true)


func _draw_gull(c: Vector2, dir: String, i: int) -> void:
	var t := _tile
	var face: float = 1.0 if dir == "E" else (-1.0 if dir == "W" else 1.0)
	var flap := sin(_clock * 9.0 + i * 1.7)
	# 그림자
	draw_set_transform(c + Vector2(0, t * 0.32), 0.0, Vector2(1.0, 0.3))
	draw_circle(Vector2.ZERO, t * 0.28, Color(0, 0, 0, 0.28), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)
	var body := c + Vector2(0, -t * 0.12 + flap * t * 0.02)
	# 날개
	for side in [-1.0, 1.0]:
		var tip := body + Vector2(-face * t * 0.05, side * t * (0.36 + flap * 0.08))
		draw_colored_polygon(PackedVector2Array([body + Vector2(face * t * 0.1, 0), tip, body + Vector2(-face * t * 0.18, 0)]), Color("#d9dde8"))
	draw_set_transform(body, 0.0, Vector2(1.0, 0.62))
	draw_circle(Vector2.ZERO, t * 0.2, Color("#f4f6fb"), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)
	draw_circle(body + Vector2(face * t * 0.18, -t * 0.04), t * 0.09, Color("#f4f6fb"), true, -1.0, true)
	draw_circle(body + Vector2(face * t * 0.21, -t * 0.06), t * 0.022, Color("#1a1a22"), true, -1.0, true)
	draw_colored_polygon(PackedVector2Array([
		body + Vector2(face * t * 0.26, -t * 0.05), body + Vector2(face * t * 0.36, -t * 0.02), body + Vector2(face * t * 0.26, 0),
	]), Color("#f2b13c"))


func _draw_crab(i: int, ca: Dictionary, cb: Dictionary, f: float, e: float) -> void:
	var t := _tile
	var colors: Array = CRAB_COLORS[i % CRAB_COLORS.size()]
	var pa := Vector2(ca.x, ca.y)
	var pb := Vector2(cb.x, cb.y)
	var pos := pa.lerp(pb, e)
	var alpha := 1.0
	var teleport := pa.distance_to(pb) > 1.01
	if teleport:
		# 휩쓸리거나 쫓겨 굴로 돌아갈 때는 사라졌다가 굴에서 나타난다.
		pos = pa if f < 0.5 else pb
		alpha = absf(f - 0.5) * 2.0
	var walk := 0.0
	var hop := 0.0
	if not teleport and pa != pb:
		walk = f
		hop = sin(f * PI) * 0.05
	if cb.mishap == "bump" and not teleport:
		pos += DIR_VEC[cb.facing] * sin(f * PI) * 0.2
	var c := _cell_center(pos) - Vector2(0, hop * t)
	# 꽃게는 옆으로 걷는다: 가는 방향과 90도 돌려 그린다.
	var angle: float = DIR_ANGLE[cb.facing] - PI / 2.0
	var shell: Color = Color(colors[0], alpha)
	var dark: Color = Color(colors[1], alpha)
	var crowd := 0.0
	# 굴 위에 여러 마리가 겹치면 조금씩 비켜 그린다.
	if Vector2(cb.x, cb.y) == Vector2(map.nest[0], map.nest[1]) and pos.distance_to(pb) < 0.01:
		crowd = (i - (map.crabs - 1) / 2.0) * 0.18
	c += Vector2(crowd * t, 0)
	var body_scale := 0.82 if map.crabs > 1 else 0.95

	if i == selected:
		var ring := Color(1, 1, 1, (0.35 + 0.2 * sin(_clock * 4.0)) * alpha)
		draw_arc(c, t * 0.42, 0.0, TAU, 40, ring, 2.0, true)
	draw_set_transform(c + Vector2(0, t * 0.28), 0.0, Vector2(1.0, 0.3))
	draw_circle(Vector2.ZERO, t * 0.27, Color(0, 0, 0, 0.25 * alpha), true, -1.0, true)
	draw_set_transform(Vector2.ZERO)
	CrabArt.draw_crab(self, c, t * body_scale, angle, walk, 0.0, shell, dark)
	var carrying: bool = ca.carrying if f < 0.5 else cb.carrying
	if carrying and alpha > 0.3:
		_draw_food(c - Vector2(0, t * 0.05), alpha)
	if map.crabs > 1:
		var font := Palette.code_font()
		var label := str(i)
		var p := c + Vector2(t * 0.26, -t * 0.26)
		draw_circle(p, t * 0.13, Color(colors[1], 0.95 * alpha), true, -1.0, true)
		var w := font.get_string_size(label, HORIZONTAL_ALIGNMENT_LEFT, -1, int(t * 0.18)).x
		draw_string(font, p + Vector2(-w / 2.0, t * 0.065), label, HORIZONTAL_ALIGNMENT_LEFT, -1, int(t * 0.18), Color(1, 1, 1, alpha))


func _draw_hud(fr: Dictionary, flood: float) -> void:
	var font := Palette.text_font()
	var x := 12.0
	var text := "틱 %d / %d" % [fr.tick, map.maxTicks]
	x = _chip(x, text, Palette.TEXT, font)
	x = _chip(x, "배달 %d / %d" % [fr.delivered, map.goal], Color("#6fe0c0"), font)
	match fr.tide.kind:
		"low":
			x = _chip(x, "썰물 · %d틱 뒤 밀물" % fr.tide.turnsLeft, Palette.WARN if fr.tide.turnsLeft <= 3 else Palette.SKY, font)
		"high":
			x = _chip(x, "밀물 · %d틱 뒤 빠짐" % fr.tide.turnsLeft, Color("#7fc8ff"), font)


func _chip(x: float, text: String, color: Color, font: Font) -> float:
	var fs := 13
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x + 20.0
	_rounded(Rect2(x, 8, w, 24), Color(0, 0, 0, 0.35), 12)
	draw_string(font, Vector2(x + 10, 8 + 12 + (font.get_ascent(fs) - font.get_descent(fs)) / 2.0), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)
	return x + w + 6.0


# ── 마우스 ───────────────────────────────────────────────────────

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		var fr := frame_at(int(round(time)))
		if fr.is_empty():
			return
		var best := -1
		var best_d := _tile * 0.6
		for i in fr.crabs.size():
			var c: Dictionary = fr.crabs[i]
			var d := _cell_center(Vector2(c.x, c.y)).distance_to(event.position)
			if d < best_d:
				best_d = d
				best = i
		if best >= 0:
			# 굴 위처럼 겹쳐 있으면 누를 때마다 다음 꽃게로.
			if best == selected and fr.crabs.size() > 1:
				var here: Dictionary = fr.crabs[best]
				for step in range(1, fr.crabs.size()):
					var j: int = (best + step) % fr.crabs.size()
					if fr.crabs[j].x == here.x and fr.crabs[j].y == here.y:
						best = j
						break
			selected = best
			crab_selected.emit(best)
			queue_redraw()
		accept_event()
