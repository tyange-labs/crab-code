## 조수 웅덩이 게임팩의 화면: 16x16 픽셀 아트로 밤바다 웅덩이를 그린다.
##
## 게임 본체(host.gd)와의 약속 (모든 게임팩 화면이 같다):
##   setup(map, frames)     게임팩 프로그램이 출력한 지도와 틱마다의 모습
##   time                   틱 단위 실수. 2.5 면 2틱 모습과 3틱 모습의 중간 (틱 사이를 보간한다)
##   selected               고른 행위자 번호
##   last_tick(), frame_at(t)
##   play_effects(t)        재생 중 t 틱에 들어설 때 효과음·불꽃
##   celebrate()            성공했을 때
##   agent_count()          행위자 수
##   describe_agent(i, t)   t 틱의 i 번 행위자를 한 줄로 (살펴보기 창에 쓴다)
##   signal agent_selected  행위자를 눌렀을 때
##
## 그림은 assets/ 에 있다. 지형·꽃게·갈매기·먹이는 ../art/sprites.py 가 만든 것이고,
## 반짝이·물방울 같은 아이콘은 Kenney 의 1-Bit Pack (CC0, assets/kenney/) 에서 가져왔다.
## 모든 것을 16픽셀 격자에 그린 뒤 정수 배율로 키워서 픽셀이 뭉개지지 않게 한다.
extends Control

## 꽃게를 눌렀을 때.
signal agent_selected(index: int)

const T := 16 # 그림 한 칸의 픽셀 수
const ASSETS := "res://packs/tidepool/view/assets/"
const CRAB_FILES := ["crab_orange", "crab_teal", "crab_yellow", "crab_purple"]
const DIR_VEC := {"N": Vector2(0, -1), "E": Vector2(1, 0), "S": Vector2(0, 1), "W": Vector2(-1, 0)}
# tiles.png 안의 순서
enum Tile { SAND0, SAND1, FLAT0, FLAT1, ROCK0, ROCK1, WATER0, WATER1, FLOOD0, FLOOD1, NEST }
# Kenney 1-Bit Pack 안의 아이콘 (열, 줄)
const ICON_BUBBLES := Vector2i(35, 11)
const ICON_SPARKLE := Vector2i(36, 11)
const ICON_HOURGLASS := Vector2i(38, 12)
const ICON_ALERT := Vector2i(36, 13)
const ICON_CORNERS := Vector2i(35, 12)

var map: Dictionary = {}
var frames: Array = []
var time := 0.0
var selected := 0

var _clock := 0.0
var _particles: Array = []
var _scale := 1 # 그림 1픽셀이 화면 몇 픽셀인지 (정수)
var _origin := Vector2.ZERO # 지도 (0, 0) 칸의 화면 위치

var _tiles: Texture2D = load(ASSETS + "tiles.png")
var _gull: Texture2D = load(ASSETS + "gull.png")
var _food: Texture2D = load(ASSETS + "food.png")
var _icons: Texture2D = load(ASSETS + "kenney/1bit.png")
var _crabs: Array = CRAB_FILES.map(func(f): return load(ASSETS + f + ".png"))


func _init() -> void:
	mouse_filter = Control.MOUSE_FILTER_STOP
	clip_contents = true
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST


func setup(new_map: Dictionary, new_frames: Array) -> void:
	map = new_map
	frames = new_frames
	time = 0.0
	_particles.clear()
	selected = clampi(selected, 0, maxi(map.get("crabs", 1) - 1, 0))
	queue_redraw()


func agent_count() -> int:
	return map.get("crabs", 0)


func describe_agent(i: int, t: int) -> String:
	var f := frame_at(t)
	if f.is_empty() or i >= f.crabs.size():
		return ""
	var c: Dictionary = f.crabs[i]
	return "(%d, %d)%s" % [c.x, c.y, " · 먹이 들고 있음" if c.carrying else ""]


func last_tick() -> int:
	return maxi(frames.size() - 1, 0)


func frame_at(t: int) -> Dictionary:
	if frames.is_empty():
		return {}
	return frames[clampi(t, 0, frames.size() - 1)]


# ── 효과 ─────────────────────────────────────────────────────────

## 재생 중 t 틱에 들어설 때.
func play_effects(tick: int) -> void:
	var f := frame_at(tick)
	if f.is_empty():
		return
	var prev := frame_at(tick - 1)
	for i in f.crabs.size():
		var c: Dictionary = f.crabs[i]
		var was: Dictionary = prev.crabs[i] if not prev.is_empty() else c
		var at := Vector2(c.x, c.y)
		match c.mishap:
			"swept":
				_icon_float(Vector2(was.x, was.y), ICON_BUBBLES, Color("#8fd0ff"))
				_burst(Vector2(was.x, was.y), Color("#6ca8d8"), 14)
				Sfx.play("bump", 0.7)
			"caught":
				_icon_float(Vector2(was.x, was.y), ICON_ALERT, Color("#ffffff"))
				_icon_float(at, ICON_HOURGLASS, Color("#f2c94c"))
				Sfx.play("fail", 1.4)
			"bump":
				Sfx.play("bump", 1.3)
		if not prev.is_empty() and c.carrying and not was.carrying:
			_icon_float(at, ICON_SPARKLE, Color("#b6ffe9"))
			Sfx.play("collect", 1.0 + i * 0.08)
	if not prev.is_empty() and f.delivered > prev.delivered:
		var nest := Vector2(map.nest[0], map.nest[1])
		_icon_float(nest, ICON_SPARKLE, Palette.ACCENT)
		_burst(nest, Palette.ACCENT, 18)
		Sfx.play("deposit")


## 칸 위로 떠오르며 사라지는 아이콘.
func _icon_float(cell: Vector2, icon: Vector2i, color: Color) -> void:
	_particles.append({"kind": "icon", "cell": cell, "icon": icon, "color": color, "life": 1.0, "age": 0.0})


## 칸에서 튀어 나가는 픽셀 조각들 (그림 픽셀 단위로 움직인다).
func _burst(cell: Vector2, color: Color, count: int) -> void:
	for i in count:
		var a := randf() * TAU
		_particles.append({
			"kind": "pixel", "pos": (cell + Vector2(0.5, 0.5)) * T, "vel": Vector2(cos(a), sin(a)) * randf_range(20.0, 55.0),
			"color": color, "life": randf_range(0.35, 0.7), "age": 0.0,
		})


func celebrate() -> void:
	var colors := [Palette.ACCENT, Palette.GREEN, Palette.SKY, Palette.WARN, Color("#b6ffe9")]
	var center := Vector2(map.get("w", 10), map.get("h", 6)) * T / 2.0
	for i in 90:
		var a := i * 2.399
		var sp := 40.0 + float((i * 37) % 100) * 0.9
		_particles.append({
			"kind": "pixel", "pos": center, "vel": Vector2(cos(a), sin(a)) * sp + Vector2(0, -60),
			"color": colors[i % 5], "life": 1.3 + float((i * 13) % 10) * 0.08, "age": 0.0, "gravity": 140.0,
		})


func _process(delta: float) -> void:
	_clock += delta
	for p in _particles:
		p.age += delta
		p.life -= delta
		if p.kind == "pixel":
			p.vel.y += p.get("gravity", 60.0) * delta
			p.pos += p.vel * delta
	_particles = _particles.filter(func(p): return p.life > 0.0)
	queue_redraw()


# ── 좌표 ─────────────────────────────────────────────────────────

func _ground(x: int, y: int) -> String:
	if x < 0 or y < 0 or x >= map.w or y >= map.h:
		return "#"
	return map.ground[y * map.w + x]


func _layout() -> void:
	# 위쪽 정보 줄은 지도 모서리(바위) 위에 겹쳐 그린다 (그래야 정수 배율을 한 단계 크게 쓸 수 있다).
	var top := 8.0
	var avail := Vector2(size.x - 16.0, size.y - top - 8.0)
	_scale = maxi(1, int(floor(minf(avail.x / (map.w * T), avail.y / (map.h * T)))))
	var px := Vector2(map.w, map.h) * T * _scale
	_origin = Vector2(floor((size.x - px.x) / 2.0), floor(top + (size.y - top - px.y) / 2.0))


## 그림 픽셀 좌표(지도 왼쪽 위 기준)를 화면 좌표로. 픽셀 격자에 딱 맞춘다.
func _screen(art_px: Vector2) -> Vector2:
	return _origin + art_px.round() * _scale


func _draw_tile(cell: Vector2, index: int, color := Color.WHITE) -> void:
	var dst := Rect2(_screen(cell * T), Vector2(T, T) * _scale)
	draw_texture_rect_region(_tiles, dst, Rect2(index * T, 0, T, T), color)


func _draw_sprite(tex: Texture2D, frame: int, art_px: Vector2, color := Color.WHITE, flip := false, rotate_quarter := 0) -> void:
	var size_px := Vector2(T, T) * _scale
	var pos := _screen(art_px)
	if flip or rotate_quarter != 0:
		var center := pos + size_px / 2.0
		draw_set_transform(center, rotate_quarter * PI / 2.0, Vector2(-1 if flip else 1, 1))
		draw_texture_rect_region(tex, Rect2(-size_px / 2.0, size_px), Rect2(frame * T, 0, T, T), color)
		draw_set_transform(Vector2.ZERO)
	else:
		draw_texture_rect_region(tex, Rect2(pos, size_px), Rect2(frame * T, 0, T, T), color)


func _draw_icon(icon: Vector2i, art_px: Vector2, color: Color) -> void:
	var dst := Rect2(_screen(art_px), Vector2(T, T) * _scale)
	draw_texture_rect_region(_icons, dst, Rect2(icon.x * T, icon.y * T, T, T), color)


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color("#0f263d"))
	if map.is_empty() or frames.is_empty() or size.x < 60.0 or size.y < 60.0:
		return
	_layout()
	var k := int(floor(time))
	var f := clampf(time - k, 0.0, 1.0)
	var a := frame_at(k)
	var b := frame_at(k + 1)
	var e := f * f * (3.0 - 2.0 * f)
	var water_frame := int(_clock * 1.6) % 2

	# 화면 전체를 바다로 덮는다 (지도 격자에 맞춰서).
	var first := ((-_origin) / (T * _scale)).floor() - Vector2.ONE
	var count := (size / (T * _scale)).ceil() + Vector2(3, 3)
	for gy in int(count.y):
		for gx in int(count.x):
			var cell := first + Vector2(gx, gy)
			# posmod: 음수 칸에서도 0 또는 1 (그냥 % 는 -1 이 나와 다른 타일을 가리킨다)
			_draw_tile(cell, Tile.WATER0 + posmod(water_frame + int(cell.x + cell.y), 2))

	# 밀물 정도 (0 = 다 빠짐, 1 = 다 참)
	var flood: float = lerp(1.0 if a.tide.kind == "high" else 0.0, 1.0 if b.tide.kind == "high" else 0.0, e)

	for y in map.h:
		for x in map.w:
			var g := _ground(x, y)
			var cell := Vector2(x, y)
			var h := absi(hash(Vector2i(x, y)))
			match g:
				"#":
					if _near_open(x, y):
						_draw_tile(cell, Tile.ROCK0 + h % 2)
				"w":
					pass
				"~":
					_draw_tile(cell, Tile.FLAT0 + h % 2)
					if flood > 0.0:
						_draw_tile(cell, Tile.FLOOD0 + water_frame, Color(1, 1, 1, flood))
				"N":
					_draw_tile(cell, Tile.NEST)
				_:
					_draw_tile(cell, Tile.SAND0 + h % 2)

	var food_frame := int(_clock * 3.0) % 2
	for food in (a.foods if f < 0.5 else b.foods):
		_draw_sprite(_food, food_frame, Vector2(food.x, food.y) * T)

	for i in a.gulls.size():
		var ga: Dictionary = a.gulls[i]
		var gb: Dictionary = b.gulls[i] if i < b.gulls.size() else ga
		var pos := Vector2(ga.x, ga.y).lerp(Vector2(gb.x, gb.y), e) * T
		# 그림자 → 갈매기 (조금 떠 있다)
		draw_rect(Rect2(_screen(pos + Vector2(4, 12)), Vector2(8, 2) * _scale), Color(0, 0, 0, 0.35))
		_draw_sprite(_gull, int(_clock * 6.0 + i) % 2, pos + Vector2(0, -3), Color.WHITE, gb.dir == "W")

	var order := range(a.crabs.size())
	order.erase(selected)
	order.append(selected) # 고른 꽃게를 맨 위에
	for i in order:
		if i < a.crabs.size():
			_draw_crab(i, a.crabs[i], b.crabs[i] if i < b.crabs.size() else a.crabs[i], f, e)

	for p in _particles:
		var lk := clampf(p.life, 0.0, 1.0)
		if p.kind == "icon":
			_draw_icon(p.icon, p.cell * T + Vector2(0, -6 - p.age * 10.0), Color(p.color, lk))
		else:
			draw_rect(Rect2(_screen(p.pos), Vector2(_scale, _scale)), Color(p.color, lk))

	_draw_hud(a if f < 0.5 else b)


func _near_open(x: int, y: int) -> bool:
	for dy in range(-1, 2):
		for dx in range(-1, 2):
			var nx := x + dx
			var ny := y + dy
			if nx >= 0 and ny >= 0 and nx < map.w and ny < map.h and _ground(nx, ny) != "#":
				return true
	return false


func _draw_crab(i: int, ca: Dictionary, cb: Dictionary, f: float, e: float) -> void:
	var pa := Vector2(ca.x, ca.y)
	var pb := Vector2(cb.x, cb.y)
	var cell := pa.lerp(pb, e)
	var alpha := 1.0
	var teleport := pa.distance_to(pb) > 1.01
	if teleport:
		# 휩쓸리거나 쫓겨 굴로 돌아갈 때는 사라졌다가 굴에서 나타난다.
		cell = pa if f < 0.5 else pb
		alpha = absf(f - 0.5) * 2.0
	var moving := not teleport and pa != pb
	var art := cell * T
	if cb.mishap == "bump" and not teleport:
		art += DIR_VEC[cb.facing] * round(sin(f * PI) * 3.0)
	if moving:
		art.y -= round(sin(f * PI) * 1.0) # 걸을 때 살짝 들썩
	# 굴 위에 여러 마리가 겹치면 조금씩 비켜 그린다.
	if Vector2(cb.x, cb.y) == Vector2(map.nest[0], map.nest[1]) and not moving and map.crabs > 1:
		art.x += (i - (map.crabs - 1) / 2.0) * 4.0
	var frame := (1 if f >= 0.5 else 0) if moving else 0
	# 꽃게는 옆으로 걷는다: 동서로 가면 그대로, 남북으로 가면 90도 돌린다.
	var quarter := 1 if cb.facing in ["N", "S"] else 0
	var color := Color(1, 1, 1, alpha)

	if i == selected and map.crabs > 1:
		_draw_icon(ICON_CORNERS, art, Color(1, 1, 1, 0.55 + 0.25 * sin(_clock * 5.0)))
	draw_rect(Rect2(_screen(art + Vector2(3, 13)), Vector2(10, 2) * _scale), Color(0, 0, 0, 0.3 * alpha))
	_draw_sprite(_crabs[i % _crabs.size()], frame, art, color, false, quarter)
	var carrying: bool = ca.carrying if f < 0.5 else cb.carrying
	if carrying and alpha > 0.3:
		_draw_sprite(_food, 0, art + Vector2(0, -7), color)


func _draw_hud(fr: Dictionary) -> void:
	var font := Palette.pixel_font()
	var x := 12.0
	x = _chip(x, "틱 %d / %d" % [fr.tick, map.maxTicks], Palette.TEXT, font)
	x = _chip(x, "배달 %d / %d" % [fr.delivered, map.goal], Color("#b6ffe9"), font)
	match fr.tide.kind:
		"low":
			_chip(x, "썰물 · %d틱 뒤 밀물" % fr.tide.turnsLeft, Palette.WARN if fr.tide.turnsLeft <= 3 else Palette.SKY, font)
		"high":
			_chip(x, "밀물 · %d틱 뒤 빠짐" % fr.tide.turnsLeft, Color("#8fd0ff"), font)


func _chip(x: float, text: String, color: Color, font: Font) -> float:
	var fs := Palette.PIXEL_SIZE
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x + 20.0
	draw_rect(Rect2(x, 8, w, 24), Color(0.02, 0.05, 0.09, 0.75))
	draw_rect(Rect2(x, 8, w, 24), Color(1, 1, 1, 0.12), false, 2.0)
	var baseline := 8.0 + 12.0 + (font.get_ascent(fs) - font.get_descent(fs)) / 2.0
	draw_string(font, Vector2(x + 10, round(baseline)), text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, color)
	return x + w + 6.0


# ── 마우스 ───────────────────────────────────────────────────────

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		var fr := frame_at(int(round(time)))
		if fr.is_empty():
			return
		var best := -1
		var best_d := float(T * _scale) * 0.7
		for i in fr.crabs.size():
			var c: Dictionary = fr.crabs[i]
			var center := _screen(Vector2(c.x, c.y) * T) + Vector2(T, T) * _scale / 2.0
			var d: float = center.distance_to(event.position)
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
			agent_selected.emit(best)
			queue_redraw()
		accept_event()
