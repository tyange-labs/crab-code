// 정거장 복구 21 · 작전 지시문 — FromStr, 에러 타입, From
// 저장하면 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin briefing

use station::level21::*;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Go(Dir, u32),
    Grab,
    Insert,
}

#[derive(Debug)]
enum ParseError {
    Empty,
    UnknownCode(String),
    BadCount(ParseIntError),
}

/// `?` 가 ParseIntError 를 내 에러 타입으로 바꿔 주게 한다.
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        ParseError::BadCount(e)
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(token: &str) -> Result<Command, ParseError> {
        let mut chars = token.chars();
        let code = chars.next().ok_or(ParseError::Empty)?;
        let rest = chars.as_str();
        match code {
            'N' | 'E' | 'S' | 'W' => {
                let steps = if rest.is_empty() { 1 } else { rest.parse::<u32>()? };
                // code 로 방향을 골라 Ok(Command::Go(dir, steps)) 를 돌려주게 채워 보세요.
                let _ = steps;
                Err(ParseError::Empty)
            }
            'G' => Ok(Command::Grab),
            'I' => Ok(Command::Insert),
            _ => Err(ParseError::UnknownCode(token.to_string())),
        }
    }
}

#[derive(Debug)]
struct Echo {
    queue: VecDeque<Action>,
    loaded: bool,
}

impl Echo {
    /// 지시문을 읽어 실행 대기열을 만든다. 틀린 조각은 알려 주고 건너뛴다.
    fn load(&mut self, briefing: &str) {
        for token in briefing.split(' ') {
            match token.parse::<Command>() {
                Ok(Command::Go(dir, steps)) => {
                    for _ in 0..steps {
                        self.queue.push_back(Action::Move(dir));
                    }
                }
                Ok(Command::Grab) => self.queue.push_back(Action::Grab),
                Ok(Command::Insert) => self.queue.push_back(Action::Insert),
                Err(error) => println!("건너뜀: {token} ({error:?})"),
            }
        }
        self.loaded = true;
    }
}

impl Bot for Echo {
    fn new(_id: usize) -> Self {
        Echo { queue: VecDeque::new(), loaded: false }
    }

    fn tick(&mut self, view: &View) -> Action {
        if !self.loaded {
            self.load(view.briefing());
        }
        self.queue.pop_front().unwrap_or(Action::Wait)
    }
}

fn main() {
    play::<Echo>();
}
