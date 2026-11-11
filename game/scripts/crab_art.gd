## 꽃게와 보석을 그리는 함수들. 월드와 로고가 함께 쓴다.
class_name CrabArt

const SHELL := Color("#f4511e")
const DARK := Color("#b93a12")


## 꽃게. 오른쪽(+x)을 보는 모양을 angle 만큼 돌려 그린다. s 는 한 칸 크기.
## walk 는 0~1 걷는 동작, squash 는 착지할 때 납작해지는 정도.
static func draw_crab(ci: CanvasItem, center: Vector2, s: float, angle: float, walk := 0.0, squash := 0.0) -> void:
	var sc := Vector2(s * (1.0 + squash * 0.12), s * (1.0 - squash * 0.08))
	# 꽃게 기준 좌표(한 칸 = 1)를 화면 좌표로 바꾼다.
	var at := func(v: Vector2) -> Vector2: return center + (v * sc).rotated(angle)
	var circle := func(v: Vector2, r: float, color: Color) -> void:
		ci.draw_circle(at.call(v), r * s, color, true, -1.0, true)
	var line := func(a: Vector2, b: Vector2, color: Color, w: float) -> void:
		ci.draw_line(at.call(a), at.call(b), color, w * s, true)
	for side in [-1.0, 1.0]:
		for i in 3:
			var fx := -0.17 + i * 0.12
			var wiggle := sin(walk * PI * 4.0 + i * 2.0) * 0.035
			line.call(Vector2(fx, side * 0.18), Vector2(fx - 0.03 + wiggle, side * 0.41), DARK, 0.05)
		line.call(Vector2(0.1, side * 0.18), Vector2(0.3, side * 0.3), DARK, 0.06)
		var pinch := sin(walk * PI * 2.0) * 0.02
		circle.call(Vector2(0.3 + pinch, side * 0.3), 0.105, SHELL)
		circle.call(Vector2(0.385 + pinch, side * 0.27), 0.04, DARK)
	circle.call(Vector2.ZERO, 0.275, DARK)
	circle.call(Vector2.ZERO, 0.245, SHELL)
	circle.call(Vector2(-0.06, -0.08), 0.09, SHELL.lightened(0.18))
	for side in [-1.0, 1.0]:
		circle.call(Vector2(0.16, side * 0.105), 0.078, Color.WHITE)
		circle.call(Vector2(0.19, side * 0.105), 0.04, Color("#1a1a22"))


static func diamond(ci: CanvasItem, c: Vector2, r: float, color: Color) -> void:
	var pts := PackedVector2Array([c + Vector2(r, 0), c + Vector2(0, r), c + Vector2(-r, 0), c + Vector2(0, -r)])
	ci.draw_colored_polygon(pts, color)
	pts.append(pts[0])
	ci.draw_polyline(pts, color, 1.0, true)


static func gem_color(g: Dictionary) -> Color:
	var base: Color = Palette.GEM[g.color]
	return base if g.shiny else base.lerp(Color("#666a7c"), 0.72)


static func draw_gem(ci: CanvasItem, c: Vector2, r: float, g: Dictionary) -> void:
	var base: Color = Palette.GEM[g.color]
	diamond(ci, c, r, base.lerp(Color.BLACK, 0.25 if g.shiny else 0.6))
	diamond(ci, c, r * 0.84, gem_color(g))
	if g.shiny:
		diamond(ci, c + Vector2(-r * 0.12, -r * 0.18), r * 0.36, base.lerp(Color.WHITE, 0.55))
