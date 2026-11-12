#!/usr/bin/env python3
"""조수 웅덩이 게임팩의 픽셀 아트 (16x16, 밤바다).

그림은 아래의 글자 격자로 그린다. 글자 하나가 픽셀 하나이고, 무슨 색인지는 PALETTE 에 있다.
고친 뒤 이 파일을 실행하면 ../view/assets/ 의 PNG 가 다시 만들어진다.

    python3 game/packs/tidepool/art/sprites.py

PIL 없이 표준 라이브러리만 쓴다.
"""

import os
import struct
import zlib

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "view", "assets")

# 글자 -> (r, g, b, a). '.' 는 투명.
PALETTE = {
    ".": (0, 0, 0, 0),
    # 모래 (달빛 받은 젖은 모래)
    "a": (0x6b, 0x5d, 0x4c, 255),
    "b": (0x62, 0x55, 0x45, 255),
    "c": (0x7e, 0x6f, 0x5b, 255),
    "d": (0x55, 0x49, 0x3a, 255),
    # 갯벌
    "e": (0x4a, 0x52, 0x63, 255),
    "f": (0x43, 0x4a, 0x5a, 255),
    "h": (0x5f, 0x6a, 0x7e, 255),
    # 바위
    "k": (0x1d, 0x1f, 0x2c, 255),
    "m": (0x2c, 0x2f, 0x43, 255),
    "n": (0x3c, 0x41, 0x5c, 255),
    "p": (0x55, 0x5c, 0x80, 255),
    # 물
    "q": (0x0f, 0x26, 0x3d, 255),
    "r": (0x15, 0x34, 0x52, 255),
    "s": (0x2a, 0x5a, 0x86, 255),
    "t": (0x6c, 0xa8, 0xd8, 255),
    # 밀물에 잠긴 갯벌 (반투명 물)
    "u": (0x1a, 0x45, 0x6e, 215),
    "v": (0x2a, 0x5a, 0x86, 225),
    "w": (0x6c, 0xa8, 0xd8, 200),
    # 굴
    "x": (0x16, 0x0f, 0x0b, 255),
    "y": (0x34, 0x25, 0x1b, 255),
    "z": (0x55, 0x3d, 0x2c, 255),
    "Z": (0xf4, 0x6a, 0x25, 255),
    # 꽃게 (색은 CRAB_COLORS 로 바꿔 칠한다)
    "D": (0x8f, 0x2e, 0x10, 255),
    "S": (0xe5, 0x54, 0x1f, 255),
    "L": (0xff, 0x8a, 0x4c, 255),
    "W": (0xf4, 0xf1, 0xe8, 255),
    "P": (0x1a, 0x1a, 0x22, 255),
    # 갈매기
    "G": (0xee, 0xf1, 0xf6, 255),
    "H": (0xaa, 0xb3, 0xc5, 255),
    "J": (0x5d, 0x66, 0x80, 255),
    "Y": (0xf2, 0xb1, 0x3c, 255),
    # 먹이 (빛나는 플랑크톤)
    "o": (0x2f, 0x8f, 0x7a, 140),
    "g": (0x6f, 0xe0, 0xc0, 220),
    "C": (0xb6, 0xff, 0xe9, 255),
    "O": (0xff, 0xff, 0xff, 255),
}

# 꽃게 색: (어두운 색, 껍데기, 밝은 색). 팩의 pack.json colors 와 같은 순서.
CRAB_COLORS = {
    "orange": ((0x8f, 0x2e, 0x10), (0xe5, 0x54, 0x1f), (0xff, 0x8a, 0x4c)),
    "teal": ((0x1d, 0x63, 0x5c), (0x3f, 0xb6, 0xa8), (0x8c, 0xe6, 0xd8)),
    "yellow": ((0x86, 0x66, 0x14), (0xe2, 0xb3, 0x3c), (0xff, 0xe0, 0x84)),
    "purple": ((0x5a, 0x34, 0x8e), (0xb0, 0x7c, 0xf0), (0xd9, 0xbf, 0xff)),
}

# ── 지형 타일 (16x16) ────────────────────────────────────────────

SAND_0 = """
aaaaaaaaaaaaaaaa
aaaacaaaaaaaaaba
aaaaaaaaabaaaaaa
aabaaaaaaaaaaaaa
aaaaaaaaaaaacaaa
aaaaaadaaaaaaaaa
aaaaaaaaaaaaaaaa
acaaaaaaaaabaaaa
aaaaaaaaaaaaaaaa
aaaaaabaaaaaaaaa
aaaaaaaaaacaaaaa
aaadaaaaaaaaaaba
aaaaaaaaaaaaaaaa
aaaaaaaaabaaaaaa
aabaaaacaaaaaaaa
aaaaaaaaaaaaadaa
"""

SAND_1 = """
bbbbbbbbbbbbbbbb
bbbbbbbbcbbbbbbb
bbabbbbbbbbbbbbb
bbbbbbbbbbbbbabb
bbbbbdbbbbbbbbbb
bbbbbbbbbbbcbbbb
bbcbbbbbbbbbbbbb
bbbbbbbbabbbbbbb
bbbbbbbbbbbbbbdb
bbbbbabbbbbbbbbb
bbbbbbbbbbbbbbbb
bdbbbbbbbbcbbbbb
bbbbbbbbbbbbbbbb
bbbbbbbcbbbbbbbb
bbbbbbbbbbbbbabb
bbbbabbbbbbbbbbb
"""

FLAT_0 = """
eeeeeeeeeeeeeeee
eeeeeeeeeeeeeeee
eeehhhheeeeeeeee
eeeeeeeeeeeeeeee
eeeeeeeeeeeffeee
eeeeeeeeeeeeeeee
eeeeeeeeeeeeeeee
eeeeeeehhhhheeee
eeeeeeeeeeeeeeee
effeeeeeeeeeeeee
eeeeeeeeeeeeeeee
eeeeeeeeeeeeeeee
eeeeehhheeeeeeee
eeeeeeeeeeeeefee
eeeeeeeeeeeeeeee
eeeeeeeeeeeeeeee
"""

FLAT_1 = """
ffffffffffffffff
ffffffffffffffff
fffffffffhhhhfff
ffffffffffffffff
ffeeffffffffffff
ffffffffffffffff
ffffffffffffffff
ffhhhhhfffffffff
ffffffffffffffff
fffffffffffeefff
ffffffffffffffff
ffffffffhhhhffff
ffffffffffffffff
ffffffffffffffff
fffeffffffffffff
ffffffffffffffff
"""

ROCK_0 = """
................
.....mmmmm......
...mmnnnnnmm....
..mnnppnnnnnm...
..mnppnnnnnnmm..
.mnnpnnnnnnnnm..
.mnnnnnnnnnnnmm.
.mnnnnnnnnnnnnm.
.mmnnnnnnnnnnnm.
kmmmnnnnnnnnmmm.
kkmmmmnnnnmmmmk.
.kkkmmmmmmmmmkk.
..kkkkkmmmmkkk..
....kkkkkkkkk...
................
................
"""

ROCK_1 = """
................
................
.........mmmm...
..mmmm..mnnnnm..
.mnppnm.mnpnnmm.
.mnpnnnmmnnnnnm.
mnnnnnnnmnnnnnm.
mnnnnnnnmmnnnmk.
mmnnnnnmmkmmmkk.
kmmnnnmmkkkkkk..
kkmmmmmkkk......
.kkkkkkkk..mmm..
..........mnpnm.
..........mnnnmk
...........kmkk.
................
"""

WATER_0 = """
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqrrrqqqqqqqqqq
qqrqqqrqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqrrrqqq
qqqqqqqqqrqqqrqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqsqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqqqqrrrqqqqqq
qqqqqqrqqqrqqqqq
qqqqqqqqqqqqqtqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
"""

WATER_1 = """
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqrrrqqqqqqqqq
qqqrqqqrqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqqrrrqq
qqqqqqqqqqrqqqrq
qqqqqqqqqqqqqqqq
qqqqsqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqqq
qqqqqqqqrrrqqqqq
qqqqqqqrqqqrqqqq
qqqqqqqqqqqqqqqq
qqqqqqqqqqqqqqtq
qqqqqqqqqqqqqqqq
"""

FLOOD_0 = """
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuvvvuuuuuuuuuu
uuvuuuvuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuvvvuuu
uuuuuuuuuvuuuvuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuwuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuuuuvvvuuuuuu
uuuuuuvuuuvuuuuu
uuuuuuuuuuuuuwuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
"""

FLOOD_1 = """
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuvvvuuuuuuuuu
uuuvuuuvuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuuvvvuu
uuuuuuuuuuvuuuvu
uuuuuuuuuuuuuuuu
uuuuwuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuuu
uuuuuuuuvvvuuuuu
uuuuuuuvuuuvuuuu
uuuuuuuuuuuuuuuu
uuuuuuuuuuuuuuwu
uuuuuuuuuuuuuuuu
"""

NEST = """
aaaaaaaaaaaaaaaa
aaaaaaaaaaaaaaaa
aaaaazzzzzzaaaaa
aaazzyyyyyyzzaaa
aazyyyxxxxyyyzaa
aazyxxxxxxxxyzaa
azyxxxxxxxxxxyza
azyxxxxxxxxxxyza
azyxxxxxxxxxxyza
azyyxxxxxxxxyyza
aazyyxxxxxxyyzaa
aaazzyyyyyyzzaaa
aaaaazzzzzzaaaaa
aaaaaaaaaaaaaaaa
aaaaaaaaaaaaaaaa
aaaaaaaaaaaaaaaa
"""

# ── 꽃게: 위에서 본 모습, 집게가 북쪽 (2프레임 걷기) ───────────────────

CRAB_0 = """
................
.LS..........SL.
.SSD........DSS.
.DSS........SSD.
..DSD.W..W.DSD..
...D..P..P..D...
....DSSSSSSD....
...DSLLSSSSSD...
.D.SSLSSSSSSS.D.
D.DSSSSSSSSSSD.D
.D.SSSSSSSSSS.D.
D..DSSSSSSSSD..D
.D..DDSSSSDD..D.
...D..DDDD..D...
..D..........D..
................
"""

CRAB_1 = """
................
.LS..........SL.
.SSD........DSS.
.DSS........SSD.
..DSD.W..W.DSD..
...D..P..P..D...
....DSSSSSSD....
...DSLLSSSSSD...
D..SSLSSSSSSS..D
.DDSSSSSSSSSSDD.
D..SSSSSSSSSS..D
.D.DSSSSSSSSD.D.
D...DDSSSSDD...D
....D.DDDD.D....
................
................
"""

# ── 갈매기: 옆에서 본 모습, 동쪽을 본다 (날개 위/아래) ──────────────────

GULL_0 = """
................
...G............
...GH...........
....GH..........
....GGH.........
.....GGH..GGG...
......GGGGGGGPY.
...HGGGGGGGGGYY.
....HHGGGGGG....
......HHHH......
.......J..J.....
................
................
................
................
................
"""

GULL_1 = """
................
................
................
................
................
..........GGG...
......GGGGGGGPY.
...HGGGGGGGGGYY.
....HHGGGGGG....
.....GGHHH......
....GGH.J.J.....
...GH...........
...G............
................
................
................
"""

# ── 먹이: 빛나는 플랑크톤 (2프레임 깜빡임) ──────────────────────────

FOOD_0 = """
................
................
................
................
......oooo......
.....ogggo......
....oggCCgo.....
....ogCOOCgo....
....ogCOOCgo....
....oggCCggo....
.....ogggo......
......oooo......
................
................
................
................
"""

FOOD_1 = """
................
................
................
................
................
......gggg......
.....ggCCgg.....
.....gCOOCg.....
.....gCOOCg.....
.....ggCCgg.....
......gggg......
................
................
................
................
................
"""


def grid(text):
    rows = [r for r in text.strip("\n").split("\n")]
    assert len(rows) == 16, f"16줄이어야 해요: {len(rows)}"
    for r in rows:
        assert len(r) == 16, f"16칸이어야 해요: {r!r}"
    return rows


def pixels(text, recolor=None):
    out = []
    for row in grid(text):
        line = []
        for ch in row:
            rgba = PALETTE[ch]
            if recolor and ch in recolor:
                rgba = recolor[ch] + (255,)
            line.append(rgba)
        out.append(line)
    return out


def write_png(path, rows):
    h = len(rows)
    w = len(rows[0])
    raw = b"".join(b"\x00" + bytes(c for px in row for c in px) for row in rows)

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    png = b"\x89PNG\r\n\x1a\n"
    png += chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(raw, 9))
    png += chunk(b"IEND", b"")
    with open(path, "wb") as f:
        f.write(png)


def strip(frames):
    """여러 16x16 그림을 가로로 이어 붙인다."""
    return [sum((f[y] for f in frames), []) for y in range(16)]


def main():
    os.makedirs(OUT, exist_ok=True)
    # 지형: sand0 sand1 flat0 flat1 rock0 rock1 water0 water1 flood0 flood1 nest
    tiles = [SAND_0, SAND_1, FLAT_0, FLAT_1, ROCK_0, ROCK_1, WATER_0, WATER_1, FLOOD_0, FLOOD_1, NEST]
    write_png(os.path.join(OUT, "tiles.png"), strip([pixels(t) for t in tiles]))
    for name, (dark, shell, light) in CRAB_COLORS.items():
        recolor = {"D": dark, "S": shell, "L": light}
        write_png(os.path.join(OUT, f"crab_{name}.png"), strip([pixels(CRAB_0, recolor), pixels(CRAB_1, recolor)]))
    write_png(os.path.join(OUT, "gull.png"), strip([pixels(GULL_0), pixels(GULL_1)]))
    write_png(os.path.join(OUT, "food.png"), strip([pixels(FOOD_0), pixels(FOOD_1)]))
    print("그림을 만들었어요:", os.path.normpath(OUT))


if __name__ == "__main__":
    main()
