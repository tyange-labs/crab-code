## 정거장 복구 게임팩의 화면: 어두운 SF 16x16 픽셀 아트로 버려진 정거장을 그린다.
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
## 그림은 assets/ 에 있다. 지형·로봇·경비 드론·코어·장식은 ../art/draw.py 가 코드로 그린 것이고,
## 선택 표시 같은 아이콘은 Kenney 의 1-Bit Pack (CC0, assets/kenney/) 에서 가져왔다.
## 모든 것을 16픽셀 격자에 그린 뒤 정수 배율로 키워서 픽셀이 뭉개지지 않게 한다.
extends Control

# HUD 글자색: 어두운 세계 위에 그리므로 UI 테마(Palette)와 따로 둔다.
const HUD_TEXT := Color("#e4e6f0")
const HUD_DIM := Color("#9aa0ba")
const HUD_FAINT := Color("#7f85a3")
const HUD_CODE := Color("#ffc66d")
const HUD_WARN := Color("#f2c94c")
const HUD_SKY := Color("#5cb8ff")
const HUD_GREEN := Color("#5fd38d")


## 로봇을 눌렀을 때.
signal agent_selected(index: int)

const T := 16 # 그림 한 칸의 픽셀 수
const ASSETS := "res://packs/station/view/assets/"
const FARMER_FILES := ["bot_0", "bot_1", "bot_2", "bot_3"]
const ACTOR := 24 # 로봇·경비 드론 그림 한 프레임의 픽셀 수
const DIR_VEC := {"N": Vector2(0, -1), "E": Vector2(1, 0), "S": Vector2(0, 1), "W": Vector2(-1, 0)}
# tiles.png 안의 순서
enum Tile { GRASS0, GRASS1, DECK0, DECK1, DUCT0, DUCT1, WALL0, WALL1, VOID0, VOID1, VENT0, VENT1, REACTOR, SLOT }
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
var _sentry: Texture2D = load(ASSETS + "sentry.png")
var _plants: Texture2D = load(ASSETS + "plants.png")
var _hud_box := _make_hud_box()
var _deco := {}
var _icons: Texture2D = load(ASSETS + "kenney/1bit.png")
var _bots: Array = FARMER_FILES.map(func(f): return load(ASSETS + f + ".png"))


const PLANT_KINDS := 6


func _make_hud_box() -> StyleBoxTexture:
	var sb := StyleBoxTexture.new()
	sb.texture = load(ASSETS + "hud_box.png")
	sb.set_texture_margin_all(3)
	sb.draw_center = true
	return sb


func _tex(name: String) -> Texture2D:
	if not _deco.has(name):
		_deco[name] = load(ASSETS + name + ".png")
	return _deco[name]


func _init() -> void:
	mouse_filter = Control.MOUSE_FILTER_STOP
	clip_contents = true
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST


func setup(new_map: Dictionary, new_frames: Array) -> void:
	map = new_map
	frames = new_frames
	time = 0.0
	_particles.clear()
	selected = clampi(selected, 0, maxi(map.get("bots", 1) - 1, 0))
	queue_redraw()


func agent_count() -> int:
	return map.get("bots", 0)


func describe_agent(i: int, t: int) -> String:
	var f := frame_at(t)
	if f.is_empty() or i >= f.bots.size():
		return ""
	var c: Dictionary = f.bots[i]
	return "(%d, %d)%s" % [c.x, c.y, " · 코어 들고 있음" if c.carrying else ""]


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
	for i in f.bots.size():
		var c: Dictionary = f.bots[i]
		var was: Dictionary = prev.bots[i] if not prev.is_empty() else c
		var at := Vector2(c.x, c.y)
		match c.mishap:
			"swept":
				_exp_float(Vector2(was.x, was.y), "exp_stress")
				_burst(Vector2(was.x, was.y), Color("#6ca8d8"), 14)
				Sfx.play("bump", 0.7)
			"caught":
				_exp_float(Vector2(was.x, was.y), "exp_alert")
				Sfx.play("fail", 1.4)
			"bump":
				_exp_float(at, "exp_confused")
				Sfx.play("bump", 1.3)
		if not prev.is_empty() and c.carrying and not was.carrying:
			_exp_float(at, "exp_love")
			_burst(at, Color("#ffd96b"), 8)
			Sfx.play("collect", 1.0 + i * 0.08)
	if not prev.is_empty() and f.delivered > prev.delivered:
		var reactor := Vector2(map.reactor[0], map.reactor[1])
		_exp_float(reactor, "exp_love")
		_burst(reactor, Palette.ACCENT, 18)
		Sfx.play("deposit")
	# 슬롯: 심기·물 주기·코어가 열리는 순간
	if not prev.is_empty() and f.has("slots") and prev.has("slots"):
		for k in f.slots.size():
			var now: Dictionary = f.slots[k]
			var was_p: Dictionary = prev.slots[k]
			var cell := Vector2(now.x, now.y)
			if now.stage != was_p.stage:
				if now.stage == "seed":
					_exp_float(cell, "exp_working")
					_burst(cell, Color("#8a5a35"), 8)
					Sfx.play("collect", 0.7)
				elif now.stage == "booting" and was_p.stage == "seed":
					_exp_float(cell, "exp_working")
					_burst(cell, Color("#6cc4ff"), 14)
					Sfx.play("bump", 1.8)
				elif now.stage == "empty" and was_p.stage == "booting":
					_burst(cell, Color("#ffe27a"), 16)
					Sfx.play("deposit", 1.4)


## 칸 위로 떠오르며 사라지는 말풍선 (Sunnyside 의 expression 아이콘).
func _exp_float(cell: Vector2, name: String) -> void:
	_particles.append({"kind": "exp", "cell": cell, "tex": name, "life": 1.1, "age": 0.0})


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
	var colors := [Palette.ACCENT, HUD_GREEN, HUD_SKY, HUD_WARN, Color("#35e0ff")]
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
	# 위쪽 정보 줄은 지도 모서리(돌) 위에 겹쳐 그린다 (그래야 정수 배율을 한 단계 크게 쓸 수 있다).
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


## 로봇·경비 드론처럼 한 칸보다 큰(24x24) 프레임. 발이 칸 아래쪽에 오도록 맞춘다.
func _draw_actor(tex: Texture2D, col: int, row: int, art_px: Vector2, color := Color.WHITE, flip := false) -> void:
	var size_px := Vector2(ACTOR, ACTOR) * _scale
	var pos := _screen(art_px + Vector2(-4, -9))
	var src := Rect2(col * ACTOR, row * ACTOR, ACTOR, ACTOR)
	if flip:
		draw_set_transform(pos + Vector2(size_px.x, 0), 0.0, Vector2(-1, 1))
		draw_texture_rect_region(tex, Rect2(Vector2.ZERO, size_px), src, color)
		draw_set_transform(Vector2.ZERO)
	else:
		draw_texture_rect_region(tex, Rect2(pos, size_px), src, color)


func _draw_icon(icon: Vector2i, art_px: Vector2, color: Color) -> void:
	var dst := Rect2(_screen(art_px), Vector2(T, T) * _scale)
	draw_texture_rect_region(_icons, dst, Rect2(icon.x * T, icon.y * T, T, T), color)


# ── 그리기 ───────────────────────────────────────────────────────

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color("#070a14"))
	if map.is_empty() or frames.is_empty() or size.x < 60.0 or size.y < 60.0:
		return
	_layout()
	var k := int(floor(time))
	var f := clampf(time - k, 0.0, 1.0)
	var a := frame_at(k)
	var b := frame_at(k + 1)
	var e := f * f * (3.0 - 2.0 * f)
	var water_frame := int(_clock * 1.6) % 2

	# 화면 전체를 우주 공간으로 덮는다 (지도 격자에 맞춰서).
	var first := ((-_origin) / (T * _scale)).floor() - Vector2.ONE
	var count := (size / (T * _scale)).ceil() + Vector2(3, 3)
	for gy in int(count.y):
		for gx in int(count.x):
			var cell := first + Vector2(gx, gy)
			# posmod: 음수 칸에서도 0 또는 1 (그냥 % 는 -1 이 나와 다른 타일을 가리킨다)
			_draw_tile(cell, Tile.GRASS0 + posmod(int(cell.x * 3 + cell.y * 7) / 2, 2))

	_draw_decor(first, count)

	# 감압 정도 (0 = 다 빠짐, 1 = 다 참)
	var pressure: float = lerp(1.0 if a.pressure.kind == "high" else 0.0, 1.0 if b.pressure.kind == "high" else 0.0, e)

	for y in map.h:
		for x in map.w:
			var g := _ground(x, y)
			var cell := Vector2(x, y)
			var h := absi(hash(Vector2i(x, y)))
			match g:
				"#":
					if _near_open(x, y):
						_draw_tile(cell, Tile.WALL0 + h % 2)
				"w":
					_draw_tile(cell, Tile.VOID0 + posmod(water_frame + x + y, 2))
				"~":
					_draw_tile(cell, Tile.DUCT0 + h % 2)
					if pressure > 0.0:
						_draw_tile(cell, Tile.VENT0 + water_frame, Color(1, 1, 1, pressure))
				"N":
					_draw_tile(cell, Tile.REACTOR)
				"p":
					_draw_tile(cell, Tile.SLOT)
				_:
					_draw_tile(cell, Tile.DECK0 + h % 2)

	# 슬롯에서 부팅되는 코어 (모듈 → 부팅 중 → 완성)
	if a.has("slots"):
		for slot in (a.slots if f < 0.5 else b.slots):
			var stage := -1
			match slot.stage:
				"seed":
					stage = 0
				"booting":
					stage = clampi(1 + (4 - int(slot.left)), 1, 4)
			if stage >= 0:
				_draw_plant(_core_kind(slot.x, slot.y), stage, Vector2(slot.x, slot.y) * T)
	# 다 자란 코어: 위아래로 살짝 들썩이고 가끔 반짝인다.
	var bob := -1.0 if int(_clock * 3.0) % 2 == 1 else 0.0
	for core in (a.cores if f < 0.5 else b.cores):
		var at := Vector2(core.x, core.y) * T
		_draw_plant(_core_kind(core.x, core.y), 5, at + Vector2(0, bob))
		var gl := int(_clock * 5.0 + core.x * 3 + core.y * 5) % 24
		if gl < 6:
			var glint := _tex("deco_glint")
			draw_texture_rect_region(glint, Rect2(_screen(at + Vector2(9, 1)), Vector2(7, 7) * _scale), Rect2(gl * 7, 0, 7, 7))

	for i in a.sentries.size():
		var ga: Dictionary = a.sentries[i]
		var gb: Dictionary = b.sentries[i] if i < b.sentries.size() else ga
		var pos := Vector2(ga.x, ga.y).lerp(Vector2(gb.x, gb.y), e) * T
		# 그림자 → 경비 드론 (조금 떠 있다)
		draw_rect(Rect2(_screen(pos + Vector2(3, 13)), Vector2(10, 2) * _scale), Color(0, 0, 0, 0.25))
		_draw_actor(_sentry, int(_clock * 8.0 + i) % 8, 0, pos, Color.WHITE, gb.dir == "W")

	var order := range(a.bots.size())
	order.erase(selected)
	order.append(selected) # 고른 로봇을 맨 위에
	for i in order:
		if i < a.bots.size():
			_draw_bot(i, a.bots[i], b.bots[i] if i < b.bots.size() else a.bots[i], f, e)

	if map.get("fog", false):
		_draw_fog(a)

	for p in _particles:
		var lk := clampf(p.life, 0.0, 1.0)
		if p.kind == "exp":
			var t: Texture2D = _tex(p.tex)
			var pos := _screen(p.cell * T + Vector2((T - t.get_width()) / 2.0, -9 - p.age * 8.0))
			draw_texture_rect(t, Rect2(pos, t.get_size() * _scale), false, Color(1, 1, 1, lk))
		elif p.kind == "icon":
			_draw_icon(p.icon, p.cell * T + Vector2(0, -6 - p.age * 10.0), Color(p.color, lk))
		else:
			draw_rect(Rect2(_screen(p.pos), Vector2(_scale, _scale)), Color(p.color, lk))

	_draw_hud(a if f < 0.5 else b)


## 코어 색은 칸마다 정해서 단조롭지 않게 한다. (청록이 가장 많다.)
func _core_kind(x: int, y: int) -> int:
	var h := absi(hash(Vector2i(x, y)))
	return 0 if h % 3 != 0 else 1 + (h / 3) % (PLANT_KINDS - 1)


func _draw_plant(kind: int, stage: int, art_px: Vector2, color := Color.WHITE) -> void:
	var dst := Rect2(_screen(art_px), Vector2(T, T) * _scale)
	draw_texture_rect_region(_plants, dst, Rect2(stage * T, kind * T, T, T), color)


## 지도 밖 우주 공간에 잔해·신호등·안테나를 흩뿌린다 (칸 번호로 정해서 늘 같은 자리).
func _draw_decor(first: Vector2, count: Vector2) -> void:
	for gy in int(count.y):
		for gx in int(count.x):
			var cell := first + Vector2(gx, gy)
			if cell.x >= -1 and cell.y >= -1 and cell.x <= map.w and cell.y <= map.h:
				continue
			var h := absi(hash(Vector2i(int(cell.x), int(cell.y))))
			var base := cell * T
			var r := h % 29
			var name := ""
			var rate := 3.0
			match r:
				0, 1:
					name = "deco_debris"
					# 잔해는 천천히 떠돈다
					base += Vector2(sin(_clock * 0.4 + h) * 3.0, cos(_clock * 0.3 + h) * 2.0)
				2:
					name = "deco_beacon"
					rate = 2.0
				3:
					name = "deco_dish"
					rate = 1.0
			if name == "":
				continue
			var frame := int(_clock * rate + h % 4) % 4
			draw_texture_rect_region(_tex(name), Rect2(_screen(base), Vector2(T, T) * _scale), Rect2(frame * 16, 0, 16, 16))


## 안개: 어느 로봇에게서도 시야(걸음 수) 밖인 칸을 어둡게 가린다. (지도 안쪽만)
func _draw_fog(fr: Dictionary) -> void:
	var sight: int = map.get("sight", 7)
	for y in map.h:
		for x in map.w:
			var seen := false
			for c in fr.bots:
				if absi(c.x - x) + absi(c.y - y) <= sight:
					seen = true
					break
			if not seen:
				draw_rect(Rect2(_screen(Vector2(x, y) * T), Vector2(T, T) * _scale), Color(0.02, 0.03, 0.08, 0.7))


func _near_open(x: int, y: int) -> bool:
	for dy in range(-1, 2):
		for dx in range(-1, 2):
			var nx := x + dx
			var ny := y + dy
			if nx >= 0 and ny >= 0 and nx < map.w and ny < map.h and _ground(nx, ny) != "#":
				return true
	return false


func _draw_bot(i: int, ca: Dictionary, cb: Dictionary, f: float, e: float) -> void:
	var pa := Vector2(ca.x, ca.y)
	var pb := Vector2(cb.x, cb.y)
	var cell := pa.lerp(pb, e)
	var alpha := 1.0
	var teleport := pa.distance_to(pb) > 1.01
	if teleport:
		# 휩쓸리거나 쫓겨 반응로로 돌아갈 때는 사라졌다가 반응로에서 나타난다.
		cell = pa if f < 0.5 else pb
		alpha = absf(f - 0.5) * 2.0
	var moving := not teleport and pa != pb
	var art := cell * T
	if cb.mishap == "bump" and not teleport:
		art += DIR_VEC[cb.facing] * round(sin(f * PI) * 3.0)
	if moving:
		art.y -= round(sin(f * PI) * 1.0) # 걸을 때 살짝 들썩
	# 반응로 위에 여러 마리가 겹치면 조금씩 비켜 그린다.
	if Vector2(cb.x, cb.y) == Vector2(map.reactor[0], map.reactor[1]) and not moving and map.bots > 1:
		art.x += (i - (map.bots - 1) / 2.0) * 4.0
	var carrying: bool = ca.carrying if f < 0.5 else cb.carrying
	# 걷는 동안은 걷기 8프레임을 타임라인 위치(f)에 맞춰, 멈춰 있으면 가만히 있는 9프레임을 돌린다.
	var col := int(f * 7.99) if moving else int(_clock * 6.0) % 9
	var anim_row := (1 if carrying else 0) if moving else 2
	if not moving and carrying:
		anim_row = 1
		col = 0
	var color := Color(1, 1, 1, alpha)

	if i == selected and map.bots > 1:
		_draw_icon(ICON_CORNERS, art, Color(1, 1, 1, 0.55 + 0.25 * sin(_clock * 5.0)))
	draw_rect(Rect2(_screen(art + Vector2(3, 13)), Vector2(10, 2) * _scale), Color(0, 0, 0, 0.3 * alpha))
	_draw_actor(_bots[i % _bots.size()], col, anim_row, art, color, cb.facing == "W")
	if carrying and alpha > 0.3:
		_draw_plant(0, 5, art + Vector2(0, -12), color)


func _draw_hud(fr: Dictionary) -> void:
	var font := Palette.pixel_font()
	var x := 12.0
	x = _chip(x, "틱 %d / %d" % [fr.tick, map.maxTicks], HUD_TEXT, font)
	x = _chip(x, "장착 %d / %d" % [fr.delivered, map.goal], Color("#35e0ff"), font)
	match fr.pressure.kind:
		"low":
			_chip(x, "기압 정상 · %d틱 뒤 감압" % fr.pressure.turnsLeft, HUD_WARN if fr.pressure.turnsLeft <= 3 else HUD_SKY, font)
		"high":
			_chip(x, "진공 · %d틱 뒤 정상" % fr.pressure.turnsLeft, Color("#ff7a8a"), font)


func _chip(x: float, text: String, color: Color, font: Font) -> float:
	var fs := Palette.PIXEL_SIZE
	var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x + 20.0
	draw_style_box(_hud_box, Rect2(x, 8, w, 24))
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
		for i in fr.bots.size():
			var c: Dictionary = fr.bots[i]
			var center := _screen(Vector2(c.x, c.y) * T) + Vector2(T, T) * _scale / 2.0
			var d: float = center.distance_to(event.position)
			if d < best_d:
				best_d = d
				best = i
		if best >= 0:
			# 반응로 위처럼 겹쳐 있으면 누를 때마다 다음 로봇으로.
			if best == selected and fr.bots.size() > 1:
				var here: Dictionary = fr.bots[best]
				for step in range(1, fr.bots.size()):
					var j: int = (best + step) % fr.bots.size()
					if fr.bots[j].x == here.x and fr.bots[j].y == here.y:
						best = j
						break
			selected = best
			agent_selected.emit(best)
			queue_redraw()
		accept_event()
