## JSON 을 읽되, 정수로 쓸 수 있는 숫자는 int 로 바꾼다.
## (Godot 의 JSON 은 모든 숫자를 float 로 읽어서 배열 인덱스나 % 연산에 쓸 수 없다.)
class_name JsonUtil


static func parse(text: String) -> Variant:
	return _fix(JSON.parse_string(text))


static func _fix(v: Variant) -> Variant:
	match typeof(v):
		TYPE_FLOAT:
			return int(v) if v == floorf(v) and absf(v) < 9.0e15 else v
		TYPE_ARRAY:
			return v.map(_fix)
		TYPE_DICTIONARY:
			var out := {}
			for k in v:
				out[k] = _fix(v[k])
			return out
	return v
