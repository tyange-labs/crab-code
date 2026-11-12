#!/usr/bin/env python3
"""게임 본체(UI)의 9칸 상자 그림을 코드로 그린다. Pillow 필요: pip install pillow
    python3 art/ui.py
game/assets/ 에 쓴다. 모서리 3px 를 늘이지 않는 9x9 그림이다 (StyleBoxTexture 의 모서리 3).
"""
from pathlib import Path
from PIL import Image

OUT = Path(__file__).resolve().parent.parent / "assets"


def C(h, a=255):
    h = h.lstrip("#")
    return (int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16), a)


def box(fill, edge, hi=None):
    im = Image.new("RGBA", (9, 9), fill)
    for i in range(9):
        for (x, y) in ((i, 0), (i, 8), (0, i), (8, i)):
            im.putpixel((x, y), edge)
    for (x, y) in ((0, 0), (8, 0), (0, 8), (8, 8)):  # 모서리를 깎는다
        im.putpixel((x, y), (0, 0, 0, 0))
    if hi:  # 윗변 안쪽 하이라이트
        for x in range(2, 7):
            im.putpixel((x, 1), hi)
    return im


if __name__ == "__main__":
    OUT.mkdir(exist_ok=True)
    box(C("#0e1424"), C("#2b3a5e"), C("#16203a")).save(OUT / "ui_panel.png")
    box(C("#1a2442"), C("#3b6a86"), C("#243258")).save(OUT / "ui_btn.png")
    box(C("#0a0f1c"), C("#35e0ff"), None).save(OUT / "ui_btn_down.png")
    print("done:", OUT)
