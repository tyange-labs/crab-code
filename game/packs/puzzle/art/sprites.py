#!/usr/bin/env python3
"""퍼즐 게임팩의 픽셀 아트 (16x16).

그림은 아래의 글자 격자로 그린다. 글자 하나가 픽셀 하나이고, 무슨 색인지는 PALETTE 에 있다.
고친 뒤 이 파일을 실행하면 ../view/assets/ 의 PNG 가 다시 만들어진다.

    python3 game/packs/puzzle/art/sprites.py

PIL 없이 표준 라이브러리만 쓴다.
"""

import os
import struct
import zlib

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "view", "assets")

PALETTE = {
    ".": (0, 0, 0, 0),
    # 바닥 (동굴 웅덩이의 돌바닥)
    "a": (0x36, 0x3b, 0x58, 255),
    "b": (0x32, 0x36, 0x51, 255),
    "c": (0x40, 0x46, 0x66, 255),
    "d": (0x2a, 0x2e, 0x45, 255),
    # 빨강 보석
    "r": (0x9e, 0x24, 0x3d, 255),
    "R": (0xff, 0x5c, 0x7a, 255),
    "Q": (0xff, 0xb3, 0xc2, 255),
    # 파랑 보석
    "u": (0x1f, 0x5f, 0x9e, 255),
    "U": (0x4f, 0xb3, 0xff, 255),
    "V": (0xb8, 0xe2, 0xff, 255),
    # 흐릿한 보석
    "g": (0x45, 0x40, 0x4c, 255),
    "G": (0x7a, 0x6e, 0x7a, 255),
    "H": (0x9a, 0x92, 0x9c, 255),
    # 받침대
    "p": (0x1c, 0x1e, 0x2e, 255),
    "P": (0xb9, 0xbe, 0xd4, 255),
    "e": (0xc4, 0x46, 0x5e, 255),
    "f": (0x3f, 0x8f, 0xd6, 255),
    # 문
    "w": (0x5c, 0x3a, 0x1c, 255),
    "W": (0x8a, 0x5a, 0x2b, 255),
    "X": (0xa9, 0x71, 0x3a, 255),
    "y": (0xf2, 0xc9, 0x4c, 255),
    "k": (0x3a, 0x2c, 0x00, 255),
    # 깃발
    "l": (0xe4, 0xe6, 0xf0, 255),
    "n": (0x2f, 0x9e, 0x5f, 255),
    "N": (0x5f, 0xd3, 0x8d, 255),
    # 숫자 타일
    "t": (0x1f, 0x33, 0x4d, 255),
    "T": (0x27, 0x40, 0x5e, 255),
    # 꽃게
    "D": (0x8f, 0x2e, 0x10, 255),
    "S": (0xe5, 0x54, 0x1f, 255),
    "L": (0xff, 0x8a, 0x4c, 255),
    "O": (0xf4, 0xf1, 0xe8, 255),
    "K": (0x1a, 0x1a, 0x22, 255),
}

FLOOR_0 = """
dddddddddddddddd
daaaaaaaaaaaaaab
daaaaaaaaaaaaaab
daaacaaaaaaaaaab
daaaaaaaaaaaaaab
daaaaaaaaacaaaab
daaaaaaaaaaaaaab
daaaaaaaaaaaaaab
daaaaacaaaaaaaab
daaaaaaaaaaaaaab
daaaaaaaaaaaacab
daaaaaaaaaaaaaab
daacaaaaaaaaaaab
daaaaaaaaaacaaab
daaaaaaaaaaaaaab
dbbbbbbbbbbbbbbb
"""

FLOOR_1 = """
dddddddddddddddd
dbbbbbbbbbbbbbba
dbbbbbbbbbcbbbba
dbbbbbbbbbbbbbba
dbbcbbbbbbbbbbba
dbbbbbbbbbbbbbba
dbbbbbbbbbbbbcba
dbbbbbbbbbbbbbba
dbbbbbbbbbbbbbba
dbbbbbbcbbbbbbba
dbbbbbbbbbbbbbba
dbbbbbbbbbbbbbba
dbbbbbbbbbbcbbba
dbbbcbbbbbbbbbba
dbbbbbbbbbbbbbba
daaaaaaaaaaaaaaa
"""

GEM_RED = """
................
................
................
.......r........
......rQr.......
.....rQRRr......
....rQRRRRr.....
...rRQRRRRRr....
...rRRRRRRRr....
....rRRRRRr.....
.....rRRRr......
......rRr.......
.......r........
................
................
................
"""

GEM_BLUE = GEM_RED.replace("r", "u").replace("R", "U").replace("Q", "V")
GEM_DULL = GEM_RED.replace("r", "g").replace("R", "G").replace("Q", "H")

PEDESTAL = """
................
................
...pppppppppp...
..pPPPPPPPPPPp..
..pPppppppppPp..
..pPp......pPp..
..pPp......pPp..
..pPp......pPp..
..pPp......pPp..
..pPp......pPp..
..pPp......pPp..
..pPppppppppPp..
..pPPPPPPPPPPp..
...pppppppppp...
................
................
"""

PEDESTAL_RED = PEDESTAL.replace("P", "e")
PEDESTAL_BLUE = PEDESTAL.replace("P", "f")

DOOR_CLOSED = """
.wwwwwwwwwwwwww.
.wWWWWWWWWWWWWw.
.wWXXXXXXXXXXWw.
.wWXXXXXXXXXXWw.
.wWXXXXXXXXXXWw.
.wWXXXXyyXXXXWw.
.wWXXXyyyyXXXWw.
.wWXXXyyyyXXXWw.
.wWXXXXyyXXXXWw.
.wWXXXXkyXXXXWw.
.wWXXXXkyXXXXWw.
.wWXXXXXXXXXXWw.
.wWXXXXXXXXXXWw.
.wWWWWWWWWWWWWw.
.wwwwwwwwwwwwww.
................
"""

DOOR_OPEN = """
.wwwwwwwwwwwwww.
.wWWWWWWWWWWWWw.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wW..........Ww.
.wWWWWWWWWWWWWw.
.wwwwwwwwwwwwww.
................
"""

FLAG_0 = """
................
................
....lNNNN.......
....lNNNNNNN....
....lNNNNNNNNN..
....lNNNNNNNn...
....lNNNNnn.....
....lnn.........
....l...........
....l...........
....l...........
....l...........
....l...........
...lll..........
................
................
"""

FLAG_1 = """
................
................
....lNNN........
....lNNNNNNN....
....lNNNNNNNNNN.
....lNNNNNNNNn..
....lNNNNNnn....
....lNnn........
....l...........
....l...........
....l...........
....l...........
....l...........
...lll..........
................
................
"""

SIGN_LEFT = """
................
................
................
................
....yyyyyyyyyy..
...yykkyyyyyyy..
..yykkkkkkkkky..
..ykkkkkkkkkky..
..yykkkkkkkkky..
...yykkyyyyyyy..
....yyyyyyyyyy..
.......kk.......
.......kk.......
................
................
................
"""

SIGN_RIGHT = "\n".join(row[::-1] for row in SIGN_LEFT.split("\n"))

NUMBER = """
................
................
................
...TTTTTTTTTT...
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
..TttttttttttT..
...TTTTTTTTTT...
................
................
................
"""

CRAB_0 = """
................
.LS..........SL.
.SSD........DSS.
.DSS........SSD.
..DSD.O..O.DSD..
...D..K..K..D...
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
..DSD.O..O.DSD..
...D..K..K..D...
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
    # 순서가 view.gd 의 Sprite enum 과 같아야 한다.
    sheet = [FLOOR_0, FLOOR_1, GEM_RED, GEM_BLUE, GEM_DULL, PEDESTAL, PEDESTAL_RED, PEDESTAL_BLUE,
             DOOR_CLOSED, DOOR_OPEN, FLAG_0, FLAG_1, SIGN_LEFT, SIGN_RIGHT, NUMBER, CRAB_0, CRAB_1]
    write_png(os.path.join(OUT, "sheet.png"), strip([pixels(s) for s in sheet]))
    print("그림을 만들었어요:", os.path.normpath(OUT))


if __name__ == "__main__":
    main()
