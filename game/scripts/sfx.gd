## 효과음. 소리 파일 없이 시작할 때 파형을 직접 만들어 쓴다.
extends Node

const RATE := 22050

## 소리는 기본으로 꺼져 있다 (설정이나 윗줄 버튼으로 켠다).
var enabled := false
var _sounds := {}
var _players: Array[AudioStreamPlayer] = []


func _ready() -> void:
	# [주파수 시작, 끝, 길이(초), 파형, 음량]  — 어두운 SF: 전자음·비프·알람
	_sounds.step = _tone([[420.0, 380.0, 0.04, "triangle", 0.08]])
	_sounds.turn = _tone([[300.0, 360.0, 0.04, "triangle", 0.07]])
	_sounds.grab = _tone([[700.0, 1100.0, 0.05, "sine", 0.16], [1400.0, 1400.0, 0.07, "sine", 0.14]])
	_sounds.insert = _tone([[180.0, 360.0, 0.10, "saw", 0.12], [880.0, 1320.0, 0.14, "sine", 0.20]])
	_sounds.install = _tone([[1500.0, 1500.0, 0.03, "square", 0.08], [900.0, 900.0, 0.05, "square", 0.08]])
	_sounds.charge = _tone([[300.0, 1500.0, 0.22, "saw", 0.10]])
	_sounds.boot = _tone([[660.0, 660.0, 0.05, "sine", 0.18], [990.0, 990.0, 0.05, "sine", 0.18], [1320.0, 1320.0, 0.12, "sine", 0.18]])
	_sounds.bump = _tone([[110.0, 60.0, 0.16, "noise", 0.28]])
	_sounds.alarm = _tone([[880.0, 880.0, 0.09, "square", 0.14], [660.0, 660.0, 0.09, "square", 0.14], [880.0, 880.0, 0.09, "square", 0.14], [660.0, 560.0, 0.16, "square", 0.14]])
	_sounds.win = _tone([
		[440.0, 440.0, 0.08, "sine", 0.18], [554.0, 554.0, 0.08, "sine", 0.18],
		[659.0, 659.0, 0.08, "sine", 0.18], [880.0, 880.0, 0.30, "sine", 0.20],
	])
	_sounds.fail = _tone([[400.0, 300.0, 0.14, "saw", 0.12], [300.0, 180.0, 0.28, "saw", 0.12]])
	_sounds.compile = _tone([[500.0, 800.0, 0.06, "sine", 0.10]])
	# 예전 이름도 남겨 둔다 (다른 팩이 쓸 수 있다).
	_sounds.collect = _sounds.grab
	_sounds.deposit = _sounds.insert
	_sounds.polish = _sounds.boot
	_sounds.unlock = _sounds.boot
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
				"saw": v = 2.0 * fmod(phase, 1.0) - 1.0
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
