// 글꼴, 색, 공용 그리기 도구.

use macroquad::prelude::*;

pub const fn hex(v: u32) -> Color {
    Color {
        r: ((v >> 16) & 0xff) as f32 / 255.0,
        g: ((v >> 8) & 0xff) as f32 / 255.0,
        b: (v & 0xff) as f32 / 255.0,
        a: 1.0,
    }
}

pub const BG: Color = hex(0x151722);
pub const PANEL: Color = hex(0x202334);
pub const PANEL_HI: Color = hex(0x2c3049);
pub const FLOOR: Color = hex(0x363b58);
pub const TEXT: Color = hex(0xe4e6f0);
pub const DIM: Color = hex(0x8a8fa8);
pub const ACCENT: Color = hex(0xf46a25);
pub const OK_GREEN: Color = hex(0x5fd38d);
pub const ERR_RED: Color = hex(0xff6b7a);
pub const WARN: Color = hex(0xf2c94c);
pub const SKY: Color = hex(0x5cb8ff);

pub fn mix(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

pub fn alpha(c: Color, a: f32) -> Color {
    Color { a, ..c }
}

/// 화면에서 차지하는 칸 수. 한글 등 비 ASCII 문자는 두 칸.
pub fn char_cols(c: char) -> usize {
    if c.is_ascii() {
        1
    } else {
        2
    }
}

pub fn str_cols(s: &str) -> usize {
    s.chars().map(char_cols).sum()
}

pub fn rrect(x: f32, y: f32, w: f32, h: f32, r: f32, c: Color) {
    let r = r.min(w / 2.0).min(h / 2.0);
    draw_rectangle(x + r, y, w - 2.0 * r, h, c);
    draw_rectangle(x, y + r, w, h - 2.0 * r, c);
    for (cx, cy) in [
        (x + r, y + r),
        (x + w - r, y + r),
        (x + r, y + h - r),
        (x + w - r, y + h - r),
    ] {
        draw_circle(cx, cy, r, c);
    }
}

pub fn mouse() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

/// 본문 레이아웃의 한 단어.
pub struct Word {
    pub text: String,
    pub code: bool,
    pub x: f32,
    pub w: f32,
    /// 앞 단어와 같은 코드 조각에 속한다 (배경을 이어 그린다).
    pub joined: bool,
}

pub struct Ui {
    pub kr: Option<Font>,
    pub mono: Option<Font>,
}

fn load_first(paths: &[&str]) -> Option<Font> {
    for path in paths {
        if let Ok(bytes) = std::fs::read(path) {
            if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                return Some(font);
            }
        }
    }
    None
}

impl Ui {
    pub fn load() -> Ui {
        let kr = load_first(&[
            "/System/Library/Fonts/AppleSDGothicNeo.ttc",
            "/System/Library/Fonts/Supplemental/AppleGothic.ttf",
            "C:\\Windows\\Fonts\\malgun.ttf",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/nanum/NanumGothic.ttf",
        ]);
        let mono = load_first(&[
            "/System/Library/Fonts/Menlo.ttc",
            "/System/Library/Fonts/Monaco.ttf",
            "C:\\Windows\\Fonts\\consola.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        ]);
        Ui { kr, mono }
    }

    pub fn text(&self, s: &str, x: f32, y: f32, size: u16, color: Color) {
        draw_text_ex(
            s,
            x,
            y,
            TextParams {
                font: self.kr.as_ref(),
                font_size: size,
                color,
                ..Default::default()
            },
        );
    }

    pub fn width(&self, s: &str, size: u16) -> f32 {
        measure_text(s, self.kr.as_ref(), size, 1.0).width
    }

    pub fn text_centered(&self, s: &str, cx: f32, y: f32, size: u16, color: Color) {
        self.text(s, cx - self.width(s, size) / 2.0, y, size, color);
    }

    /// 고정폭 글꼴의 한 칸 너비.
    pub fn cell_w(&self, size: u16) -> f32 {
        measure_text("MMMMMMMMMM", self.mono.as_ref(), size, 1.0).width / 10.0
    }

    /// 고정폭 격자에 맞춰 그린다. ASCII 는 고정폭 글꼴, 그 외는 한글 글꼴로 두 칸을 차지한다.
    pub fn grid_text(&self, s: &str, x: f32, y: f32, size: u16, color: Color) {
        let cw = self.cell_w(size);
        let mut col = 0usize;
        let mut run = String::new();
        let mut run_col = 0usize;
        let flush = |run: &mut String, run_col: usize| {
            if !run.is_empty() {
                draw_text_ex(
                    run.as_str(),
                    x + run_col as f32 * cw,
                    y,
                    TextParams {
                        font: self.mono.as_ref(),
                        font_size: size,
                        color,
                        ..Default::default()
                    },
                );
                run.clear();
            }
        };
        for c in s.chars() {
            if c.is_ascii() {
                if run.is_empty() {
                    run_col = col;
                }
                run.push(c);
                col += 1;
            } else {
                flush(&mut run, run_col);
                let mut buf = [0u8; 4];
                // 두 칸을 채우도록 조금 크게 그린다.
                self.text(c.encode_utf8(&mut buf), x + col as f32 * cw, y, size + 2, color);
                col += 2;
            }
        }
        flush(&mut run, run_col);
    }

    pub fn button(&self, r: Rect, label: &str, bg: Color, fg: Color) -> bool {
        let hover = r.contains(mouse());
        let fill = if hover { mix(bg, WHITE, 0.14) } else { bg };
        rrect(r.x, r.y, r.w, r.h, 8.0, fill);
        self.text_centered(label, r.x + r.w / 2.0, r.y + r.h / 2.0 + 5.5, 16, fg);
        hover && is_mouse_button_pressed(MouseButton::Left)
    }

    /// `백틱` 코드 조각이 섞인 글을 너비에 맞춰 줄바꿈한다.
    pub fn layout(&self, text: &str, size: u16, max_w: f32) -> Vec<Vec<Word>> {
        let space = size as f32 * 0.3;
        let cw = self.cell_w(size - 1);
        let mut lines = Vec::new();
        for para in text.split('\n') {
            // (글자, 코드 여부, 앞에 공백이 있었나)
            let mut tokens: Vec<(String, bool, bool)> = Vec::new();
            let mut cur = String::new();
            let mut code = false;
            let mut pending_space = false;
            let mut cur_space = false;
            for ch in para.chars() {
                if ch == '`' || ch == ' ' {
                    if !cur.is_empty() {
                        tokens.push((std::mem::take(&mut cur), code, cur_space));
                        cur_space = false;
                    }
                    if ch == '`' {
                        code = !code;
                    } else {
                        pending_space = true;
                    }
                } else {
                    if cur.is_empty() {
                        cur_space = pending_space;
                        pending_space = false;
                    }
                    cur.push(ch);
                }
            }
            if !cur.is_empty() {
                tokens.push((cur, code, cur_space));
            }

            let mut line: Vec<Word> = Vec::new();
            let mut x = 0.0f32;
            for (text, code, space_before) in tokens {
                let w = if code {
                    str_cols(&text) as f32 * cw
                } else {
                    self.width(&text, size)
                };
                let prev_code = line.last().map(|p| p.code).unwrap_or(false);
                let gap = if !space_before || line.is_empty() {
                    0.0
                } else if code && prev_code {
                    cw
                } else {
                    space
                };
                if space_before && !line.is_empty() && x + gap + w > max_w {
                    lines.push(std::mem::take(&mut line));
                    x = 0.0;
                } else {
                    x += gap;
                }
                let joined = code && space_before && line.last().map(|p| p.code).unwrap_or(false);
                line.push(Word {
                    text,
                    code,
                    x,
                    w,
                    joined,
                });
                x += w;
            }
            lines.push(line);
        }
        lines
    }
}
