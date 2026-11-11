## 왼쪽 위의 꽃게 로고.
extends Control


func _draw() -> void:
	CrabArt.draw_crab(self, size / 2.0, minf(size.x, size.y) * 0.95, -PI / 2.0)
