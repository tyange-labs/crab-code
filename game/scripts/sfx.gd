## 효과음. 소리 파일 없이 시작할 때 파형을 직접 만들어 쓴다.
extends Node

const RATE := 22050

var enabled := true
var _sounds := {}
var _players: Array[AudioStreamPlayer] = []


func _ready() -> void:
	# [주파수 시작, 끝, 길이(초), 파형, 음량]
	_sounds.step = _tone([[520.0, 440.0, 0.05, "square", 0.10]])
	_sounds.turn = _tone([[380.0, 460.0, 0.05, "square", 0.08]])
	_sounds.collect = _tone([[880.0, 880.0, 0.06, "square", 0.14], [1320.0, 1320.0, 0.10, "square", 0.14]])
	_sounds.deposit = _tone([[660.0, 660.0, 0.06, "triangle", 0.25], [990.0, 990.0, 0.12, "triangle", 0.25]])
	_sounds.polish = _tone([[1200.0, 2400.0, 0.14, "sine", 0.18]])
	_sounds.unlock = _tone([[440.0, 440.0, 0.06, "square", 0.12], [660.0, 660.0, 0.06, "square", 0.12], [880.0, 880.0, 0.12, "square", 0.12]])
	_sounds.bump = _tone([[140.0, 60.0, 0.18, "noise", 0.30]])
	_sounds.fail = _tone([[330.0, 330.0, 0.12, "triangle", 0.25], [247.0, 247.0, 0.12, "triangle", 0.25], [196.0, 160.0, 0.25, "triangle", 0.25]])
	_sounds.win = _tone([
		[523.0, 523.0, 0.09, "square", 0.12], [659.0, 659.0, 0.09, "square", 0.12],
		[784.0, 784.0, 0.09, "square", 0.12], [1047.0, 1047.0, 0.28, "square", 0.12],
	])
	_sounds.compile = _tone([[300.0, 600.0, 0.08, "sine", 0.12]])
	for i in 6:
		var p := AudioStreamPlayer.new()
		add_child(p)
		_players.append(p)


func play(sound: String, pitch := 1.0) -> void:
	if not enabled or not _sounds.has(sound):
		return
	for p in _players:
		if not p.playing:
			p.stream = _sounds[sound]
			p.pitch_scale = pitch
			p.play()
			return


func _tone(parts: Array) -> AudioStreamWAV:
	var data := PackedByteArray()
	var phase := 0.0
	for part in parts:
		var n := int(part[2] * RATE)
		for i in n:
			var t := float(i) / n
			var freq: float = lerp(part[0], part[1], t)
			phase += freq / RATE
			var v := 0.0
			match part[3]:
				"square": v = 1.0 if fmod(phase, 1.0) < 0.5 else -1.0
				"triangle": v = 4.0 * absf(fmod(phase, 1.0) - 0.5) - 1.0
				"sine": v = sin(phase * TAU)
				"noise": v = randf_range(-1.0, 1.0) * 0.6 + sin(phase * TAU) * 0.4
			# 시작과 끝을 부드럽게 해서 딸깍 소리를 없앤다.
			var env := minf(1.0, t * 40.0) * pow(1.0 - t, 1.5)
			var s := int(clampf(v * env * part[4], -1.0, 1.0) * 32767.0)
			data.append(s & 0xff)
			data.append((s >> 8) & 0xff)
	var wav := AudioStreamWAV.new()
	wav.format = AudioStreamWAV.FORMAT_16_BITS
	wav.mix_rate = RATE
	wav.data = data
	return wav
