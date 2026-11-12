#!/usr/bin/env python3
"""Sunnyside World (danieldiggle, itch.io) 에서 이 팩의 그림을 만든다.

art/sunnyside/ 의 원본 PNG 를 잘라 붙여 view/assets/ 에 쓴다. (Pillow 필요: pip install pillow)
    python3 art/sunnyside.py

만드는 것
  tiles.png        16x16 타일 13개 (view.gd 의 enum Tile 순서)
  plants.png       16x16 작물 6종 × 자라는 6단계 (줄 = 작물, 칸 = 0 씨앗 … 5 다 자람)
  deco_*.png       동물·풍차·나무 같은 장식 (원본 그대로 가로 줄)
  exp_*.png        말풍선 아이콘 (놀람·하트·물음표·일하는 중·땀)
  hud_box.png      HUD 상자용 9칸 그림 (9x9, 모서리 3px)
  farmer_N.png     24x24 농부: 1줄 걷기 8프레임, 2줄 들고 걷기 8프레임, 3줄 가만히 9프레임
  goblin.png       24x24 고블린: 1줄 걷기 8프레임, 2줄 가만히 9프레임
"""
from pathlib import Path
from PIL import Image

HERE = Path(__file__).resolve().parent
SRC = HERE / "sunnyside"
OUT = HERE.parent / "view" / "assets"

HAIRS = ["shorthair", "longhair", "spikeyhair", "curlyhair"]
CELL = 24  # 캐릭터 한 프레임 (원본 96x64 의 가운데를 잘라 쓴다)
CROP_BOX = (36, 17, 60, 41)


def load(path):
    return Image.open(path).convert("RGBA")


def tile(sheet, col, row, dx=0, dy=0):
    """타일셋의 (열, 줄) 한 칸. dx, dy 로 픽셀 단위 어긋남."""
    x, y = col * 16 + dx, row * 16 + dy
    return sheet.crop((x, y, x + 16, y + 16))


def over(base, top):
    out = base.copy()
    out.alpha_composite(top)
    return out


def build_tiles():
    sheet = load(SRC / "tileset_16px.png")
    grass = [tile(sheet, 2, 2), tile(sheet, 3, 2)]
    soil = [tile(sheet, 9, 7), tile(sheet, 10, 7)]
    paddy = [tile(sheet, 12, 7), tile(sheet, 11, 7)]
    bush = over(grass[0], tile(sheet, 51, 4))
    berry = over(grass[1], sheet.crop((790, 54, 806, 70)))
    water0 = sheet.crop((176, 288, 192, 304))
    water1 = sheet.crop((176, 288, 192, 304)).transpose(Image.FLIP_LEFT_RIGHT)
    flood = [Image.new("RGBA", (16, 16), (40, 140, 230, 150)), Image.new("RGBA", (16, 16), (70, 170, 240, 150))]
    crate = over(soil[0], sheet.crop((567, 144, 583, 160)))
    # 이랑: 흙을 어둡게 하고 줄을 번갈아 밝게·어둡게 그어 갈아 놓은 밭처럼
    plot = Image.new("RGBA", (16, 16), (150, 98, 62, 255))
    for y in range(16):
        shade = (176, 118, 76, 255) if y % 5 in (1, 2) else (126, 80, 50, 255) if y % 5 == 4 else (150, 98, 62, 255)
        for x in range(16):
            plot.putpixel((x, y), shade)
    tiles = grass + soil + paddy + [bush, berry, water0, water1] + flood + [crate, plot]
    atlas = Image.new("RGBA", (16 * len(tiles), 16))
    for i, t in enumerate(tiles):
        atlas.paste(t, (i * 16, 0))
    atlas.save(OUT / "tiles.png")


KINDS = ["pumpkin", "carrot", "cabbage", "cauliflower", "radish", "sunflower"]


def build_plants():
    """작물 한 종의 자라는 6단계를 16x16 칸 아래쪽 가운데에 맞춰 한 줄로."""
    sheet = Image.new("RGBA", (16 * 6, 16 * len(KINDS)))
    for row, kind in enumerate(KINDS):
        for stage in range(6):
            im = load(SRC / "Crops" / f"{kind}_{stage:02d}.png")
            sheet.alpha_composite(im, (stage * 16 + (16 - im.width) // 2, row * 16 + (16 - im.height) - 1))
    sheet.save(OUT / "plants.png")


DECOR = {
    "Animals/spr_deco_cow_strip4.png": "deco_cow.png",
    "Animals/spr_deco_sheep_01_strip4.png": "deco_sheep.png",
    "Animals/spr_deco_pig_01_strip4.png": "deco_pig.png",
    "Animals/spr_deco_chicken_01_strip4.png": "deco_chicken.png",
    "Animals/spr_deco_duck_01_strip4.png": "deco_duck.png",
    "Animals/spr_deco_bird_01_strip4.png": "deco_bird.png",
    "Other/spr_deco_windmill_withshadow_strip9.png": "deco_windmill.png",
    "Plants/spr_deco_tree_01_strip4.png": "deco_tree1.png",
    "Plants/spr_deco_tree_02_strip4.png": "deco_tree2.png",
    "Plants/spr_deco_mushroom_red_01_strip4.png": "deco_mushroom.png",
    "VFX/spr_deco_glint_01_strip6.png": "deco_glint.png",
    "UI/expression_alerted.png": "exp_alert.png",
    "UI/expression_love.png": "exp_love.png",
    "UI/expression_confused.png": "exp_confused.png",
    "UI/expression_working.png": "exp_working.png",
    "UI/expression_stress.png": "exp_stress.png",
    "UI/water.png": "icon_water.png",
    "UI/plant.png": "icon_plant.png",
    "UI/stopwatch.png": "icon_clock.png",
}


def build_decor():
    for src, name in DECOR.items():
        load(SRC / src).save(OUT / name)
    names = ["tl", "tc", "tr", "lc", "c", "rc", "bl", "bc", "br"]
    box = Image.new("RGBA", (9, 9))
    for i, n in enumerate(names):
        box.paste(load(SRC / "UI" / f"dt_box_9slice_{n}.png"), ((i % 3) * 3, (i // 3) * 3))
    box.save(OUT / "hud_box.png")


def strip_frames(path, n):
    im = load(path)
    return [im.crop((i * 96, 0, (i + 1) * 96, 64)).crop(CROP_BOX) for i in range(n)]


def layered(hair, anim, n):
    """맨몸 + 머리 + 도구를 겹친 프레임들."""
    frames = strip_frames(SRC / "Human" / f"base_{anim}_strip{n}.png", n)
    for layer in (hair, "tools"):
        top = strip_frames(SRC / "Human" / f"{layer}_{anim}_strip{n}.png", n)
        frames = [over(a, b) for a, b in zip(frames, top)]
    return frames


def row(sheet, frames, y):
    for i, f in enumerate(frames):
        sheet.paste(f, (i * CELL, y * CELL))


def build_farmers():
    for i, hair in enumerate(HAIRS):
        sheet = Image.new("RGBA", (9 * CELL, 3 * CELL))
        row(sheet, layered(hair, "walk", 8), 0)
        row(sheet, layered(hair, "carry", 8), 1)
        row(sheet, layered(hair, "idle", 9), 2)
        sheet.save(OUT / f"farmer_{i}.png")


def build_goblin():
    sheet = Image.new("RGBA", (9 * CELL, 2 * CELL))
    row(sheet, strip_frames(SRC / "Goblin" / "spr_walk_strip8.png", 8), 0)
    row(sheet, strip_frames(SRC / "Goblin" / "spr_idle_strip9.png", 9), 1)
    sheet.save(OUT / "goblin.png")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    build_tiles()
    build_plants()
    build_decor()
    build_farmers()
    build_goblin()
    print("done:", OUT)
