// 게임 창으로 보낼 JSON 을 만드는 작은 도구 (std 만 쓰려고 직접 만든다).

pub struct Json {
    buf: String,
    kind: Kind,
}

#[derive(PartialEq)]
enum Kind {
    Object,
    Raw,
}

impl Json {
    pub fn object() -> Json {
        Json { buf: String::from("{"), kind: Kind::Object }
    }

    pub fn null() -> Json {
        Json { buf: String::from("null"), kind: Kind::Raw }
    }

    fn key(&mut self, key: &str) -> &mut Self {
        if self.buf.len() > 1 {
            self.buf.push(',');
        }
        push_str(&mut self.buf, key);
        self.buf.push(':');
        self
    }

    pub fn str(&mut self, key: &str, value: &str) -> &mut Self {
        self.key(key);
        push_str(&mut self.buf, value);
        self
    }

    pub fn num(&mut self, key: &str, value: impl std::fmt::Display) -> &mut Self {
        self.key(key);
        self.buf.push_str(&value.to_string());
        self
    }

    pub fn bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.key(key);
        self.buf.push_str(if value { "true" } else { "false" });
        self
    }

    pub fn raw(&mut self, key: &str, value: &str) -> &mut Self {
        self.key(key);
        self.buf.push_str(value);
        self
    }

    pub fn json(&mut self, key: &str, value: Json) -> &mut Self {
        self.key(key);
        self.buf.push_str(&value.finish());
        self
    }

    pub fn array(&mut self, key: &str, items: impl Iterator<Item = Json>) -> &mut Self {
        self.key(key);
        self.buf.push('[');
        for (i, item) in items.enumerate() {
            if i > 0 {
                self.buf.push(',');
            }
            self.buf.push_str(&item.finish());
        }
        self.buf.push(']');
        self
    }

    pub fn finish(mut self) -> String {
        if self.kind == Kind::Object {
            self.buf.push('}');
        }
        self.buf
    }
}

fn push_str(buf: &mut String, s: &str) {
    buf.push('"');
    for c in s.chars() {
        match c {
            '"' => buf.push_str("\\\""),
            '\\' => buf.push_str("\\\\"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            '\t' => buf.push_str("\\t"),
            c if (c as u32) < 0x20 => buf.push_str(&format!("\\u{:04x}", c as u32)),
            c => buf.push(c),
        }
    }
    buf.push('"');
}
