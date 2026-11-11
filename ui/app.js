// Crab Code 화면.
// 백엔드(Rust)가 컴파일·실행·시뮬레이션을 맡고, 여기서는 결과를 그리고 에디터(Monaco)를 다룬다.
'use strict';

const invoke = window.__TAURI__.core.invoke;
const $ = (id) => document.getElementById(id);

const SPEEDS = [1, 2, 4];
const DX = [1, 0, -1, 0];
const DY = [0, 1, 0, -1];

const state = {
  chapters: [],
  lessons: [],
  dev: false,
  level: 0,
  codes: {},
  cleared: new Set(),
  world: null,
  phase: 'idle', // idle | compiling | playing | won | failed
  message: '',
  queue: [],
  outcome: null,
  anim: null,
  bump: 0,
  bumpForward: false,
  speedIdx: 0,
  particles: [],
  runToken: 0,
  checkToken: 0,
  confirmSolution: false,
  time: 0,
};

let editor = null;
let saveTimer = null;

// ── 게임 함수 목록 (자동 완성과 마우스 올렸을 때 설명에 쓴다) ──────────

const API = [
  ['move_forward', 'move_forward()', 'fn move_forward()', '앞으로 한 칸 이동합니다. 벽이나 잠긴 문에 부딪히면 프로그램이 멈춥니다.'],
  ['turn_left', 'turn_left()', 'fn turn_left()', '제자리에서 왼쪽으로 90도 돕니다.'],
  ['turn_right', 'turn_right()', 'fn turn_right()', '제자리에서 오른쪽으로 90도 돕니다.'],
  ['collect', 'collect()', 'fn collect() -> Gem', '발밑의 보석을 줍습니다. 주운 보석의 소유권을 돌려줍니다.'],
  ['deposit', 'deposit(${1:gem})', 'fn deposit(gem: Gem)', '발밑 받침대에 보석을 올립니다. 보석의 소유권을 가져갑니다.'],
  ['color_of', 'color_of(&${1:gem})', 'fn color_of(gem: &Gem) -> Color', '보석의 색을 확인합니다. 빌려서 보기만 합니다.'],
  ['polish', 'polish(&mut ${1:gem})', 'fn polish(gem: &mut Gem)', '보석에 광을 냅니다. 보석을 바꾸므로 가변 레퍼런스가 필요합니다.'],
  ['inspect', 'inspect(&${1:gem})', 'fn inspect(gem: &Gem) -> GemInfo', '보석을 살펴봅니다. `GemInfo { color, weight, shiny }` 를 돌려줍니다.'],
  ['unlock', 'unlock(${1:code})', 'fn unlock(code: u32)', '바로 앞의 문에 비밀번호를 입력합니다.'],
  ['front_is_clear', 'front_is_clear()', 'fn front_is_clear() -> bool', '앞 칸으로 갈 수 있으면 `true`.'],
  ['on_gem', 'on_gem()', 'fn on_gem() -> bool', '발밑에 보석이 있으면 `true`.'],
  ['read_sign', 'read_sign()', 'fn read_sign() -> Sign', '발밑의 표지판을 읽습니다. 표지판이 없으면 `Sign::Straight`.'],
  ['read_number', 'read_number()', 'fn read_number() -> Option<u32>', '발밑의 숫자 타일을 읽습니다. 숫자가 없으면 `None`.'],
  ['read_dial', 'read_dial()', 'fn read_dial() -> u32', '다이얼에 적힌 숫자를 읽습니다.'],
  ['read_scroll', 'read_scroll()', 'fn read_scroll() -> Vec<u32>', '두루마리에 적힌 숫자 목록을 읽습니다.'],
  ['read_note', 'read_note()', 'fn read_note() -> String', '쪽지에 적힌 글을 읽습니다.'],
  ['read_two_notes', 'read_two_notes()', 'fn read_two_notes() -> (String, String)', '쪽지 두 장을 한꺼번에 읽습니다.'],
  ['gem_offset', 'gem_offset()', 'fn gem_offset() -> (u32, u32)', '보석까지의 거리: (앞으로 몇 칸, 오른쪽으로 몇 칸).'],
];

const TYPES = [
  ['Color', 'enum Color { Red, Blue }', '보석의 색.'],
  ['Sign', 'enum Sign { Straight, Left, Right, Finish }', '바닥 표지판.'],
  ['Gem', 'struct Gem', '보석. `Copy` 도 `Clone` 도 아니라서 주인은 언제나 하나뿐입니다.'],
  ['GemInfo', 'struct GemInfo { color: Color, weight: u32, shiny: bool }', '`inspect` 가 돌려주는 보석 정보.'],
];

const VARIANTS = ['Color::Red', 'Color::Blue', 'Sign::Straight', 'Sign::Left', 'Sign::Right', 'Sign::Finish'];

const SNIPPETS = [
  ['for', 'for ${1:_} in 0..${2:3} {\n\t$0\n}', 'for 반복문'],
  ['while', 'while ${1:front_is_clear()} {\n\t$0\n}', 'while 반복문'],
  ['if', 'if ${1:on_gem()} {\n\t$0\n}', 'if'],
  ['iflet', 'if let Some(${1:n}) = ${2:read_number()} {\n\t$0\n}', 'if let'],
  ['match', 'match ${1:value} {\n\t${2:_} => $0,\n}', 'match'],
  ['fn', 'fn ${1:name}(${2}) {\n\t$0\n}', '함수 정의'],
  ['println', 'println!("${1:{}}", ${2:value});', '출력 창에 찍기'],
];

// ── 시작 ─────────────────────────────────────────────────────────

const monacoBase = new URL('vendor/monaco/', location.href).href;
window.MonacoEnvironment = {
  getWorkerUrl() {
    const src = `self.MonacoEnvironment = { baseUrl: '${monacoBase}' }; importScripts('${monacoBase}vs/base/worker/workerMain.js');`;
    return URL.createObjectURL(new Blob([src], { type: 'text/javascript' }));
  },
};
require.config({ paths: { vs: 'vendor/monaco/vs' }, 'vs/nls': { availableLanguages: { '*': 'ko' } } });
require(['vs/editor/editor.main'], () => {
  boot().catch((e) => {
    document.body.insertAdjacentHTML('beforeend', `<pre style="position:fixed;left:16px;bottom:16px;color:#ff6b7a">${escapeHtml(String(e && e.stack || e))}</pre>`);
  });
});

async function boot() {
  const [data, save] = await Promise.all([invoke('get_lessons'), invoke('load_save')]);
  state.chapters = data.chapters;
  state.lessons = data.lessons;
  state.dev = data.dev;
  if (!state.dev) {
    state.codes = save.codes;
    state.cleared = new Set(save.cleared);
  }

  // 글꼴이 준비된 뒤에 에디터를 만들어야 글자 폭 계산이 맞는다.
  await Promise.all([document.fonts.load('14px "JetBrains Mono"'), document.fonts.load('15px "Pretendard"')]);
  setupMonaco();
  drawLogo();

  $('run').onclick = run;
  $('reset').onclick = resetCode;
  $('hint').onclick = () => log(`힌트: ${lesson().hint}`, 'warn text');
  $('solution').onclick = showSolution;
  $('speed').onclick = () => {
    state.speedIdx = (state.speedIdx + 1) % SPEEDS.length;
    $('speed').textContent = `속도 ${SPEEDS[state.speedIdx]}x`;
  };
  window.addEventListener('keydown', (e) => {
    if (e.key === 'F5' || ((e.metaKey || e.ctrlKey) && e.key === 'Enter')) {
      e.preventDefault();
      run();
    }
  });
  window.addEventListener('beforeunload', saveNow);

  let first = state.lessons.findIndex((l) => !state.cleared.has(l.id));
  if (data.startLevel) {
    first = Math.min(Math.max(data.startLevel, 1), state.lessons.length) - 1;
  }
  await setLevel(Math.max(first, 0));
  if (data.demo) {
    if (data.demo !== 'start') {
      editor.setValue(lesson().solution);
    }
    run();
  }
  requestAnimationFrame(tick);
}

function setupMonaco() {
  monaco.editor.defineTheme('crab', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'keyword', foreground: 'ff8f5a' },
      { token: 'keyword.type', foreground: '6fe0c0' },
      { token: 'type.identifier', foreground: '6fe0c0' },
      { token: 'comment', foreground: '717791' },
      { token: 'string', foreground: 'b5e07a' },
      { token: 'number', foreground: 'd9a8ff' },
      { token: 'identifier', foreground: 'e4e6f0' },
      { token: 'delimiter', foreground: 'b4b9d0' },
      { token: 'operator', foreground: 'b4b9d0' },
    ],
    colors: {
      'editor.background': '#1b1d2c',
      'editor.foreground': '#e4e6f0',
      'editor.lineHighlightBackground': '#24273a',
      'editor.lineHighlightBorder': '#00000000',
      'editorLineNumber.foreground': '#5d627a',
      'editorLineNumber.activeForeground': '#e4e6f0',
      'editor.selectionBackground': '#2c4a7c',
      'editor.inactiveSelectionBackground': '#2c4a7c88',
      'editorIndentGuide.background1': '#2d3150',
      'editorIndentGuide.activeBackground1': '#4a5080',
      'editorCursor.foreground': '#e4e6f0',
      'editorWidget.background': '#23263b',
      'editorWidget.border': '#454a70',
      'editorSuggestWidget.background': '#23263b',
      'editorSuggestWidget.border': '#454a70',
      'editorSuggestWidget.selectedBackground': '#2f4a7a',
      'editorHoverWidget.background': '#23263b',
      'editorHoverWidget.border': '#454a70',
      'editorError.foreground': '#ff6b7a',
      'editorWarning.foreground': '#f2c94c',
      'editorBracketMatch.background': '#3a3f5c',
      'editorBracketMatch.border': '#6b7299',
      'scrollbarSlider.background': '#3a3f5c99',
      'scrollbarSlider.hoverBackground': '#4a5080',
    },
  });

  const Kind = monaco.languages.CompletionItemKind;
  const asSnippet = monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet;
  monaco.languages.registerCompletionItemProvider('rust', {
    provideCompletionItems(model, position) {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };
      const suggestions = [];
      for (const [name, snippet, sig, doc] of API) {
        suggestions.push({ label: name, kind: Kind.Function, insertText: snippet, insertTextRules: asSnippet, detail: sig, documentation: { value: doc }, range, sortText: `0${name}` });
      }
      for (const [name, sig, doc] of TYPES) {
        suggestions.push({ label: name, kind: Kind.Struct, insertText: name, detail: sig, documentation: { value: doc }, range, sortText: `1${name}` });
      }
      for (const name of VARIANTS) {
        suggestions.push({ label: name, kind: Kind.EnumMember, insertText: name, range, sortText: `1${name}` });
      }
      for (const [name, snippet, doc] of SNIPPETS) {
        suggestions.push({ label: name, kind: Kind.Snippet, insertText: snippet, insertTextRules: asSnippet, detail: doc, range, sortText: `2${name}` });
      }
      return { suggestions };
    },
  });
  monaco.languages.registerHoverProvider('rust', {
    provideHover(model, position) {
      const word = model.getWordAtPosition(position);
      if (!word) return null;
      const hit = API.find((a) => a[0] === word.word);
      const type = TYPES.find((t) => t[0] === word.word);
      const sig = hit ? hit[2] : type ? type[1] : null;
      if (!sig) return null;
      return {
        range: new monaco.Range(position.lineNumber, word.startColumn, position.lineNumber, word.endColumn),
        contents: [{ value: '```rust\n' + sig + '\n```' }, { value: hit ? hit[3] : type[2] }],
      };
    },
  });

  editor = monaco.editor.create($('editor'), {
    value: '',
    language: 'rust',
    theme: 'crab',
    fontFamily: '"JetBrains Mono", "Pretendard", monospace',
    fontSize: 14,
    lineHeight: 22,
    fontLigatures: true,
    tabSize: 4,
    insertSpaces: true,
    detectIndentation: false,
    minimap: { enabled: false },
    automaticLayout: true,
    scrollBeyondLastLine: false,
    padding: { top: 8, bottom: 8 },
    renderLineHighlight: 'all',
    bracketPairColorization: { enabled: true },
    guides: { indentation: true, bracketPairs: false },
    smoothScrolling: true,
    cursorBlinking: 'smooth',
    fixedOverflowWidgets: true,
    lineNumbersMinChars: 3,
    scrollbar: { verticalScrollbarSize: 9, horizontalScrollbarSize: 9 },
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    wordBasedSuggestions: 'currentDocument',
    stickyScroll: { enabled: false },
  });
  monaco.editor.remeasureFonts();
  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, run);
  editor.addCommand(monaco.KeyCode.F5, run);

  editor.onDidChangeModelContent(() => {
    state.confirmSolution = false;
    $('solution').textContent = '정답 보기';
    clearTimeout(saveTimer);
    // 입력이 멈추면 저장하고, 컴파일 에러를 확인해 밑줄을 긋는다.
    saveTimer = setTimeout(() => {
      saveNow();
      checkCode();
    }, 600);
  });
}

// ── 레슨과 저장 ───────────────────────────────────────────────────

function lesson() {
  return state.lessons[state.level];
}

function saveNow() {
  if (!editor) return;
  const id = lesson().id;
  const code = editor.getValue();
  state.codes[id] = code;
  if (!state.dev) {
    invoke('save_code', { id, code });
  }
}

async function checkCode() {
  const token = ++state.checkToken;
  const version = editor.getModel().getVersionId();
  const diagnostics = await invoke('check_code', { code: editor.getValue() });
  if (token === state.checkToken && version === editor.getModel().getVersionId()) {
    setMarkers(diagnostics);
  }
}

function setMarkers(diagnostics) {
  const markers = diagnostics.map((d) => ({
    startLineNumber: d.line,
    startColumn: d.col,
    endLineNumber: d.endLine,
    endColumn: d.endCol,
    message: d.message,
    severity: d.error ? monaco.MarkerSeverity.Error : monaco.MarkerSeverity.Warning,
    source: 'rustc',
  }));
  monaco.editor.setModelMarkers(editor.getModel(), 'rustc', markers);
}

function randomSeed() {
  return Math.floor(Math.random() * 0xffffffff) + 1;
}

async function setLevel(index) {
  if (state.world) {
    clearTimeout(saveTimer);
    saveNow();
  }
  state.level = index;
  state.runToken++;
  state.checkToken++;
  const l = lesson();
  editor.setValue(state.codes[l.id] ?? l.start);
  setMarkers([]);
  state.confirmSolution = false;
  $('solution').textContent = '정답 보기';
  await loadIdleWorld();
  intro();
  renderLesson();
  $('lesson').scrollTop = 0;
  editor.setScrollTop(0);
  editor.focus();
  checkCode();
}

async function loadIdleWorld() {
  const level = state.level;
  const world = await invoke('get_world', { level, seed: randomSeed() });
  if (level !== state.level) return;
  state.world = world;
  state.queue = [];
  state.anim = null;
  state.bump = 0;
  state.particles = [];
  setPhase('idle');
  renderWorldInfo();
}

function escapeHtml(s) {
  return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));
}

/** `백틱` 을 코드 조각으로 바꾼다. */
function rich(text) {
  return escapeHtml(text)
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replace(/^목표:/, '<span class="goal-label">목표:</span>');
}

function chip(label, { current, done }, onClick) {
  const b = document.createElement('button');
  b.className = `chip${current ? ' current' : done ? ' done' : ''}`;
  b.textContent = label;
  b.onclick = onClick;
  return b;
}

function renderLesson() {
  const l = lesson();
  const inChapter = (c) => state.lessons.map((x, i) => i).filter((i) => state.lessons[i].chapter === c);

  const chapters = $('chapters');
  chapters.replaceChildren();
  state.chapters.forEach((ch, c) => {
    const levels = inChapter(c);
    const done = levels.every((i) => state.cleared.has(state.lessons[i].id));
    chapters.append(
      chip(ch.short, { current: c === l.chapter, done }, () => {
        if (c === lesson().chapter) return;
        // 그 장에서 아직 깨지 않은 첫 레벨로 간다.
        const next = levels.find((i) => !state.cleared.has(state.lessons[i].id)) ?? levels[0];
        setLevel(next);
      }),
    );
  });

  const levels = inChapter(l.chapter);
  const nav = $('levels');
  nav.replaceChildren();
  levels.forEach((i, n) => {
    nav.append(
      chip(String(n + 1), { current: i === state.level, done: state.cleared.has(state.lessons[i].id) }, () => {
        if (i !== state.level) setLevel(i);
      }),
    );
  });

  $('chapter-name').textContent = state.chapters[l.chapter].name;
  $('lesson-title').textContent = `${levels.indexOf(state.level) + 1}. ${l.title}`;
  $('lesson-body').innerHTML = l.body
    .split('\n\n')
    .map((para) => `<p class="${para.startsWith('목표:') ? 'goal' : ''}">${para.split('\n').map(rich).join('<br>')}</p>`)
    .join('');
}

function goalHtml() {
  const goal = lesson().body.split('\n').find((line) => line.startsWith('목표:'));
  return goal ? rich(goal) : '';
}

// ── 출력 창 ──────────────────────────────────────────────────────

function log(text, tone = '') {
  const box = $('console');
  const div = document.createElement('div');
  div.className = `line ${tone}`;
  div.textContent = text;
  box.append(div);
  box.scrollTop = box.scrollHeight;
}

function clearConsole() {
  $('console').replaceChildren();
}

function intro() {
  clearConsole();
  log('실행 버튼(F5 또는 Cmd+Enter)을 누르면 코드를 진짜 rustc 로 컴파일해서 돌려요.', 'dim text');
  log('println! 출력과 컴파일러 메시지가 여기에 나옵니다.', 'dim text');
}

function logCompiler(output, warningsOnly) {
  for (const line of output.split('\n')) {
    const t = line.trimStart();
    let tone = '';
    if (t.startsWith('error')) tone = 'bad';
    else if (t.startsWith('warning')) tone = 'warn';
    else if (t.startsWith('help') || t.startsWith('= help')) tone = 'good';
    else if (t.startsWith('-->') || t.startsWith('= note') || t.startsWith('note')) tone = 'dim';
    if (warningsOnly && tone !== 'warn') tone = 'dim';
    log(line, tone);
  }
}

// ── 실행 ─────────────────────────────────────────────────────────

function setPhase(phase, message = '') {
  state.phase = phase;
  state.message = message;
  const banner = $('banner');
  banner.className = '';
  $('run').textContent = phase === 'compiling' ? '컴파일 중...' : '실행 (F5)';
  $('run').disabled = phase === 'compiling';
  if (phase === 'failed') {
    banner.className = 'fail';
    banner.textContent = message;
  } else if (phase === 'won') {
    banner.className = 'win';
    const last = state.level + 1 >= state.lessons.length;
    banner.textContent = last ? '졸업을 축하해요! 이제 진짜 Rust 를 쓸 준비가 됐어요.' : '성공! 잘했어요.';
    if (!last) {
      const next = document.createElement('button');
      next.textContent = '다음 레벨';
      next.onclick = () => setLevel(state.level + 1);
      banner.append(next);
    }
  } else if (phase === 'compiling') {
    banner.className = 'busy';
    banner.textContent = 'rustc 가 코드를 컴파일하는 중...';
  } else {
    // 목표는 본문을 스크롤하지 않아도 항상 보이게 여기에 고정한다.
    banner.innerHTML = `<span>${goalHtml()}</span>`;
  }
}

async function run() {
  if (state.phase === 'compiling' || !editor) return;
  const token = ++state.runToken;
  const level = state.level;
  clearTimeout(saveTimer);
  saveNow();
  state.queue = [];
  state.anim = null;
  state.particles = [];
  setPhase('compiling');
  clearConsole();
  log('컴파일 중...', 'dim text');

  const result = await invoke('run_code', { code: editor.getValue(), level, seed: randomSeed() });
  if (token !== state.runToken || level !== state.level) return;
  clearConsole();
  state.checkToken++;
  setMarkers(result.diagnostics);
  state.world = result.world;
  renderWorldInfo();

  if (!result.compiled) {
    logCompiler(result.output, false);
    $('console').scrollTop = 0;
    const first = result.diagnostics.find((d) => d.error);
    if (first) {
      editor.revealLineInCenterIfOutsideViewport(first.line);
    }
    setPhase('failed', '컴파일 에러! 아래 컴파일러 메시지를 읽어 보세요.');
    return;
  }
  if (result.output) {
    logCompiler(result.output, true);
  }
  state.queue = result.steps;
  state.outcome = { won: result.won, message: result.message };
  setPhase('playing');
}

function finishRun() {
  const { won, message } = state.outcome;
  if (!won) {
    log(message, 'bad text');
    setPhase('failed', message);
    return;
  }
  log(message, 'good text');
  state.cleared.add(lesson().id);
  if (!state.dev) {
    invoke('save_cleared', { ids: [...state.cleared] });
  }
  setPhase('won');
  renderLesson();
  const colors = ['#f46a25', '#5fd38d', '#5cb8ff', '#f2c94c', '#ff5c7a'];
  for (let i = 0; i < 90; i++) {
    const a = i * 2.399;
    const speed = 120 + ((i * 37) % 100) * 2.6;
    state.particles.push({ x: 0, y: 0, vx: Math.cos(a) * speed, vy: Math.sin(a) * speed - 180, life: 1.2 + ((i * 13) % 10) * 0.08, color: colors[i % 5] });
  }
}

function advance(dt) {
  if (state.anim) {
    const base = state.anim.action === 'forward' ? 0.24 : state.anim.action === 'left' || state.anim.action === 'right' ? 0.18 : 0.22;
    state.anim.t += (dt * SPEEDS[state.speedIdx]) / base;
    if (state.anim.t < 1) return;
    state.anim = null;
  }
  for (;;) {
    const step = state.queue.shift();
    if (!step) {
      finishRun();
      return;
    }
    if (step.type === 'print') {
      log(step.text);
    } else if (step.type === 'stderr') {
      log(step.text, 'bad');
    } else if (step.type === 'bump') {
      state.bump = 0.3;
      state.bumpForward = step.forward;
    } else {
      state.anim = { fromX: state.world.x, fromY: state.world.y, fromDir: state.world.dir, t: 0, action: step.action };
      state.world = step.world;
      renderWorldInfo();
      return;
    }
  }
}

function resetCode() {
  replaceCode(lesson().start);
  state.runToken++;
  loadIdleWorld();
  intro();
}

function showSolution() {
  if (!state.confirmSolution) {
    state.confirmSolution = true;
    $('solution').textContent = '정말 볼까요?';
    return;
  }
  replaceCode(lesson().solution);
}

/** 내용을 통째로 바꾸되, Cmd+Z 로 되돌릴 수 있게 한다. */
function replaceCode(text) {
  const model = editor.getModel();
  editor.pushUndoStop();
  editor.executeEdits('crab', [{ range: model.getFullModelRange(), text }]);
  editor.pushUndoStop();
  editor.setPosition({ lineNumber: 1, column: 1 });
  editor.focus();
}

// ── 월드 그리기 ───────────────────────────────────────────────────

const GEM = { red: '#ff5c7a', blue: '#4fb3ff' };

function mixHex(a, b, t) {
  const pa = [1, 3, 5].map((i) => parseInt(a.slice(i, i + 2), 16));
  const pb = [1, 3, 5].map((i) => parseInt(b.slice(i, i + 2), 16));
  return `rgb(${pa.map((v, i) => Math.round(v + (pb[i] - v) * t)).join(',')})`;
}

function gemColor(g) {
  return g.shiny ? GEM[g.color] : mixHex(GEM[g.color], '#666a7c', 0.72);
}

function renderWorldInfo() {
  const w = state.world;
  const bag = $('bag');
  bag.replaceChildren('가방');
  const held = w.gems.filter((g) => g.held);
  if (held.length === 0) {
    const empty = document.createElement('span');
    empty.className = 'empty';
    empty.textContent = '비어 있음';
    bag.append(empty);
  }
  for (const g of held) {
    const icon = document.createElement('span');
    icon.className = 'gem-icon';
    icon.style.background = gemColor(g);
    bag.append(icon);
  }

  const items = [];
  if (w.dial != null) items.push(['다이얼', String(w.dial)]);
  if (w.scroll.length) items.push(['두루마리', `[${w.scroll.join(', ')}]`]);
  if (w.note != null) items.push(['쪽지', JSON.stringify(w.note)]);
  if (w.note2 != null) items.push(['쪽지 2', JSON.stringify(w.note2)]);
  $('info').innerHTML = items.map(([label, value]) => `<span>${label}<code>${escapeHtml(value)}</code></span>`).join('');
}

function rr(ctx, x, y, w, h, r, color) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.roundRect(x, y, w, h, r);
  ctx.fill();
}

function diamond(ctx, x, y, r, color) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.moveTo(x + r, y);
  ctx.lineTo(x, y + r);
  ctx.lineTo(x - r, y);
  ctx.lineTo(x, y - r);
  ctx.closePath();
  ctx.fill();
}

function circle(ctx, x, y, r, color) {
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.arc(x, y, r, 0, Math.PI * 2);
  ctx.fill();
}

function drawGem(ctx, x, y, r, g) {
  const c = gemColor(g);
  diamond(ctx, x, y, r, mixHex(GEM[g.color], '#000000', g.shiny ? 0.25 : 0.6));
  diamond(ctx, x, y, r * 0.84, c);
  if (g.shiny) {
    diamond(ctx, x - r * 0.12, y - r * 0.18, r * 0.36, mixHex(GEM[g.color], '#ffffff', 0.55));
  }
}

/** 꽃게. 오른쪽(+x)을 보는 모양을 angle 만큼 돌려 그린다. */
function drawCrab(ctx, cx, cy, s, angle, walk) {
  const shell = '#f4511e';
  const dark = '#b93a12';
  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(angle);
  ctx.scale(s, s);
  ctx.lineCap = 'round';
  ctx.strokeStyle = dark;
  for (const side of [-1, 1]) {
    ctx.lineWidth = 0.05;
    for (let i = 0; i < 3; i++) {
      const fx = -0.17 + i * 0.12;
      const wiggle = Math.sin(walk * Math.PI * 4 + i * 2) * 0.035;
      ctx.beginPath();
      ctx.moveTo(fx, side * 0.18);
      ctx.lineTo(fx - 0.03 + wiggle, side * 0.41);
      ctx.stroke();
    }
    ctx.lineWidth = 0.06;
    ctx.beginPath();
    ctx.moveTo(0.1, side * 0.18);
    ctx.lineTo(0.3, side * 0.3);
    ctx.stroke();
    circle(ctx, 0.3, side * 0.3, 0.105, shell);
    circle(ctx, 0.385, side * 0.27, 0.04, dark);
  }
  circle(ctx, 0, 0, 0.275, dark);
  circle(ctx, 0, 0, 0.245, shell);
  for (const side of [-1, 1]) {
    circle(ctx, 0.16, side * 0.105, 0.078, '#ffffff');
    circle(ctx, 0.19, side * 0.105, 0.04, '#1a1a22');
  }
  ctx.restore();
}

function drawLogo() {
  const ctx = $('logo').getContext('2d');
  drawCrab(ctx, 44, 46, 80, -Math.PI / 2, 0);
}

function drawWorld() {
  const cv = $('world');
  const wrap = cv.parentElement;
  const dpr = window.devicePixelRatio || 1;
  const W = wrap.clientWidth;
  const H = wrap.clientHeight;
  if (cv.width !== Math.round(W * dpr) || cv.height !== Math.round(H * dpr)) {
    cv.width = Math.round(W * dpr);
    cv.height = Math.round(H * dpr);
  }
  const ctx = cv.getContext('2d');
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, W, H);
  const w = state.world;
  if (!w || W < 10 || H < 10) return;

  // 바닥이 있는 범위만 화면에 맞춘다.
  const wall = (x, y) => x < 0 || y < 0 || x >= w.w || y >= w.h || w.walls[y * w.w + x];
  let [x0, y0, x1, y1] = [w.w, w.h, 0, 0];
  for (let y = 0; y < w.h; y++) {
    for (let x = 0; x < w.w; x++) {
      if (!wall(x, y)) {
        x0 = Math.min(x0, x);
        y0 = Math.min(y0, y);
        x1 = Math.max(x1, x);
        y1 = Math.max(y1, y);
      }
    }
  }
  const cols = x1 - x0 + 1;
  const rows = y1 - y0 + 1;
  const tile = Math.min((W - 24) / cols, (H - 20) / rows, 92);
  const ox = (W - tile * cols) / 2 - x0 * tile;
  const oy = (H - tile * rows) / 2 - y0 * tile;
  const cx = (x) => ox + (x + 0.5) * tile;
  const cy = (y) => oy + (y + 0.5) * tile;
  const label = (text, x, y, size, color, font = 'Pretendard') => {
    ctx.fillStyle = color;
    ctx.font = `${size}px "${font}"`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(text, x, y);
  };

  for (let y = 0; y < w.h; y++) {
    for (let x = 0; x < w.w; x++) {
      if (!wall(x, y)) {
        rr(ctx, ox + x * tile + 2, oy + y * tile + 2, tile - 4, tile - 4, tile * 0.14, (x + y) % 2 === 0 ? '#363b58' : '#323651');
      }
    }
  }

  for (const n of w.numbers) {
    rr(ctx, cx(n.x) - tile * 0.3, cy(n.y) - tile * 0.3, tile * 0.6, tile * 0.6, tile * 0.1, '#27405e');
    label(String(n.n), cx(n.x), cy(n.y) + tile * 0.02, tile * 0.4, '#5cb8ff', 'JetBrains Mono');
  }

  for (const s of w.signs) {
    if (s.sign !== 'left' && s.sign !== 'right') continue;
    rr(ctx, cx(s.x) - tile * 0.4, cy(s.y) - tile * 0.2, tile * 0.8, tile * 0.4, tile * 0.08, '#f2c94c');
    label(s.sign === 'left' ? '왼쪽' : '오른쪽', cx(s.x), cy(s.y) + tile * 0.01, tile * 0.23, '#3a2c00');
  }

  if (w.flag) {
    const px = cx(w.flag.x) - tile * 0.16;
    const fy = cy(w.flag.y);
    ctx.strokeStyle = '#e4e6f0';
    ctx.lineWidth = tile * 0.05;
    ctx.lineCap = 'round';
    ctx.beginPath();
    ctx.moveTo(px, fy - tile * 0.32);
    ctx.lineTo(px, fy + tile * 0.32);
    ctx.stroke();
    const wave = Math.sin(state.time * 4) * tile * 0.03;
    ctx.fillStyle = '#5fd38d';
    ctx.beginPath();
    ctx.moveTo(px, fy - tile * 0.32);
    ctx.lineTo(px + tile * 0.4, fy - tile * 0.17 + wave);
    ctx.lineTo(px, fy - tile * 0.02);
    ctx.closePath();
    ctx.fill();
  }

  for (const p of w.pedestals) {
    const ring = p.want ? GEM[p.want] : '#c9cde0';
    rr(ctx, cx(p.x) - tile * 0.36, cy(p.y) - tile * 0.36, tile * 0.72, tile * 0.72, tile * 0.12, mixHex(ring, '#000000', 0.2));
    rr(ctx, cx(p.x) - tile * 0.29, cy(p.y) - tile * 0.29, tile * 0.58, tile * 0.58, tile * 0.09, '#23263a');
  }

  if (w.door) {
    const dx = cx(w.door.x);
    const dy = cy(w.door.y);
    if (w.door.open) {
      ctx.strokeStyle = '#8a5a2b';
      ctx.lineWidth = tile * 0.07;
      ctx.strokeRect(dx - tile * 0.38, dy - tile * 0.38, tile * 0.76, tile * 0.76);
    } else {
      rr(ctx, dx - tile * 0.42, dy - tile * 0.42, tile * 0.84, tile * 0.84, tile * 0.1, '#8a5a2b');
      rr(ctx, dx - tile * 0.34, dy - tile * 0.34, tile * 0.68, tile * 0.68, tile * 0.07, '#a9713a');
      circle(ctx, dx, dy - tile * 0.04, tile * 0.11, '#f2c94c');
      ctx.fillRect(dx - tile * 0.03, dy, tile * 0.06, tile * 0.16);
    }
  }

  for (const g of w.gems) {
    if (g.held) continue;
    const bob = g.placed ? 0 : Math.sin(state.time * 2.6 + g.x) * tile * 0.03;
    drawGem(ctx, cx(g.x), cy(g.y) + bob, tile * 0.24, g);
    if (w.showWeights) {
      label(String(g.weight), cx(g.x), cy(g.y) + bob + tile * 0.01, tile * 0.2, '#1a1a22', 'JetBrains Mono');
    }
  }

  // 꽃게
  let fx = w.x;
  let fy = w.y;
  let angle = (w.dir * Math.PI) / 2;
  let walk = 0;
  const anim = state.anim;
  if (anim) {
    const t = Math.min(Math.max(anim.t, 0), 1);
    const e = t * t * (3 - 2 * t);
    fx = anim.fromX + (w.x - anim.fromX) * e;
    fy = anim.fromY + (w.y - anim.fromY) * e;
    const delta = ((w.dir - anim.fromDir + 6) % 4) - 2;
    angle = ((anim.fromDir + delta * e) * Math.PI) / 2;
    if (anim.action === 'forward') walk = t;
  }
  if (state.bump > 0 && state.bumpForward) {
    const k = Math.sin(Math.PI * (1 - state.bump / 0.3)) * 0.22;
    fx += DX[w.dir] * k;
    fy += DY[w.dir] * k;
  }
  drawCrab(ctx, cx(fx), cy(fy), tile, angle, walk);

  if (anim) {
    const ring = { collect: '#f2c94c', deposit: '#5fd38d', polish: '#ffffff', unlock: '#5cb8ff' }[anim.action];
    if (ring) {
      const t = Math.min(Math.max(anim.t, 0), 1);
      ctx.globalAlpha = 1 - t;
      ctx.strokeStyle = ring;
      ctx.lineWidth = tile * 0.05;
      ctx.beginPath();
      ctx.arc(cx(fx), cy(fy), tile * (0.3 + 0.35 * t), 0, Math.PI * 2);
      ctx.stroke();
      ctx.globalAlpha = 1;
    }
  } else {
    // 꽃게가 받침대 위에 서 있으면 올려 둔 보석이 가려지므로 위에 다시 그린다.
    for (const g of w.gems) {
      if (g.placed && g.x === w.x && g.y === w.y) {
        drawGem(ctx, cx(fx), cy(fy) - tile * 0.3, tile * 0.17, g);
      }
    }
  }

  // 축하 색종이
  for (const p of state.particles) {
    const size = 7 * Math.min(Math.max(p.life, 0), 1);
    ctx.fillStyle = p.color;
    ctx.fillRect(W / 2 + p.x, H * 0.45 + p.y, size, size);
  }
}

let lastTime = 0;
function tick(now) {
  const dt = Math.min((now - lastTime) / 1000 || 0, 0.05);
  lastTime = now;
  state.time += dt;
  if (state.phase === 'playing') {
    advance(dt);
  }
  state.bump = Math.max(state.bump - dt, 0);
  for (const p of state.particles) {
    p.vy += 520 * dt;
    p.x += p.vx * dt;
    p.y += p.vy * dt;
    p.life -= dt;
  }
  state.particles = state.particles.filter((p) => p.life > 0);
  drawWorld();
  requestAnimationFrame(tick);
}
