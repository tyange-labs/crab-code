// Crab Code — 꽃게 페리스를 Rust 코드로 움직이며 배우는 퍼즐 게임.
//
// 실행:        cargo run --release
// 레벨 검증:   cargo run --release -- --verify   (모든 정답/시작 코드를 실제로 컴파일해 본다)

mod editor;
mod lessons;
mod runner;
#[allow(dead_code)] // 일부 함수는 플레이어 프로그램 쪽에서만 쓴다.
mod sim;
mod ui;

use editor::Editor;
use lessons::{CHAPTERS, LESSONS};
use macroquad::prelude::*;
use runner::{OutLine, RunResult};
use sim::{Action, World, LEVEL_COUNT};
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use ui::*;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Idle,
    Compiling,
    Playing,
    Won,
    Failed,
}

#[derive(Clone, Copy)]
enum Tone {
    Normal,
    Dim,
    Good,
    Bad,
    Warn,
}

struct Anim {
    from_x: f32,
    from_y: f32,
    from_dir: i32,
    t: f32,
    action: Action,
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    color: Color,
}

struct App {
    level: usize,
    codes: Vec<String>,
    cleared: Vec<bool>,
    editor: Editor,
    world: World,
    phase: Phase,
    banner: String,
    console: Vec<(String, Tone)>,
    console_scroll: usize,
    console_follow: bool,
    rx: Option<Receiver<RunResult>>,
    queue: VecDeque<OutLine>,
    tail: Option<RunResult>,
    anim: Option<Anim>,
    bump: f32,
    speed_idx: usize,
    lesson_scroll: f32,
    lesson_cache: Option<(usize, i32, Vec<Vec<Word>>)>,
    particles: Vec<Particle>,
    confirm_solution: bool,
    persist: bool,
    time: f32,
}

const SPEEDS: [f32; 3] = [1.0, 2.0, 4.0];

fn now_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(7)
}

fn save_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(|home| PathBuf::from(home).join(".crab-code"))
}

fn compiler_tone(line: &str) -> Tone {
    let t = line.trim_start();
    if t.starts_with("error") {
        Tone::Bad
    } else if t.starts_with("warning") {
        Tone::Warn
    } else if t.starts_with("help") || t.starts_with("= help") {
        Tone::Good
    } else if t.starts_with("-->") || t.starts_with("= note") || t.starts_with("note") {
        Tone::Dim
    } else {
        Tone::Normal
    }
}

fn gem_color(color: sim::Color, shiny: bool) -> Color {
    let base = match color {
        sim::Color::Red => hex(0xff5c7a),
        sim::Color::Blue => hex(0x4fb3ff),
    };
    if shiny {
        base
    } else {
        mix(base, hex(0x666a7c), 0.72)
    }
}

fn draw_gem(x: f32, y: f32, r: f32, color: sim::Color, shiny: bool) {
    let c = gem_color(color, shiny);
    draw_poly(x, y, 4, r, 0.0, mix(c, BLACK, 0.25));
    draw_poly(x, y, 4, r * 0.84, 0.0, c);
    if shiny {
        draw_poly(x - r * 0.12, y - r * 0.18, 4, r * 0.36, 0.0, mix(c, WHITE, 0.55));
    }
}

fn draw_crab(cx: f32, cy: f32, s: f32, angle: f32, walk: f32) {
    let (sn, cs) = angle.sin_cos();
    let p = |fx: f32, fy: f32| vec2(cx + (fx * cs - fy * sn) * s, cy + (fx * sn + fy * cs) * s);
    let shell = hex(0xf4511e);
    let dark = hex(0xb93a12);
    for side in [-1.0f32, 1.0] {
        for i in 0..3 {
            let fx = -0.17 + i as f32 * 0.12;
            let wiggle = (walk * PI * 4.0 + i as f32 * 2.0).sin() * 0.035;
            let a = p(fx, side * 0.18);
            let b = p(fx - 0.03 + wiggle, side * 0.41);
            draw_line(a.x, a.y, b.x, b.y, s * 0.05, dark);
        }
        let a = p(0.1, side * 0.18);
        let b = p(0.3, side * 0.3);
        draw_line(a.x, a.y, b.x, b.y, s * 0.06, dark);
        draw_circle(b.x, b.y, s * 0.105, shell);
        let tip = p(0.385, side * 0.27);
        draw_circle(tip.x, tip.y, s * 0.04, dark);
    }
    let c = p(0.0, 0.0);
    draw_circle(c.x, c.y, s * 0.275, dark);
    draw_circle(c.x, c.y, s * 0.245, shell);
    for side in [-1.0f32, 1.0] {
        let e = p(0.16, side * 0.105);
        draw_circle(e.x, e.y, s * 0.078, WHITE);
        let pupil = p(0.19, side * 0.105);
        draw_circle(pupil.x, pupil.y, s * 0.04, hex(0x1a1a22));
    }
}

/// `코드` 조각이 섞인 한 줄을 그린다.
fn draw_rich_line(ui: &Ui, line: &[Word], x0: f32, y: f32, size: u16, line_h: f32) {
    let base = y + line_h * 0.72;
    let cw = ui.cell_w(size - 1);
    // 배경을 먼저 다 그리고 글자를 그린다 (이어진 배경이 앞 글자를 덮지 않도록).
    for word in line.iter().filter(|w| w.code) {
        let x = x0 + word.x;
        let extra = if word.joined { cw } else { 0.0 };
        rrect(x - 4.0 - extra, y + 1.0, word.w + 8.0 + extra, line_h - 4.0, 5.0, hex(0x33374f));
    }
    for word in line {
        let x = x0 + word.x;
        if word.code {
            ui.grid_text(&word.text, x, base, size - 1, hex(0xffc66d));
        } else {
            let color = if word.text == "목표:" { OK_GREEN } else { TEXT };
            ui.text(&word.text, x, base, size, color);
        }
    }
}

impl App {
    fn new(persist: bool) -> App {
        let mut codes: Vec<String> = LESSONS.iter().map(|l| l.start.to_string()).collect();
        let mut cleared = vec![false; LEVEL_COUNT];
        if persist {
            if let Some(dir) = save_dir() {
                for (i, code) in codes.iter_mut().enumerate() {
                    if let Ok(saved) = std::fs::read_to_string(dir.join(format!("{}.rs", LESSONS[i].id))) {
                        *code = saved;
                    }
                }
                if let Ok(ids) = std::fs::read_to_string(dir.join("cleared.txt")) {
                    for id in ids.lines() {
                        if let Some(i) = LESSONS.iter().position(|l| l.id == id.trim()) {
                            cleared[i] = true;
                        }
                    }
                }
            }
        }
        // 아직 깨지 않은 첫 레벨에서 시작한다.
        let level = cleared.iter().position(|c| !*c).unwrap_or(0);
        let mut app = App {
            level,
            codes,
            cleared,
            editor: Editor::new(),
            world: sim::build_level(level, now_seed()),
            phase: Phase::Idle,
            banner: String::new(),
            console: Vec::new(),
            console_scroll: 0,
            console_follow: true,
            rx: None,
            queue: VecDeque::new(),
            tail: None,
            anim: None,
            bump: 0.0,
            speed_idx: 0,
            lesson_scroll: 0.0,
            lesson_cache: None,
            particles: Vec::new(),
            confirm_solution: false,
            persist,
            time: 0.0,
        };
        app.editor.set_text(&app.codes[level]);
        app.intro();
        app
    }

    fn save(&mut self) {
        self.codes[self.level] = self.editor.text();
        if !self.persist {
            return;
        }
        if let Some(dir) = save_dir() {
            let _ = std::fs::create_dir_all(&dir);
            let _ = std::fs::write(
                dir.join(format!("{}.rs", LESSONS[self.level].id)),
                &self.codes[self.level],
            );
            let ids: Vec<&str> = (0..LEVEL_COUNT).filter(|i| self.cleared[*i]).map(|i| LESSONS[i].id).collect();
            let _ = std::fs::write(dir.join("cleared.txt"), ids.join("\n"));
        }
    }

    fn log(&mut self, text: impl Into<String>, tone: Tone) {
        self.console.push((text.into(), tone));
    }

    fn intro(&mut self) {
        self.console.clear();
        self.console_follow = true;
        self.log("실행 버튼(F5 또는 Cmd+Enter)을 누르면 코드를 진짜 rustc 로 컴파일해서 돌려요.", Tone::Dim);
        self.log("println! 출력과 컴파일러 메시지가 여기에 나옵니다.", Tone::Dim);
    }

    fn reset_world(&mut self) {
        self.world = sim::build_level(self.level, now_seed());
        self.phase = Phase::Idle;
        self.queue.clear();
        self.anim = None;
        self.rx = None;
        self.tail = None;
        self.bump = 0.0;
        self.particles.clear();
    }

    fn set_level(&mut self, idx: usize) {
        self.save();
        self.level = idx;
        self.editor.set_text(&self.codes[idx]);
        self.reset_world();
        self.intro();
        self.lesson_scroll = 0.0;
        self.confirm_solution = false;
    }

    fn start_run(&mut self) {
        if self.phase == Phase::Compiling {
            return;
        }
        self.save();
        let seed = now_seed();
        self.reset_world();
        self.world = sim::build_level(self.level, seed);
        self.editor.error_row = None;
        self.console.clear();
        self.console_follow = true;
        self.log("컴파일 중...", Tone::Dim);
        let (tx, rx) = channel();
        let code = self.editor.text();
        let level = self.level;
        std::thread::spawn(move || {
            let _ = tx.send(runner::run(&code, level, seed));
        });
        self.rx = Some(rx);
        self.phase = Phase::Compiling;
    }

    fn poll_result(&mut self) {
        let Some(rx) = &self.rx else { return };
        let Ok(result) = rx.try_recv() else { return };
        self.rx = None;
        self.console.clear();
        if !result.compiled {
            for line in result.compiler_output.lines() {
                self.log(line, compiler_tone(line));
            }
            self.editor.error_row = runner::first_error_line(&result.compiler_output).map(|n| n - 1);
            self.phase = Phase::Failed;
            self.banner = "컴파일 에러! 아래 컴파일러 메시지를 읽어 보세요.".into();
            self.console_follow = false;
            self.console_scroll = 0;
            return;
        }
        for line in result.compiler_output.lines() {
            let tone = match compiler_tone(line) {
                Tone::Warn => Tone::Warn,
                _ => Tone::Dim,
            };
            self.log(line, tone);
        }
        self.queue = result.lines.iter().cloned().collect();
        self.tail = Some(result);
        self.phase = Phase::Playing;
    }

    fn fail(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.log(msg.clone(), Tone::Bad);
        self.banner = msg;
        self.phase = Phase::Failed;
        self.queue.clear();
        self.console_follow = true;
    }

    fn playback(&mut self, dt: f32) {
        if let Some(anim) = &mut self.anim {
            let base = match anim.action {
                Action::Forward => 0.24,
                Action::Left | Action::Right => 0.18,
                _ => 0.22,
            };
            anim.t += dt * SPEEDS[self.speed_idx] / base;
            if anim.t < 1.0 {
                return;
            }
            self.anim = None;
        }
        loop {
            match self.queue.pop_front() {
                Some(OutLine::Print(text)) => self.log(text, Tone::Normal),
                Some(OutLine::Limit) => {
                    self.fail("너무 많이 움직였어요. 반복문이 끝나지 않는 건 아닐까요?");
                    return;
                }
                Some(OutLine::Act(action)) => {
                    let from = (self.world.x as f32, self.world.y as f32, self.world.dir);
                    match self.world.apply(action) {
                        Ok(()) => {
                            self.anim = Some(Anim {
                                from_x: from.0,
                                from_y: from.1,
                                from_dir: from.2,
                                t: 0.0,
                                action,
                            });
                        }
                        Err(msg) => {
                            if action == Action::Forward {
                                self.bump = 0.3;
                            }
                            self.fail(msg);
                        }
                    }
                    return;
                }
                None => {
                    self.finish_run();
                    return;
                }
            }
        }
    }

    fn finish_run(&mut self) {
        let tail = self.tail.take().unwrap_or_default();
        if tail.timed_out {
            self.fail("시간 초과! 프로그램이 끝나지 않아요. 무한 루프인지 확인해 보세요.");
            return;
        }
        if tail.crashed {
            for line in tail.runtime_stderr.lines() {
                self.log(line, Tone::Bad);
            }
            self.fail("프로그램이 실행 중에 멈췄어요 (패닉).");
            return;
        }
        match self.world.todo_hint() {
            Some(todo) => self.fail(format!("아직이에요. {todo}")),
            None => {
                self.phase = Phase::Won;
                self.cleared[self.level] = true;
                self.log("성공! 목표를 달성했어요.", Tone::Good);
                self.console_follow = true;
                self.save();
                for i in 0..90 {
                    let a = i as f32 * 2.399;
                    let speed = 120.0 + ((i * 37) % 100) as f32 * 2.6;
                    self.particles.push(Particle {
                        pos: vec2(0.0, 0.0),
                        vel: vec2(a.cos() * speed, a.sin() * speed - 180.0),
                        life: 1.2 + ((i * 13) % 10) as f32 * 0.08,
                        color: [ACCENT, OK_GREEN, SKY, WARN, hex(0xff5c7a)][i % 5],
                    });
                }
            }
        }
    }

    fn frame(&mut self, ui: &Ui) {
        let dt = get_frame_time().min(0.05);
        self.time += dt;
        let (sw, sh) = (screen_width(), screen_height());
        let pad = 14.0;
        let top = 58.0;
        let left_w = (sw * 0.43).clamp(420.0, 660.0);
        let lesson_r = Rect::new(pad, top, left_w, ((sh - top - pad) * 0.5).floor());
        let btn_h = 40.0;
        let editor_y = lesson_r.bottom() + pad;
        let editor_r = Rect::new(pad, editor_y, left_w, sh - editor_y - pad - btn_h - 10.0);
        let btn_y = editor_r.bottom() + 10.0;
        let right_x = pad + left_w + pad;
        let right_w = sw - right_x - pad;
        let world_r = Rect::new(right_x, top, right_w, ((sh - top - pad) * 0.6).floor());
        let console_y = world_r.bottom() + pad;
        let console_r = Rect::new(right_x, console_y, right_w, sh - console_y - pad);

        // ── 입력과 진행 ─────────────────────────────────────────────
        if self.editor.input() {
            self.confirm_solution = false;
        }
        let cmd = is_key_down(KeyCode::LeftSuper)
            || is_key_down(KeyCode::RightSuper)
            || is_key_down(KeyCode::LeftControl)
            || is_key_down(KeyCode::RightControl);
        if is_key_pressed(KeyCode::F5)
            || (cmd && (is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)))
        {
            self.start_run();
        }
        self.poll_result();
        if self.phase == Phase::Playing {
            self.playback(dt);
        }
        self.bump = (self.bump - dt).max(0.0);

        // ── 상단 바 ────────────────────────────────────────────────
        draw_crab(34.0, 30.0, 40.0, -PI / 2.0, self.time * 0.25);
        ui.text("Crab Code", 62.0, 30.0, 22, TEXT);
        ui.text("Rust 를 코드로 배우는 퍼즐", 62.0, 48.0, 13, DIM);
        // 장 선택 칩
        let chip = 34.0;
        let chips_w = CHAPTERS.len() as f32 * (chip + 6.0) - 6.0;
        ui.text("장", sw - pad - chips_w - 24.0, 35.0, 14, DIM);
        let current_chapter = LESSONS[self.level].chapter;
        let mut goto = None;
        for (ci, (short, _)) in CHAPTERS.iter().enumerate() {
            let r = Rect::new(sw - pad - chips_w + ci as f32 * (chip + 6.0), 14.0, chip, 30.0);
            let hover = r.contains(mouse());
            let levels: Vec<usize> = (0..LEVEL_COUNT).filter(|i| LESSONS[*i].chapter == ci).collect();
            let done = levels.iter().all(|i| self.cleared[*i]);
            let (bg, fg) = if ci == current_chapter {
                (ACCENT, WHITE)
            } else if done {
                (hex(0x1f4a35), OK_GREEN)
            } else {
                (PANEL, DIM)
            };
            rrect(r.x, r.y, r.w, r.h, 8.0, if hover { mix(bg, WHITE, 0.15) } else { bg });
            ui.text_centered(short, r.x + chip / 2.0, r.y + 20.5, 15, fg);
            if hover && is_mouse_button_pressed(MouseButton::Left) && ci != current_chapter {
                // 그 장에서 아직 깨지 않은 첫 레벨로 간다.
                goto = levels.iter().copied().find(|i| !self.cleared[*i]).or(levels.first().copied());
            }
        }
        if let Some(i) = goto {
            self.set_level(i);
        }

        self.draw_lesson(ui, lesson_r);
        self.editor.draw(ui, editor_r);

        // ── 버튼 ───────────────────────────────────────────────────
        let lesson = &LESSONS[self.level];
        let mut bx = pad;
        let run_label = if self.phase == Phase::Compiling { "컴파일 중..." } else { "실행  (F5)" };
        if ui.button(Rect::new(bx, btn_y, 160.0, btn_h), run_label, ACCENT, WHITE) {
            self.start_run();
        }
        bx += 170.0;
        if ui.button(Rect::new(bx, btn_y, 112.0, btn_h), "처음 코드로", PANEL_HI, TEXT) {
            self.editor.set_text(lesson.start);
            self.reset_world();
            self.intro();
        }
        bx += 122.0;
        if ui.button(Rect::new(bx, btn_y, 76.0, btn_h), "힌트", PANEL_HI, TEXT) {
            self.log(format!("힌트: {}", lesson.hint), Tone::Warn);
            self.console_follow = true;
        }
        bx += 86.0;
        let sol_label = if self.confirm_solution { "정말 볼까요?" } else { "정답 보기" };
        if ui.button(Rect::new(bx, btn_y, 112.0, btn_h), sol_label, PANEL_HI, TEXT) {
            if self.confirm_solution {
                self.editor.set_text(lesson.solution);
                self.confirm_solution = false;
            } else {
                self.confirm_solution = true;
            }
        }

        // ── 월드 ───────────────────────────────────────────────────
        rrect(world_r.x, world_r.y, world_r.w, world_r.h, 12.0, PANEL);
        self.draw_world(ui, world_r);
        let speed_r = Rect::new(world_r.right() - 96.0, world_r.y + 10.0, 84.0, 30.0);
        if ui.button(speed_r, &format!("속도 {}x", SPEEDS[self.speed_idx] as i32), PANEL_HI, TEXT) {
            self.speed_idx = (self.speed_idx + 1) % SPEEDS.len();
        }
        let bar = Rect::new(world_r.x + 12.0, world_r.bottom() - 52.0, world_r.w - 24.0, 40.0);
        match self.phase {
            Phase::Won => {
                rrect(bar.x, bar.y, bar.w, bar.h, 8.0, hex(0x1f4a35));
                if self.level + 1 < LEVEL_COUNT {
                    ui.text("성공! 잘했어요.", bar.x + 14.0, bar.y + 26.0, 17, OK_GREEN);
                    let next = Rect::new(bar.right() - 124.0, bar.y + 4.0, 120.0, 32.0);
                    if ui.button(next, "다음 레벨", OK_GREEN, hex(0x10261b)) {
                        self.set_level(self.level + 1);
                    }
                } else {
                    ui.text("졸업을 축하해요! 이제 진짜 Rust 를 쓸 준비가 됐어요.", bar.x + 14.0, bar.y + 26.0, 17, OK_GREEN);
                }
            }
            Phase::Failed => {
                rrect(bar.x, bar.y, bar.w, bar.h, 8.0, hex(0x4b2233));
                ui.text(&self.banner, bar.x + 14.0, bar.y + 26.0, 16, ERR_RED);
            }
            Phase::Compiling => {
                let dots = ".".repeat(1 + (self.time * 3.0) as usize % 3);
                ui.text(&format!("rustc 가 코드를 컴파일하는 중{dots}"), bar.x + 14.0, bar.y + 26.0, 16, DIM);
            }
            Phase::Idle | Phase::Playing => {
                // 목표는 본문을 스크롤하지 않아도 항상 보이게 여기에 고정한다.
                if let Some(goal) = lesson.body.lines().find(|l| l.starts_with("목표:")) {
                    let lines = ui.layout(goal, 15, bar.w - 28.0);
                    let line_h = 23.0;
                    let mut y = bar.bottom() - 6.0 - lines.len() as f32 * line_h;
                    for line in &lines {
                        draw_rich_line(ui, line, bar.x + 14.0, y, 15, line_h);
                        y += line_h;
                    }
                }
            }
        }

        // 축하 색종이
        let origin = vec2(world_r.x + world_r.w / 2.0, world_r.y + world_r.h * 0.45);
        for p in &mut self.particles {
            p.vel.y += 520.0 * dt;
            p.pos += p.vel * dt;
            p.life -= dt;
            let size = 7.0 * p.life.clamp(0.0, 1.0);
            draw_rectangle(origin.x + p.pos.x, origin.y + p.pos.y, size, size, p.color);
        }
        self.particles.retain(|p| p.life > 0.0);

        self.draw_console(ui, console_r);
    }

    fn draw_lesson(&mut self, ui: &Ui, r: Rect) {
        let lesson = &LESSONS[self.level];
        rrect(r.x, r.y, r.w, r.h, 12.0, PANEL);
        // 장 이름과, 그 장 안의 레벨 칩
        ui.text(CHAPTERS[lesson.chapter].1, r.x + 18.0, r.y + 27.0, 13, DIM);
        let levels: Vec<usize> = (0..LEVEL_COUNT).filter(|i| LESSONS[*i].chapter == lesson.chapter).collect();
        let chip = 24.0;
        let mut goto = None;
        for (n, li) in levels.iter().enumerate() {
            let cr = Rect::new(r.right() - 14.0 - (levels.len() - n) as f32 * (chip + 5.0) + 5.0, r.y + 10.0, chip, 24.0);
            let hover = cr.contains(mouse());
            let (bg, fg) = if *li == self.level {
                (ACCENT, WHITE)
            } else if self.cleared[*li] {
                (hex(0x1f4a35), OK_GREEN)
            } else {
                (PANEL_HI, DIM)
            };
            rrect(cr.x, cr.y, cr.w, cr.h, 6.0, if hover { mix(bg, WHITE, 0.15) } else { bg });
            ui.text_centered(&format!("{}", n + 1), cr.x + chip / 2.0, cr.y + 17.0, 13, fg);
            if hover && is_mouse_button_pressed(MouseButton::Left) && *li != self.level {
                goto = Some(*li);
            }
        }
        if let Some(i) = goto {
            self.set_level(i);
            return;
        }
        let number = levels.iter().position(|i| *i == self.level).unwrap_or(0) + 1;
        ui.text(&format!("{}. {}", number, lesson.title), r.x + 18.0, r.y + 58.0, 21, ACCENT);
        let size = 16u16;
        let line_h = 25.0;
        let gap = 10.0;
        let body_top = r.y + 76.0;
        let body_bottom = r.bottom() - 10.0;
        let max_w = r.w - 36.0;
        let key = (self.level, max_w as i32);
        if self.lesson_cache.as_ref().map(|c| (c.0, c.1)) != Some(key) {
            self.lesson_cache = Some((key.0, key.1, ui.layout(lesson.body, size, max_w)));
        }
        let lines = &self.lesson_cache.as_ref().unwrap().2;
        let total: f32 = lines.iter().map(|l| if l.is_empty() { gap } else { line_h }).sum();
        let max_scroll = (total - (body_bottom - body_top)).max(0.0);
        if r.contains(mouse()) {
            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                self.lesson_scroll -= wheel.signum() * 30.0;
            }
        }
        self.lesson_scroll = self.lesson_scroll.clamp(0.0, max_scroll);
        let mut y = body_top - self.lesson_scroll;
        for line in lines {
            if line.is_empty() {
                y += gap;
                continue;
            }
            if y >= body_top - 1.0 && y + line_h <= body_bottom + 1.0 {
                draw_rich_line(ui, line, r.x + 18.0, y, size, line_h);
            }
            y += line_h;
        }
        if self.lesson_scroll < max_scroll - 1.0 {
            let label = "스크롤해서 더 보기";
            let w = ui.width(label, 12);
            rrect(r.right() - w - 26.0, r.bottom() - 26.0, w + 16.0, 20.0, 6.0, PANEL_HI);
            ui.text(label, r.right() - w - 18.0, r.bottom() - 12.0, 12, DIM);
        }
    }

    fn draw_world(&self, ui: &Ui, r: Rect) {
        let w = &self.world;
        // 바닥이 있는 범위만 화면에 맞춘다.
        let (mut x0, mut y0, mut x1, mut y1) = (w.w, w.h, 0, 0);
        for y in 0..w.h {
            for x in 0..w.w {
                if !w.is_wall(x, y) {
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x);
                    y1 = y1.max(y);
                }
            }
        }
        let (cols, rows) = ((x1 - x0 + 1) as f32, (y1 - y0 + 1) as f32);
        // 다이얼 / 두루마리 / 쪽지
        let mut info: Vec<(&str, String)> = Vec::new();
        if let Some(d) = w.dial {
            info.push(("다이얼", d.to_string()));
        }
        if !w.scroll.is_empty() {
            info.push(("두루마리", format!("{:?}", w.scroll)));
        }
        if let Some(note) = &w.note {
            info.push(("쪽지", format!("{:?}", note)));
        }
        if let Some(note) = &w.note2 {
            info.push(("쪽지 2", format!("{:?}", note)));
        }
        let mut ix = r.x + 18.0;
        for (label, value) in &info {
            ui.text(label, ix, r.y + 62.0, 15, DIM);
            ix += ui.width(label, 15) + 8.0;
            let vw = str_cols(value) as f32 * ui.cell_w(15);
            rrect(ix - 5.0, r.y + 45.0, vw + 10.0, 24.0, 6.0, hex(0x33374f));
            ui.grid_text(value, ix, r.y + 62.0, 15, hex(0xffc66d));
            ix += vw + 24.0;
        }
        let top = if info.is_empty() { 52.0 } else { 80.0 };
        let area = Rect::new(r.x + 20.0, r.y + top, r.w - 40.0, r.h - top - 76.0);
        let tile = (area.w / cols).min(area.h / rows).min(92.0);
        let ox = area.x + (area.w - tile * cols) / 2.0 - x0 as f32 * tile;
        let oy = area.y + (area.h - tile * rows) / 2.0 - y0 as f32 * tile;
        let center = |x: f32, y: f32| vec2(ox + (x + 0.5) * tile, oy + (y + 0.5) * tile);

        for y in 0..w.h {
            for x in 0..w.w {
                if !w.is_wall(x, y) {
                    let shade = if (x + y) % 2 == 0 { FLOOR } else { mix(FLOOR, BLACK, 0.08) };
                    rrect(
                        ox + x as f32 * tile + 2.0,
                        oy + y as f32 * tile + 2.0,
                        tile - 4.0,
                        tile - 4.0,
                        tile * 0.14,
                        shade,
                    );
                }
            }
        }

        for (x, y, n) in &w.numbers {
            let c = center(*x as f32, *y as f32);
            rrect(c.x - tile * 0.3, c.y - tile * 0.3, tile * 0.6, tile * 0.6, tile * 0.1, hex(0x27405e));
            let size = (tile * 0.42) as u16;
            ui.text_centered(&n.to_string(), c.x, c.y + size as f32 * 0.34, size, SKY);
        }

        for (x, y, sign) in &w.signs {
            let label = match sign {
                sim::Sign::Left => "왼쪽",
                sim::Sign::Right => "오른쪽",
                _ => continue,
            };
            let c = center(*x as f32, *y as f32);
            rrect(c.x - tile * 0.4, c.y - tile * 0.2, tile * 0.8, tile * 0.4, tile * 0.08, WARN);
            let size = (tile * 0.23) as u16;
            ui.text_centered(label, c.x, c.y + size as f32 * 0.36, size, hex(0x3a2c00));
        }

        if let Some((x, y)) = w.flag {
            let c = center(x as f32, y as f32);
            let pole_x = c.x - tile * 0.16;
            draw_line(pole_x, c.y - tile * 0.32, pole_x, c.y + tile * 0.32, tile * 0.05, TEXT);
            let wave = (self.time * 4.0).sin() * tile * 0.03;
            draw_triangle(
                vec2(pole_x, c.y - tile * 0.32),
                vec2(pole_x + tile * 0.4, c.y - tile * 0.17 + wave),
                vec2(pole_x, c.y - tile * 0.02),
                OK_GREEN,
            );
        }

        for p in &w.pedestals {
            let c = center(p.x as f32, p.y as f32);
            let ring = match p.want {
                Some(color) => gem_color(color, true),
                None => hex(0xc9cde0),
            };
            rrect(c.x - tile * 0.36, c.y - tile * 0.36, tile * 0.72, tile * 0.72, tile * 0.12, mix(ring, BLACK, 0.2));
            rrect(c.x - tile * 0.29, c.y - tile * 0.29, tile * 0.58, tile * 0.58, tile * 0.09, hex(0x23263a));
        }

        if let Some(d) = &w.door {
            let c = center(d.x as f32, d.y as f32);
            if d.open {
                draw_rectangle_lines(c.x - tile * 0.4, c.y - tile * 0.4, tile * 0.8, tile * 0.8, tile * 0.07, hex(0x8a5a2b));
            } else {
                rrect(c.x - tile * 0.42, c.y - tile * 0.42, tile * 0.84, tile * 0.84, tile * 0.1, hex(0x8a5a2b));
                rrect(c.x - tile * 0.34, c.y - tile * 0.34, tile * 0.68, tile * 0.68, tile * 0.07, hex(0xa9713a));
                draw_circle(c.x, c.y - tile * 0.04, tile * 0.11, WARN);
                draw_rectangle(c.x - tile * 0.03, c.y, tile * 0.06, tile * 0.16, WARN);
            }
        }

        for g in &w.gems {
            if g.held {
                continue;
            }
            let c = center(g.x as f32, g.y as f32);
            let bob = if g.placed { 0.0 } else { (self.time * 2.6 + g.x as f32).sin() * tile * 0.03 };
            draw_gem(c.x, c.y + bob, tile * 0.24, g.color, g.shiny);
            if w.show_weights {
                let size = (tile * 0.2) as u16;
                ui.text_centered(&g.weight.to_string(), c.x, c.y + bob + size as f32 * 0.36, size, hex(0x1a1a22));
            }
        }

        // 꽃게
        let (mut fx, mut fy, mut angle, mut walk) = (w.x as f32, w.y as f32, w.dir as f32 * PI / 2.0, 0.0);
        if let Some(anim) = &self.anim {
            let t = anim.t.clamp(0.0, 1.0);
            let e = t * t * (3.0 - 2.0 * t);
            fx = anim.from_x + (w.x as f32 - anim.from_x) * e;
            fy = anim.from_y + (w.y as f32 - anim.from_y) * e;
            let delta = ((w.dir - anim.from_dir + 6) % 4 - 2) as f32;
            angle = (anim.from_dir as f32 + delta * e) * PI / 2.0;
            if anim.action == Action::Forward {
                walk = t;
            }
        }
        if self.bump > 0.0 {
            let k = (PI * (1.0 - self.bump / 0.3)).sin() * 0.22;
            fx += sim::DX[w.dir as usize] as f32 * k;
            fy += sim::DY[w.dir as usize] as f32 * k;
        }
        let c = center(fx, fy);
        draw_crab(c.x, c.y, tile, angle, walk);
        if let Some(anim) = &self.anim {
            let ring = match anim.action {
                Action::Collect => Some(WARN),
                Action::Deposit(_) => Some(OK_GREEN),
                Action::Polish(_) => Some(WHITE),
                Action::Unlock(_) => Some(SKY),
                _ => None,
            };
            if let Some(color) = ring {
                let t = anim.t.clamp(0.0, 1.0);
                draw_circle_lines(c.x, c.y, tile * (0.3 + 0.35 * t), tile * 0.05, alpha(color, 1.0 - t));
            }
        }

        // 꽃게가 받침대 위에 서 있으면 올려 둔 보석이 가려지므로 위에 다시 그린다.
        if self.anim.is_none() {
            for g in w.gems.iter().filter(|g| g.placed && g.x == w.x && g.y == w.y) {
                draw_gem(c.x, c.y - tile * 0.3, tile * 0.17, g.color, g.shiny);
            }
        }

        // 가방 (들고 있는 보석)
        ui.text("가방", r.x + 18.0, r.y + 31.0, 15, DIM);
        let mut bx = r.x + 66.0;
        let mut any = false;
        for g in w.gems.iter().filter(|g| g.held) {
            draw_gem(bx, r.y + 26.0, 10.0, g.color, g.shiny);
            bx += 26.0;
            any = true;
        }
        if !any {
            ui.text("비어 있음", r.x + 58.0, r.y + 31.0, 14, hex(0x5d627a));
        }
    }

    fn draw_console(&mut self, ui: &Ui, r: Rect) {
        rrect(r.x, r.y, r.w, r.h, 12.0, hex(0x1b1d2c));
        ui.text("출력", r.x + 16.0, r.y + 25.0, 14, DIM);
        let size = 14u16;
        let line_h = 19.0;
        let cw = ui.cell_w(size);
        let max_cols = (((r.w - 32.0) / cw).floor() as usize).max(20);
        let rows = (((r.h - 46.0) / line_h).floor() as usize).max(1);

        // 화면 너비에 맞춰 줄을 접는다.
        let mut wrapped: Vec<(String, Tone)> = Vec::new();
        for (text, tone) in &self.console {
            let mut cur = String::new();
            let mut cols = 0;
            for ch in text.chars() {
                if cols + char_cols(ch) > max_cols {
                    wrapped.push((std::mem::take(&mut cur), *tone));
                    cols = 0;
                }
                cur.push(ch);
                cols += char_cols(ch);
            }
            wrapped.push((cur, *tone));
        }

        let max_scroll = wrapped.len().saturating_sub(rows);
        if r.contains(mouse()) {
            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                self.console_follow = false;
                let step = 2;
                self.console_scroll = if wheel > 0.0 {
                    self.console_scroll.saturating_sub(step)
                } else {
                    self.console_scroll + step
                };
            }
        }
        if self.console_follow {
            self.console_scroll = max_scroll;
        }
        self.console_scroll = self.console_scroll.min(max_scroll);

        for (i, (text, tone)) in wrapped.iter().skip(self.console_scroll).take(rows).enumerate() {
            let color = match tone {
                Tone::Normal => TEXT,
                Tone::Dim => DIM,
                Tone::Good => OK_GREEN,
                Tone::Bad => ERR_RED,
                Tone::Warn => WARN,
            };
            let (x, y) = (r.x + 16.0, r.y + 50.0 + i as f32 * line_h);
            if text.is_ascii() {
                // 컴파일러 출력은 화살표 위치가 맞아야 하므로 고정폭 격자에 그린다.
                ui.grid_text(text, x, y, size, color);
            } else {
                ui.text(text, x, y, size + 1, color);
            }
        }
        if max_scroll > 0 {
            let label = format!("{}-{} / {}줄", self.console_scroll + 1, (self.console_scroll + rows).min(wrapped.len()), wrapped.len());
            ui.text(&label, r.right() - ui.width(&label, 12) - 16.0, r.y + 24.0, 12, DIM);
        }
    }
}

/// 실행 결과를 화면 없이 재생해서 성공 여부를 본다.
fn replay(result: &RunResult, level: usize, seed: u64) -> Result<(), String> {
    if !result.compiled {
        return Err(format!("컴파일 실패\n{}", result.compiler_output));
    }
    if result.timed_out || result.crashed {
        return Err(format!("실행 실패: {}", result.runtime_stderr));
    }
    let mut world = sim::build_level(level, seed);
    for line in &result.lines {
        match line {
            OutLine::Act(action) => world.apply(*action)?,
            OutLine::Limit => return Err("행동 횟수 초과".into()),
            OutLine::Print(_) => {}
        }
    }
    match world.todo_hint() {
        Some(todo) => Err(todo),
        None => Ok(()),
    }
}

/// 모든 레슨의 정답이 통과하고, 시작 코드는 통과하지 못하는지 확인한다.
fn verify() -> bool {
    let mut all_ok = true;
    if LESSONS.len() != LEVEL_COUNT {
        println!("FAIL  레슨 수({})와 레벨 수({})가 다름", LESSONS.len(), LEVEL_COUNT);
        return false;
    }
    for (i, lesson) in LESSONS.iter().enumerate() {
        if lesson.id != sim::SPECS[i].id {
            println!("FAIL  {}번째 레슨 id 불일치: {} / {}", i + 1, lesson.id, sim::SPECS[i].id);
            all_ok = false;
            continue;
        }
        let mut notes = Vec::new();
        for seed in 1..=6u64 {
            if let Err(e) = replay(&runner::run(lesson.solution, i, seed), i, seed) {
                notes.push(format!("정답 실패 (seed {seed}): {e}"));
                break;
            }
        }
        let start = runner::run(lesson.start, i, 1);
        match lesson.start_error {
            Some(code) => {
                if start.compiled || !start.compiler_output.contains(code) {
                    notes.push(format!("시작 코드가 {code} 에러를 내지 않음"));
                }
                if runner::first_error_line(&start.compiler_output).is_none() {
                    notes.push("에러 줄 번호를 찾지 못함".into());
                }
            }
            None => {
                if !start.compiled {
                    notes.push(format!("시작 코드 컴파일 실패\n{}", start.compiler_output));
                } else if replay(&start, i, 1).is_ok() {
                    notes.push("시작 코드가 그대로 통과해 버림".into());
                }
            }
        }
        if notes.is_empty() {
            println!("ok    {:<10} {}", lesson.id, lesson.title);
        } else {
            all_ok = false;
            println!("FAIL  {:<10} {}", lesson.id, lesson.title);
            for note in notes {
                println!("      {note}");
            }
        }
    }
    runner::cleanup();
    all_ok
}

async fn game() {
    // 개발용 환경 변수: CRAB_START=레벨번호, CRAB_DEMO=1 (정답 자동 실행), CRAB_SHOT=png 경로
    let shot = std::env::var("CRAB_SHOT").ok();
    let demo = std::env::var("CRAB_DEMO").is_ok();
    let shot_at: f32 = std::env::var("CRAB_SHOT_AT").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let dev = shot.is_some() || demo || std::env::var("CRAB_START").is_ok();

    let ui = Ui::load();
    let mut app = App::new(!dev);
    if let Some(n) = std::env::var("CRAB_START").ok().and_then(|v| v.parse::<usize>().ok()) {
        app.set_level(n.clamp(1, LEVEL_COUNT) - 1);
    }
    if demo {
        // CRAB_DEMO=start 면 시작 코드를 그대로 실행한다.
        if std::env::var("CRAB_DEMO").as_deref() != Ok("start") {
            app.editor.set_text(LESSONS[app.level].solution);
        }
        app.start_run();
    }
    loop {
        if is_quit_requested() {
            app.save();
            runner::cleanup();
            break;
        }
        clear_background(BG);
        app.frame(&ui);
        if let Some(path) = &shot {
            if app.time >= shot_at {
                get_screen_data().export_png(path);
                runner::cleanup();
                std::process::exit(0);
            }
        }
        next_frame().await;
    }
}

fn main() {
    if std::env::args().any(|a| a == "--verify") {
        std::process::exit(if verify() { 0 } else { 1 });
    }
    let conf = Conf {
        window_title: "Crab Code — Rust 배우기".to_string(),
        window_width: 1360,
        window_height: 860,
        high_dpi: true,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    };
    macroquad::Window::from_config(conf, async {
        prevent_quit();
        game().await;
    });
}
