#!/usr/bin/env python3
"""정거장 복구 팩의 그림을 코드로 그린다 (외부 에셋 없음). Pillow 필요: pip install pillow
    python3 art/draw.py
view/assets/ 에 쓴다. 모든 그림은 16x16 칸 격자에 맞는 도트이고, 색은 아래 팔레트만 쓴다.
"""
import math
from pathlib import Path
from PIL import Image

OUT = Path(__file__).resolve().parent.parent / "view" / "assets"

# ── 팔레트: 어두운 SF ──
def C(h, a=255):
    h = h.lstrip("#")
    return (int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16), a)

SPACE = C("#070a14")
SPACE2 = C("#0b1020")
DECK = C("#1b2336")
DECK_HI = C("#27314c")
DECK_LO = C("#121826")
WALL = C("#3b4766")
WALL_HI = C("#5a688c")
WALL_LO = C("#232b42")
STEEL = C("#a9b8d4")
STEEL_LO = C("#5f6e8e")
INK = C("#0a0d18")
CYAN = C("#35e0ff")
AMBER = C("#ffb347")
RED = C("#ff4d5e")
GREEN = C("#52f59a")
MAGENTA = C("#d56bff")
WHITE = C("#e8f2ff")
CORE_COLORS = [CYAN, AMBER, GREEN, MAGENTA, RED, WHITE]
BOT_COLORS = [AMBER, CYAN, GREEN, MAGENTA]


def blank(w=16, h=16, fill=(0, 0, 0, 0)):
    return Image.new("RGBA", (w, h), fill)


def px(im, x, y, c):
    if 0 <= x < im.width and 0 <= y < im.height:
        im.putpixel((x, y), c)


def rect(im, x0, y0, x1, y1, c):
    for y in range(y0, y1 + 1):
        for x in range(x0, x1 + 1):
            px(im, x, y, c)


def blend(c, over, k):
    return tuple(int(c[i] * (1 - k) + over[i] * k) for i in range(3)) + (255,)


# ── 타일 ──
def space_tile(seed):
    im = blank(16, 16, SPACE if seed == 0 else SPACE2)
    stars = [(3, 4), (11, 2), (8, 11), (13, 13)] if seed == 0 else [(5, 7), (12, 5), (2, 12)]
    for i, (x, y) in enumerate(stars):
        px(im, x, y, C("#3a4a78") if i % 2 else C("#7f93c8"))
    return im


def deck_tile(variant):
    im = blank(16, 16, DECK)
    rect(im, 0, 0, 15, 0, DECK_HI)
    rect(im, 0, 0, 0, 15, DECK_HI)
    rect(im, 15, 0, 15, 15, DECK_LO)
    rect(im, 0, 15, 15, 15, DECK_LO)
    if variant == 0:
        for x in (3, 12):
            for y in (3, 12):
                px(im, x, y, DECK_LO)
    else:
        rect(im, 5, 7, 10, 7, DECK_LO)
        rect(im, 5, 9, 10, 9, DECK_LO)
    return im


def duct_tile(variant):
    im = blank(16, 16, C("#202636"))
    # 가장자리 주의 줄무늬
    for i in range(16):
        stripe = ((i + (0 if variant == 0 else 4)) // 4) % 2 == 0
        c = AMBER if stripe else INK
        px(im, i, 0, c)
        px(im, i, 15, c)
        px(im, 0, i, c)
        px(im, 15, i, c)
    for y in range(3, 13, 3):  # 환기 격자
        rect(im, 3, y, 12, y, C("#0d1220"))
    return im


def wall_tile(variant):
    im = blank(16, 16, WALL)
    rect(im, 0, 0, 15, 1, WALL_HI)
    rect(im, 0, 14, 15, 15, WALL_LO)
    rect(im, 0, 0, 0, 15, WALL_HI)
    rect(im, 15, 0, 15, 15, WALL_LO)
    for x, y in ((2, 3), (13, 3), (2, 12), (13, 12)):
        px(im, x, y, WALL_LO)
        px(im, x - 1 if x > 8 else x + 1, y, WALL_HI)
    rect(im, 7, 3, 8, 12, WALL_LO)
    if variant == 1:
        px(im, 4, 7, GREEN)
        px(im, 4, 8, C("#1f7a4d"))
    return im


def void_tile(variant):
    im = blank(16, 16, C("#03050c"))
    for i in range(0, 16, 4):  # 어두운 격자
        rect(im, i, 0, i, 15, C("#0a1020"))
        rect(im, 0, i, 15, i, C("#0a1020"))
    px(im, 6 + variant * 3, 5, C("#8aa0e8"))
    px(im, 11 - variant * 4, 11, C("#4c62b0"))
    return im


def vent_overlay(variant):
    """진공이 될 때 통로 위에 덮는 붉은 서리."""
    im = blank(16, 16, (255, 60, 90, 70))
    for y in range(16):
        if (y + variant * 2) % 4 == 0:
            rect(im, 0, y, 15, y, (255, 150, 170, 120))
    return im


def reactor_tile():
    im = blank(16, 16, DECK)
    rect(im, 0, 0, 15, 0, DECK_HI)
    rect(im, 0, 0, 0, 15, DECK_HI)
    cx = cy = 7.5
    for y in range(16):
        for x in range(16):
            d = math.hypot(x - cx, y - cy)
            if d <= 3.2:
                px(im, x, y, WHITE if d <= 1.6 else CYAN)
            elif d <= 4.6:
                px(im, x, y, C("#14788f"))
            elif d <= 6.2:
                px(im, x, y, C("#10182a") if d <= 5.4 else STEEL_LO)
    return im


def slot_tile():
    im = blank(16, 16, DECK)
    rect(im, 0, 0, 15, 0, DECK_HI)
    rect(im, 0, 0, 0, 15, DECK_HI)
    rect(im, 3, 3, 12, 12, C("#0b101e"))
    rect(im, 4, 4, 11, 11, C("#0f1628"))
    for (x, y, dx, dy) in ((2, 2, 1, 1), (13, 2, -1, 1), (2, 13, 1, -1), (13, 13, -1, -1)):
        px(im, x, y, CYAN)
        px(im, x + dx, y, CYAN)
        px(im, x, y + dy, CYAN)
    return im


BLUE = C("#4aa8ff")


def door_tile(col):
    im = blank(16, 16, WALL_LO)
    rect(im, 0, 0, 15, 0, WALL_HI)
    rect(im, 0, 0, 0, 15, WALL_HI)
    rect(im, 15, 0, 15, 15, INK)
    rect(im, 0, 15, 15, 15, INK)
    rect(im, 3, 2, 12, 13, C("#161c2e"))   # 문짝
    rect(im, 7, 2, 8, 13, C("#0c1020"))   # 가운데 틈
    rect(im, 5, 6, 10, 9, INK)
    rect(im, 6, 7, 9, 8, col)             # 잠금 표시
    rect(im, 6, 6, 9, 6, blend(col, INK, 0.5))
    for y in (3, 12):
        px(im, 4, y, col)
        px(im, 11, y, col)
    return im


def card_tile(col):
    im = deck_tile(0)
    rect(im, 4, 5, 11, 10, INK)
    rect(im, 5, 6, 10, 9, col)
    rect(im, 5, 6, 10, 6, blend(col, WHITE, 0.55))
    rect(im, 5, 9, 7, 9, blend(col, INK, 0.45))
    for x in (4, 11):
        px(im, x, 4, (col[0], col[1], col[2], 90))
    return im


def pad_tile():
    im = deck_tile(0)
    for y in range(16):
        for x in range(16):
            d = math.hypot(x - 7.5, y - 7.5)
            if 4.4 <= d <= 5.6:
                px(im, x, y, C("#2a8aa0"))
    # 번개 표시
    for (x, y) in ((8, 4), (7, 5), (7, 6), (6, 7), (8, 7), (7, 8), (7, 9), (8, 10), (9, 11)):
        px(im, x, y, AMBER)
    px(im, 8, 6, WHITE)
    px(im, 8, 8, WHITE)
    return im


def build_tiles():
    tiles = [
        space_tile(0), space_tile(1),
        deck_tile(0), deck_tile(1),
        duct_tile(0), duct_tile(1),
        wall_tile(0), wall_tile(1),
        void_tile(0), void_tile(1),
        vent_overlay(0), vent_overlay(1),
        reactor_tile(), slot_tile(),
        door_tile(RED), door_tile(BLUE), card_tile(RED), card_tile(BLUE),
        pad_tile(),
    ]
    atlas = blank(16 * len(tiles), 16)
    for i, t in enumerate(tiles):
        atlas.paste(t, (i * 16, 0))
    atlas.save(OUT / "tiles.png")


# ── 코어와 모듈 ──
def glow_orb(im, cx, cy, r, col, ready=True):
    for y in range(im.height):
        for x in range(im.width):
            d = math.hypot(x - cx, y - cy)
            if d <= r:
                k = 0.0 if d < r * 0.45 else 0.35 if d < r * 0.8 else 0.6
                px(im, x, y, blend(col, WHITE, 0.75 - k * 0.6 if d < r * 0.55 else 0.0) if d < r * 0.55 else blend(col, INK, k * 0.55))
            elif ready and d <= r + 1.6:
                px(im, x, y, (col[0], col[1], col[2], 70))
    px(im, int(cx - r * 0.4), int(cy - r * 0.45), WHITE)


def core_frame(stage, col):
    im = blank(16, 16)
    if stage == 5:  # 다 된 코어: 빛나는 구슬
        glow_orb(im, 7.5, 8, 5.2, col)
        return im
    # 모듈 칩: 단계마다 LED 가 하나씩 켜진다
    rect(im, 4, 5, 11, 12, C("#2b3552"))
    rect(im, 4, 5, 11, 5, C("#46557d"))
    rect(im, 5, 6, 10, 11, C("#18203a"))
    for y in (6, 8, 10):
        px(im, 3, y, STEEL_LO)
        px(im, 12, y, STEEL_LO)
    lit = stage  # 0..4
    leds = [(6, 7), (9, 7), (6, 10), (9, 10)]
    for i, (x, y) in enumerate(leds):
        on = i < lit
        px(im, x, y, col if on else C("#2a3354"))
        if on:
            px(im, x + 1, y, blend(col, INK, 0.45))
    return im


def build_cores():
    sheet = blank(16 * 6, 16 * len(CORE_COLORS))
    for row, col in enumerate(CORE_COLORS):
        for stage in range(6):
            sheet.paste(core_frame(stage, col), (stage * 16, row * 16))
    sheet.save(OUT / "plants.png")


# ── 로봇 ──
A = 24  # 한 프레임


def draw_bot(frame, accent, state, i):
    im = blank(A, A)
    bob = (1 if i % 2 == 1 else 0) if state != "idle" else (1 if (i // 3) % 2 else 0)
    ox, oy = 6, 5 + bob
    # 다리/바퀴
    shift = (i % 4) if state != "idle" else 0
    for k, x in enumerate((ox + 2, ox + 7)):
        h = 2 if (shift + k) % 2 == 0 else 1
        rect(im, x, oy + 10, x + 2, oy + 10 + h, C("#2a3350"))
        rect(im, x, oy + 10, x + 2, oy + 10, STEEL_LO)
    # 몸통
    rect(im, ox, oy + 3, ox + 11, oy + 10, STEEL)
    rect(im, ox, oy + 8, ox + 11, oy + 10, STEEL_LO)
    rect(im, ox, oy + 3, ox + 11, oy + 3, WHITE)
    rect(im, ox + 1, oy + 9, ox + 10, oy + 9, accent)  # 가슴 줄무늬
    # 머리(바이저)
    rect(im, ox + 1, oy - 2, ox + 10, oy + 3, STEEL)
    rect(im, ox + 1, oy - 2, ox + 10, oy - 2, WHITE)
    rect(im, ox + 2, oy - 1, ox + 9, oy + 2, INK)
    blink = state == "idle" and i in (6, 7)
    eye = C("#2a3354") if blink else accent
    rect(im, ox + 6, oy, ox + 8, oy + 1, eye)  # 눈이 오른쪽(진행 방향)을 본다
    if not blink:
        px(im, ox + 8, oy, WHITE)
    # 안테나
    rect(im, ox + 5, oy - 5, ox + 5, oy - 3, STEEL_LO)
    on = (i % 3) != 0
    px(im, ox + 5, oy - 6, accent if on else blend(accent, INK, 0.6))
    if state == "carry":  # 팔을 들어 코어를 든다
        rect(im, ox - 1, oy + 3, ox - 1, oy + 6, STEEL_LO)
        rect(im, ox + 12, oy + 3, ox + 12, oy + 6, STEEL_LO)
    return im


def build_bots():
    for n, accent in enumerate(BOT_COLORS):
        sheet = blank(9 * A, 3 * A)
        for i in range(8):
            sheet.paste(draw_bot(None, accent, "walk", i), (i * A, 0))
            sheet.paste(draw_bot(None, accent, "carry", i), (i * A, A))
        for i in range(9):
            sheet.paste(draw_bot(None, accent, "idle", i), (i * A, 2 * A))
        sheet.save(OUT / f"bot_{n}.png")


# ── 경비 드론 ──
def draw_sentry(i, alert):
    im = blank(A, A)
    bob = 1 if (i // 2) % 2 else 0
    cx, cy = 12, 12 + bob
    # 프로펠러
    span = (7, 5, 3, 5)[i % 4]
    rect(im, cx - span, cy - 7, cx + span, cy - 7, STEEL_LO)
    rect(im, cx, cy - 6, cx, cy - 5, STEEL_LO)
    # 몸체
    for y in range(A):
        for x in range(A):
            d = math.hypot(x - cx, y - cy)
            if d <= 5.5:
                px(im, x, y, C("#3a1624") if d > 3.4 else C("#5a1c2a"))
            if 5.5 < d <= 6.5 and abs(x - cx) < 5:
                px(im, x, y, C("#7a2a3c") if y < cy else C("#2a0f19"))
    rect(im, cx - 2, cy - 2, cx + 2, cy + 2, RED if (i % 2 == 0 or alert) else C("#b02a3a"))
    px(im, cx, cy, WHITE)
    # 아래 분사
    if i % 2 == 0:
        px(im, cx - 2, cy + 7, (255, 120, 90, 160))
        px(im, cx + 2, cy + 7, (255, 120, 90, 160))
    return im


def build_sentry():
    sheet = blank(9 * A, 2 * A)
    for i in range(8):
        sheet.paste(draw_sentry(i, False), (i * A, 0))
    for i in range(9):
        sheet.paste(draw_sentry(i, True), (i * A, A))
    sheet.save(OUT / "sentry.png")


# ── 장식, 말풍선, 반짝임 ──
def deco_debris(i):
    im = blank(16, 16)
    cx, cy = 8, 8
    a = i * math.pi / 8
    pts = [(cx + 5 * math.cos(a + k * 1.9), cy + 4 * math.sin(a + k * 1.9)) for k in range(3)]
    for y in range(16):
        for x in range(16):
            # 삼각형 안쪽
            def side(p, q):
                return (x - q[0]) * (p[1] - q[1]) - (p[0] - q[0]) * (y - q[1])
            d1, d2, d3 = side(pts[0], pts[1]), side(pts[1], pts[2]), side(pts[2], pts[0])
            if (d1 >= 0 and d2 >= 0 and d3 >= 0) or (d1 <= 0 and d2 <= 0 and d3 <= 0):
                px(im, x, y, WALL if y < cy else WALL_LO)
    return im


def deco_beacon(i):
    im = blank(16, 16)
    rect(im, 7, 5, 8, 14, STEEL_LO)
    rect(im, 5, 14, 10, 15, WALL_LO)
    on = i % 4 < 2
    for dx, dy in ((0, 0), (1, 0), (0, 1), (1, 1)):
        px(im, 7 + dx, 3 + dy, RED if on else C("#5a1c28"))
    if on:
        px(im, 6, 4, (255, 77, 94, 110))
        px(im, 9, 4, (255, 77, 94, 110))
        px(im, 7, 2, (255, 77, 94, 110))
    return im


def deco_dish(i):
    im = blank(16, 16)
    rect(im, 7, 9, 8, 14, STEEL_LO)
    rect(im, 4, 14, 11, 15, WALL_LO)
    tilt = (i % 4) - 1
    for k in range(-5, 6):
        y = 8 - int(abs(k) * 0.5) + tilt // 2
        px(im, 8 + k, y, STEEL)
        px(im, 8 + k, y + 1, STEEL_LO)
    px(im, 8, 4 + (tilt // 2), CYAN)
    return im


def strip(frames):
    s = blank(16 * len(frames), 16)
    for i, f in enumerate(frames):
        s.paste(f, (i * 16, 0))
    return s


def glyph(rows, col):
    h = len(rows)
    w = max(len(r) for r in rows)
    im = blank(w, h)
    for y, r in enumerate(rows):
        for x, ch in enumerate(r):
            if ch == "#":
                im.putpixel((x, y), col)
            elif ch == "+":
                im.putpixel((x, y), WHITE)
    return im


def build_decor():
    strip([deco_debris(i) for i in range(4)]).save(OUT / "deco_debris.png")
    strip([deco_beacon(i) for i in range(4)]).save(OUT / "deco_beacon.png")
    strip([deco_dish(i) for i in range(4)]).save(OUT / "deco_dish.png")
    glyph([" ### ", "## ##", "#   #", "  ## ", "  #  ", "     ", "  #  ", "  #  "][:8], AMBER).save(OUT / "exp_confused.png")
    glyph(["#####", "#####", "#####", " ### ", " ### ", "  #  ", "     ", "  #  ", "#####"][:9], RED).save(OUT / "exp_alert.png")
    glyph(["     ", "    #", "   ##", "#  # ", "## # ", " ##  ", "  #  "], GREEN).save(OUT / "exp_love.png")
    glyph(["  #  ", " ### ", "## ##", "#####", "## ##", " ### ", "  #  "], CYAN).save(OUT / "exp_working.png")
    glyph(["# # #", " # # ", "# # #", " # # ", "# # #"], AMBER).save(OUT / "exp_stress.png")
    # 반짝임 6프레임 (7x7)
    g = blank(7 * 6, 7)
    for i in range(6):
        r = (0, 1, 2, 3, 2, 1)[i]
        for k in range(-r, r + 1):
            px(g, i * 7 + 3 + k, 3, WHITE if abs(k) < 2 else (200, 230, 255, 160))
            px(g, i * 7 + 3, 3 + k, WHITE if abs(k) < 2 else (200, 230, 255, 160))
    g.save(OUT / "deco_glint.png")
    # HUD 상자 (9칸): 어두운 속 + 청록 테두리
    box = blank(9, 9, C("#0b1220"))
    edge = C("#2a8aa0")
    for i in range(9):
        for (x, y) in ((i, 0), (i, 8), (0, i), (8, i)):
            box.putpixel((x, y), edge)
    for (x, y) in ((0, 0), (8, 0), (0, 8), (8, 8)):
        box.putpixel((x, y), (0, 0, 0, 0))
    for (x, y) in ((1, 1), (7, 1), (1, 7), (7, 7)):
        box.putpixel((x, y), CYAN)
    box.save(OUT / "hud_box.png")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    build_tiles()
    build_cores()
    build_bots()
    build_sentry()
    build_decor()
    print("done:", OUT)
