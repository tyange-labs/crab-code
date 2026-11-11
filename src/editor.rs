// 간단한 코드 에디터: 커서 이동, 자동 들여쓰기, 문법 색칠, 붙여넣기.

use crate::ui::*;
use macroquad::miniquad::window::{clipboard_get, clipboard_set};
use macroquad::prelude::*;

const SIZE: u16 = 16;
const LINE_H: f32 = 23.0;

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "while", "for", "in", "loop", "match", "break", "continue",
    "return", "struct", "enum", "impl", "use", "pub", "true", "false", "self", "as", "const",
    "static", "mod", "ref", "move", "where", "trait", "type",
];

const C_KEYWORD: Color = hex(0xff8f5a);
const C_FUNC: Color = hex(0x7fc8ff);
const C_TYPE: Color = hex(0x6fe0c0);
const C_NUM: Color = hex(0xd9a8ff);
const C_STR: Color = hex(0xb5e07a);
const C_COMMENT: Color = hex(0x717791);
const C_PUNCT: Color = hex(0xb4b9d0);

pub struct Editor {
    pub lines: Vec<Vec<char>>,
    pub row: usize,
    pub col: usize,
    pub error_row: Option<usize>,
    scroll: usize,
    held: Option<(KeyCode, f32)>,
    blink: f32,
}

fn highlight(line: &[char]) -> Vec<Color> {
    let mut colors = vec![TEXT; line.len()];
    let mut i = 0;
    while i < line.len() {
        let c = line[i];
        if c == '/' && line.get(i + 1) == Some(&'/') {
            for color in &mut colors[i..] {
                *color = C_COMMENT;
            }
            break;
        } else if c == '"' {
            let start = i;
            i += 1;
            while i < line.len() && line[i] != '"' {
                if line[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            i = (i + 1).min(line.len());
            for color in &mut colors[start..i] {
                *color = C_STR;
            }
        } else if c.is_ascii_digit() {
            let start = i;
            while i < line.len() && (line[i].is_ascii_alphanumeric() || line[i] == '_') {
                i += 1;
            }
            for color in &mut colors[start..i] {
                *color = C_NUM;
            }
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < line.len() && (line[i].is_alphanumeric() || line[i] == '_') {
                i += 1;
            }
            let word: String = line[start..i].iter().collect();
            let next = line.get(i).copied();
            let color = if KEYWORDS.contains(&word.as_str()) {
                C_KEYWORD
            } else if c.is_uppercase() {
                C_TYPE
            } else if next == Some('(') || next == Some('!') {
                C_FUNC
            } else {
                TEXT
            };
            for slot in &mut colors[start..i] {
                *slot = color;
            }
        } else {
            if !c.is_whitespace() {
                colors[i] = C_PUNCT;
            }
            i += 1;
        }
    }
    colors
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            lines: vec![Vec::new()],
            row: 0,
            col: 0,
            error_row: None,
            scroll: 0,
            held: None,
            blink: 0.0,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.lines = text.lines().map(|l| l.chars().collect()).collect();
        if self.lines.is_empty() {
            self.lines.push(Vec::new());
        }
        self.row = 0;
        self.col = 0;
        self.scroll = 0;
        self.error_row = None;
    }

    pub fn text(&self) -> String {
        let mut out = String::new();
        for line in &self.lines {
            out.extend(line.iter());
            out.push('\n');
        }
        out
    }

    fn repeat(&mut self, key: KeyCode) -> bool {
        if is_key_pressed(key) {
            self.held = Some((key, 0.0));
            return true;
        }
        if let Some((held_key, t)) = &mut self.held {
            if *held_key == key {
                if !is_key_down(key) {
                    self.held = None;
                    return false;
                }
                *t += get_frame_time();
                if *t > 0.38 {
                    *t -= 0.04;
                    return true;
                }
            }
        }
        false
    }

    fn indent_of(&self, row: usize) -> usize {
        self.lines[row].iter().take_while(|c| **c == ' ').count()
    }

    fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.row];
        // 닫는 중괄호는 한 단계 내어 쓴다.
        if c == '}' && self.col >= 4 && line[..self.col].iter().all(|c| *c == ' ') {
            line.drain(self.col - 4..self.col);
            self.col -= 4;
        }
        line.insert(self.col, c);
        self.col += 1;
    }

    fn newline(&mut self) {
        let mut indent = self.indent_of(self.row).min(self.col);
        let before: String = self.lines[self.row][..self.col].iter().collect();
        if before.trim_end().ends_with('{') {
            indent += 4;
        }
        let rest: Vec<char> = self.lines[self.row].split_off(self.col);
        let mut next = vec![' '; indent];
        next.extend(rest);
        self.lines.insert(self.row + 1, next);
        self.row += 1;
        self.col = indent;
    }

    fn insert_str(&mut self, s: &str) {
        for c in s.replace("\r\n", "\n").replace('\t', "    ").chars() {
            if c == '\n' {
                let rest = self.lines[self.row].split_off(self.col);
                self.lines.insert(self.row + 1, rest);
                self.row += 1;
                self.col = 0;
            } else if !c.is_control() {
                self.lines[self.row].insert(self.col, c);
                self.col += 1;
            }
        }
    }

    fn backspace(&mut self) {
        if self.col > 0 {
            let line = &mut self.lines[self.row];
            let all_spaces = line[..self.col].iter().all(|c| *c == ' ');
            let n = if all_spaces && self.col % 4 == 0 { 4 } else { 1 };
            line.drain(self.col - n..self.col);
            self.col -= n;
        } else if self.row > 0 {
            let line = self.lines.remove(self.row);
            self.row -= 1;
            self.col = self.lines[self.row].len();
            self.lines[self.row].extend(line);
        }
    }

    fn delete(&mut self) {
        if self.col < self.lines[self.row].len() {
            self.lines[self.row].remove(self.col);
        } else if self.row + 1 < self.lines.len() {
            let next = self.lines.remove(self.row + 1);
            self.lines[self.row].extend(next);
        }
    }

    /// 키 입력을 처리한다. 내용이 바뀌면 true.
    pub fn input(&mut self) -> bool {
        let cmd = is_key_down(KeyCode::LeftSuper)
            || is_key_down(KeyCode::RightSuper)
            || is_key_down(KeyCode::LeftControl)
            || is_key_down(KeyCode::RightControl);
        let mut changed = false;
        let mut moved = false;

        while let Some(c) = get_char_pressed() {
            // macOS 는 방향키 등을 U+F700 대의 문자로 보내므로 걸러낸다.
            if cmd || c.is_control() || ('\u{f700}'..='\u{f8ff}').contains(&c) {
                continue;
            }
            self.insert_char(c);
            changed = true;
        }

        if cmd {
            if is_key_pressed(KeyCode::V) {
                if let Some(text) = clipboard_get() {
                    self.insert_str(&text);
                    changed = true;
                }
            }
            if is_key_pressed(KeyCode::C) {
                clipboard_set(&self.text());
            }
            if is_key_pressed(KeyCode::Left) {
                self.col = self.indent_of(self.row);
                moved = true;
            }
            if is_key_pressed(KeyCode::Right) {
                self.col = self.lines[self.row].len();
                moved = true;
            }
        } else {
            if self.repeat(KeyCode::Enter) || self.repeat(KeyCode::KpEnter) {
                self.newline();
                changed = true;
            }
            if self.repeat(KeyCode::Backspace) {
                self.backspace();
                changed = true;
            }
            if self.repeat(KeyCode::Delete) {
                self.delete();
                changed = true;
            }
            if self.repeat(KeyCode::Tab) {
                let n = 4 - self.col % 4;
                for _ in 0..n {
                    self.lines[self.row].insert(self.col, ' ');
                    self.col += 1;
                }
                changed = true;
            }
            if self.repeat(KeyCode::Left) {
                if self.col > 0 {
                    self.col -= 1;
                } else if self.row > 0 {
                    self.row -= 1;
                    self.col = self.lines[self.row].len();
                }
                moved = true;
            }
            if self.repeat(KeyCode::Right) {
                if self.col < self.lines[self.row].len() {
                    self.col += 1;
                } else if self.row + 1 < self.lines.len() {
                    self.row += 1;
                    self.col = 0;
                }
                moved = true;
            }
            if self.repeat(KeyCode::Up) && self.row > 0 {
                self.row -= 1;
                moved = true;
            }
            if self.repeat(KeyCode::Down) && self.row + 1 < self.lines.len() {
                self.row += 1;
                moved = true;
            }
            if is_key_pressed(KeyCode::Home) {
                self.col = self.indent_of(self.row);
                moved = true;
            }
            if is_key_pressed(KeyCode::End) {
                self.col = self.lines[self.row].len();
                moved = true;
            }
        }

        self.col = self.col.min(self.lines[self.row].len());
        if changed {
            self.error_row = None;
        }
        if changed || moved {
            self.blink = 0.0;
        }
        changed
    }

    pub fn draw(&mut self, ui: &Ui, r: Rect) {
        self.blink += get_frame_time();
        let cw = ui.cell_w(SIZE);
        let gutter = cw * 4.0 + 10.0;
        let top = r.y + 10.0;
        let rows = (((r.h - 20.0) / LINE_H).floor() as usize).max(1);
        let max_cols = (((r.w - gutter - 12.0) / cw).floor() as usize).max(8);

        // 마우스 클릭으로 커서 옮기기
        let m = mouse();
        if is_mouse_button_pressed(MouseButton::Left) && r.contains(m) {
            let row = self.scroll + ((m.y - top) / LINE_H).floor().max(0.0) as usize;
            self.row = row.min(self.lines.len() - 1);
            let target = ((m.x - r.x - gutter) / cw).round().max(0.0) as usize;
            let mut cols = 0;
            self.col = self.lines[self.row].len();
            for (i, c) in self.lines[self.row].iter().enumerate() {
                if cols >= target {
                    self.col = i;
                    break;
                }
                cols += char_cols(*c);
            }
            self.blink = 0.0;
        }

        if self.row < self.scroll {
            self.scroll = self.row;
        }
        if self.row >= self.scroll + rows {
            self.scroll = self.row + 1 - rows;
        }

        rrect(r.x, r.y, r.w, r.h, 12.0, hex(0x1b1d2c));
        for i in 0..rows {
            let li = self.scroll + i;
            if li >= self.lines.len() {
                break;
            }
            let y = top + i as f32 * LINE_H;
            let base = y + LINE_H * 0.72;
            if self.error_row == Some(li) {
                draw_rectangle(r.x + 4.0, y, r.w - 8.0, LINE_H, hex(0x4b2233));
            } else if li == self.row {
                draw_rectangle(r.x + 4.0, y, r.w - 8.0, LINE_H, hex(0x24273a));
            }
            let num_color = if self.error_row == Some(li) { ERR_RED } else { C_COMMENT };
            ui.grid_text(&format!("{:>3}", li + 1), r.x + 8.0, base, SIZE, num_color);

            let line = &self.lines[li];
            let colors = highlight(line);
            let x0 = r.x + gutter;
            let mut col = 0usize;
            let mut run = String::new();
            let mut run_col = 0usize;
            let mut run_color = TEXT;
            for (i, c) in line.iter().enumerate() {
                if col + char_cols(*c) > max_cols {
                    break;
                }
                if !run.is_empty() && colors[i] != run_color {
                    ui.grid_text(&run, x0 + run_col as f32 * cw, base, SIZE, run_color);
                    run.clear();
                }
                if run.is_empty() {
                    run_col = col;
                    run_color = colors[i];
                }
                run.push(*c);
                col += char_cols(*c);
            }
            if !run.is_empty() {
                ui.grid_text(&run, x0 + run_col as f32 * cw, base, SIZE, run_color);
            }

            if li == self.row && self.blink % 1.0 < 0.55 {
                let cols: usize = line[..self.col.min(line.len())].iter().map(|c| char_cols(*c)).sum();
                draw_rectangle(x0 + cols as f32 * cw, y + 3.0, 2.0, LINE_H - 6.0, TEXT);
            }
        }
    }
}
