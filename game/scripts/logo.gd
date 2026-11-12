## 왼쪽 위의 해바라기 로고 (Sunnyside World 그림).
extends Control

var _tex: Texture2D = load("res://logo.png")


func _init() -> void:
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST


func _draw() -> void:
	var s := floorf(minf(size.x, size.y) / 16.0) * 16.0
	draw_texture_rect(_tex, Rect2((size - Vector2(s, s)) / 2.0, Vector2(s, s)), false)
