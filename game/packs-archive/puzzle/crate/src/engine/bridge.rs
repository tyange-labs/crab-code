// 엔진의 Color·Sign 을 레벨 모듈의 것으로 바꾼다.

use super::world as w;

impl From<w::Color> for crate::level16::Color {
    fn from(c: w::Color) -> crate::level16::Color {
        match c {
            w::Color::Red => crate::level16::Color::Red,
            w::Color::Blue => crate::level16::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level17::Color {
    fn from(c: w::Color) -> crate::level17::Color {
        match c {
            w::Color::Red => crate::level17::Color::Red,
            w::Color::Blue => crate::level17::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level18::Color {
    fn from(c: w::Color) -> crate::level18::Color {
        match c {
            w::Color::Red => crate::level18::Color::Red,
            w::Color::Blue => crate::level18::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level19::Color {
    fn from(c: w::Color) -> crate::level19::Color {
        match c {
            w::Color::Red => crate::level19::Color::Red,
            w::Color::Blue => crate::level19::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level20::Color {
    fn from(c: w::Color) -> crate::level20::Color {
        match c {
            w::Color::Red => crate::level20::Color::Red,
            w::Color::Blue => crate::level20::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level21::Color {
    fn from(c: w::Color) -> crate::level21::Color {
        match c {
            w::Color::Red => crate::level21::Color::Red,
            w::Color::Blue => crate::level21::Color::Blue,
        }
    }
}
impl From<w::Color> for crate::level22::Color {
    fn from(c: w::Color) -> crate::level22::Color {
        match c {
            w::Color::Red => crate::level22::Color::Red,
            w::Color::Blue => crate::level22::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level22::Sign {
    fn from(s: w::Sign) -> crate::level22::Sign {
        match s {
            w::Sign::Straight => crate::level22::Sign::Straight,
            w::Sign::Left => crate::level22::Sign::Left,
            w::Sign::Right => crate::level22::Sign::Right,
            w::Sign::Finish => crate::level22::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level23::Color {
    fn from(c: w::Color) -> crate::level23::Color {
        match c {
            w::Color::Red => crate::level23::Color::Red,
            w::Color::Blue => crate::level23::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level23::Sign {
    fn from(s: w::Sign) -> crate::level23::Sign {
        match s {
            w::Sign::Straight => crate::level23::Sign::Straight,
            w::Sign::Left => crate::level23::Sign::Left,
            w::Sign::Right => crate::level23::Sign::Right,
            w::Sign::Finish => crate::level23::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level24::Color {
    fn from(c: w::Color) -> crate::level24::Color {
        match c {
            w::Color::Red => crate::level24::Color::Red,
            w::Color::Blue => crate::level24::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level24::Sign {
    fn from(s: w::Sign) -> crate::level24::Sign {
        match s {
            w::Sign::Straight => crate::level24::Sign::Straight,
            w::Sign::Left => crate::level24::Sign::Left,
            w::Sign::Right => crate::level24::Sign::Right,
            w::Sign::Finish => crate::level24::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level25::Color {
    fn from(c: w::Color) -> crate::level25::Color {
        match c {
            w::Color::Red => crate::level25::Color::Red,
            w::Color::Blue => crate::level25::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level25::Sign {
    fn from(s: w::Sign) -> crate::level25::Sign {
        match s {
            w::Sign::Straight => crate::level25::Sign::Straight,
            w::Sign::Left => crate::level25::Sign::Left,
            w::Sign::Right => crate::level25::Sign::Right,
            w::Sign::Finish => crate::level25::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level26::Color {
    fn from(c: w::Color) -> crate::level26::Color {
        match c {
            w::Color::Red => crate::level26::Color::Red,
            w::Color::Blue => crate::level26::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level26::Sign {
    fn from(s: w::Sign) -> crate::level26::Sign {
        match s {
            w::Sign::Straight => crate::level26::Sign::Straight,
            w::Sign::Left => crate::level26::Sign::Left,
            w::Sign::Right => crate::level26::Sign::Right,
            w::Sign::Finish => crate::level26::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level27::Color {
    fn from(c: w::Color) -> crate::level27::Color {
        match c {
            w::Color::Red => crate::level27::Color::Red,
            w::Color::Blue => crate::level27::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level27::Sign {
    fn from(s: w::Sign) -> crate::level27::Sign {
        match s {
            w::Sign::Straight => crate::level27::Sign::Straight,
            w::Sign::Left => crate::level27::Sign::Left,
            w::Sign::Right => crate::level27::Sign::Right,
            w::Sign::Finish => crate::level27::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level28::Color {
    fn from(c: w::Color) -> crate::level28::Color {
        match c {
            w::Color::Red => crate::level28::Color::Red,
            w::Color::Blue => crate::level28::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level28::Sign {
    fn from(s: w::Sign) -> crate::level28::Sign {
        match s {
            w::Sign::Straight => crate::level28::Sign::Straight,
            w::Sign::Left => crate::level28::Sign::Left,
            w::Sign::Right => crate::level28::Sign::Right,
            w::Sign::Finish => crate::level28::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level29::Color {
    fn from(c: w::Color) -> crate::level29::Color {
        match c {
            w::Color::Red => crate::level29::Color::Red,
            w::Color::Blue => crate::level29::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level29::Sign {
    fn from(s: w::Sign) -> crate::level29::Sign {
        match s {
            w::Sign::Straight => crate::level29::Sign::Straight,
            w::Sign::Left => crate::level29::Sign::Left,
            w::Sign::Right => crate::level29::Sign::Right,
            w::Sign::Finish => crate::level29::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level30::Color {
    fn from(c: w::Color) -> crate::level30::Color {
        match c {
            w::Color::Red => crate::level30::Color::Red,
            w::Color::Blue => crate::level30::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level30::Sign {
    fn from(s: w::Sign) -> crate::level30::Sign {
        match s {
            w::Sign::Straight => crate::level30::Sign::Straight,
            w::Sign::Left => crate::level30::Sign::Left,
            w::Sign::Right => crate::level30::Sign::Right,
            w::Sign::Finish => crate::level30::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level31::Color {
    fn from(c: w::Color) -> crate::level31::Color {
        match c {
            w::Color::Red => crate::level31::Color::Red,
            w::Color::Blue => crate::level31::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level31::Sign {
    fn from(s: w::Sign) -> crate::level31::Sign {
        match s {
            w::Sign::Straight => crate::level31::Sign::Straight,
            w::Sign::Left => crate::level31::Sign::Left,
            w::Sign::Right => crate::level31::Sign::Right,
            w::Sign::Finish => crate::level31::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level32::Color {
    fn from(c: w::Color) -> crate::level32::Color {
        match c {
            w::Color::Red => crate::level32::Color::Red,
            w::Color::Blue => crate::level32::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level32::Sign {
    fn from(s: w::Sign) -> crate::level32::Sign {
        match s {
            w::Sign::Straight => crate::level32::Sign::Straight,
            w::Sign::Left => crate::level32::Sign::Left,
            w::Sign::Right => crate::level32::Sign::Right,
            w::Sign::Finish => crate::level32::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level33::Color {
    fn from(c: w::Color) -> crate::level33::Color {
        match c {
            w::Color::Red => crate::level33::Color::Red,
            w::Color::Blue => crate::level33::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level33::Sign {
    fn from(s: w::Sign) -> crate::level33::Sign {
        match s {
            w::Sign::Straight => crate::level33::Sign::Straight,
            w::Sign::Left => crate::level33::Sign::Left,
            w::Sign::Right => crate::level33::Sign::Right,
            w::Sign::Finish => crate::level33::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level34::Color {
    fn from(c: w::Color) -> crate::level34::Color {
        match c {
            w::Color::Red => crate::level34::Color::Red,
            w::Color::Blue => crate::level34::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level34::Sign {
    fn from(s: w::Sign) -> crate::level34::Sign {
        match s {
            w::Sign::Straight => crate::level34::Sign::Straight,
            w::Sign::Left => crate::level34::Sign::Left,
            w::Sign::Right => crate::level34::Sign::Right,
            w::Sign::Finish => crate::level34::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level35::Color {
    fn from(c: w::Color) -> crate::level35::Color {
        match c {
            w::Color::Red => crate::level35::Color::Red,
            w::Color::Blue => crate::level35::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level35::Sign {
    fn from(s: w::Sign) -> crate::level35::Sign {
        match s {
            w::Sign::Straight => crate::level35::Sign::Straight,
            w::Sign::Left => crate::level35::Sign::Left,
            w::Sign::Right => crate::level35::Sign::Right,
            w::Sign::Finish => crate::level35::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level36::Color {
    fn from(c: w::Color) -> crate::level36::Color {
        match c {
            w::Color::Red => crate::level36::Color::Red,
            w::Color::Blue => crate::level36::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level36::Sign {
    fn from(s: w::Sign) -> crate::level36::Sign {
        match s {
            w::Sign::Straight => crate::level36::Sign::Straight,
            w::Sign::Left => crate::level36::Sign::Left,
            w::Sign::Right => crate::level36::Sign::Right,
            w::Sign::Finish => crate::level36::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level37::Color {
    fn from(c: w::Color) -> crate::level37::Color {
        match c {
            w::Color::Red => crate::level37::Color::Red,
            w::Color::Blue => crate::level37::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level37::Sign {
    fn from(s: w::Sign) -> crate::level37::Sign {
        match s {
            w::Sign::Straight => crate::level37::Sign::Straight,
            w::Sign::Left => crate::level37::Sign::Left,
            w::Sign::Right => crate::level37::Sign::Right,
            w::Sign::Finish => crate::level37::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level38::Color {
    fn from(c: w::Color) -> crate::level38::Color {
        match c {
            w::Color::Red => crate::level38::Color::Red,
            w::Color::Blue => crate::level38::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level38::Sign {
    fn from(s: w::Sign) -> crate::level38::Sign {
        match s {
            w::Sign::Straight => crate::level38::Sign::Straight,
            w::Sign::Left => crate::level38::Sign::Left,
            w::Sign::Right => crate::level38::Sign::Right,
            w::Sign::Finish => crate::level38::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level39::Color {
    fn from(c: w::Color) -> crate::level39::Color {
        match c {
            w::Color::Red => crate::level39::Color::Red,
            w::Color::Blue => crate::level39::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level39::Sign {
    fn from(s: w::Sign) -> crate::level39::Sign {
        match s {
            w::Sign::Straight => crate::level39::Sign::Straight,
            w::Sign::Left => crate::level39::Sign::Left,
            w::Sign::Right => crate::level39::Sign::Right,
            w::Sign::Finish => crate::level39::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level40::Color {
    fn from(c: w::Color) -> crate::level40::Color {
        match c {
            w::Color::Red => crate::level40::Color::Red,
            w::Color::Blue => crate::level40::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level40::Sign {
    fn from(s: w::Sign) -> crate::level40::Sign {
        match s {
            w::Sign::Straight => crate::level40::Sign::Straight,
            w::Sign::Left => crate::level40::Sign::Left,
            w::Sign::Right => crate::level40::Sign::Right,
            w::Sign::Finish => crate::level40::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level41::Color {
    fn from(c: w::Color) -> crate::level41::Color {
        match c {
            w::Color::Red => crate::level41::Color::Red,
            w::Color::Blue => crate::level41::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level41::Sign {
    fn from(s: w::Sign) -> crate::level41::Sign {
        match s {
            w::Sign::Straight => crate::level41::Sign::Straight,
            w::Sign::Left => crate::level41::Sign::Left,
            w::Sign::Right => crate::level41::Sign::Right,
            w::Sign::Finish => crate::level41::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level42::Color {
    fn from(c: w::Color) -> crate::level42::Color {
        match c {
            w::Color::Red => crate::level42::Color::Red,
            w::Color::Blue => crate::level42::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level42::Sign {
    fn from(s: w::Sign) -> crate::level42::Sign {
        match s {
            w::Sign::Straight => crate::level42::Sign::Straight,
            w::Sign::Left => crate::level42::Sign::Left,
            w::Sign::Right => crate::level42::Sign::Right,
            w::Sign::Finish => crate::level42::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level43::Color {
    fn from(c: w::Color) -> crate::level43::Color {
        match c {
            w::Color::Red => crate::level43::Color::Red,
            w::Color::Blue => crate::level43::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level43::Sign {
    fn from(s: w::Sign) -> crate::level43::Sign {
        match s {
            w::Sign::Straight => crate::level43::Sign::Straight,
            w::Sign::Left => crate::level43::Sign::Left,
            w::Sign::Right => crate::level43::Sign::Right,
            w::Sign::Finish => crate::level43::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level44::Color {
    fn from(c: w::Color) -> crate::level44::Color {
        match c {
            w::Color::Red => crate::level44::Color::Red,
            w::Color::Blue => crate::level44::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level44::Sign {
    fn from(s: w::Sign) -> crate::level44::Sign {
        match s {
            w::Sign::Straight => crate::level44::Sign::Straight,
            w::Sign::Left => crate::level44::Sign::Left,
            w::Sign::Right => crate::level44::Sign::Right,
            w::Sign::Finish => crate::level44::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level45::Color {
    fn from(c: w::Color) -> crate::level45::Color {
        match c {
            w::Color::Red => crate::level45::Color::Red,
            w::Color::Blue => crate::level45::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level45::Sign {
    fn from(s: w::Sign) -> crate::level45::Sign {
        match s {
            w::Sign::Straight => crate::level45::Sign::Straight,
            w::Sign::Left => crate::level45::Sign::Left,
            w::Sign::Right => crate::level45::Sign::Right,
            w::Sign::Finish => crate::level45::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level46::Color {
    fn from(c: w::Color) -> crate::level46::Color {
        match c {
            w::Color::Red => crate::level46::Color::Red,
            w::Color::Blue => crate::level46::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level46::Sign {
    fn from(s: w::Sign) -> crate::level46::Sign {
        match s {
            w::Sign::Straight => crate::level46::Sign::Straight,
            w::Sign::Left => crate::level46::Sign::Left,
            w::Sign::Right => crate::level46::Sign::Right,
            w::Sign::Finish => crate::level46::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level47::Color {
    fn from(c: w::Color) -> crate::level47::Color {
        match c {
            w::Color::Red => crate::level47::Color::Red,
            w::Color::Blue => crate::level47::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level47::Sign {
    fn from(s: w::Sign) -> crate::level47::Sign {
        match s {
            w::Sign::Straight => crate::level47::Sign::Straight,
            w::Sign::Left => crate::level47::Sign::Left,
            w::Sign::Right => crate::level47::Sign::Right,
            w::Sign::Finish => crate::level47::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level48::Color {
    fn from(c: w::Color) -> crate::level48::Color {
        match c {
            w::Color::Red => crate::level48::Color::Red,
            w::Color::Blue => crate::level48::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level48::Sign {
    fn from(s: w::Sign) -> crate::level48::Sign {
        match s {
            w::Sign::Straight => crate::level48::Sign::Straight,
            w::Sign::Left => crate::level48::Sign::Left,
            w::Sign::Right => crate::level48::Sign::Right,
            w::Sign::Finish => crate::level48::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level49::Color {
    fn from(c: w::Color) -> crate::level49::Color {
        match c {
            w::Color::Red => crate::level49::Color::Red,
            w::Color::Blue => crate::level49::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level49::Sign {
    fn from(s: w::Sign) -> crate::level49::Sign {
        match s {
            w::Sign::Straight => crate::level49::Sign::Straight,
            w::Sign::Left => crate::level49::Sign::Left,
            w::Sign::Right => crate::level49::Sign::Right,
            w::Sign::Finish => crate::level49::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level50::Color {
    fn from(c: w::Color) -> crate::level50::Color {
        match c {
            w::Color::Red => crate::level50::Color::Red,
            w::Color::Blue => crate::level50::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level50::Sign {
    fn from(s: w::Sign) -> crate::level50::Sign {
        match s {
            w::Sign::Straight => crate::level50::Sign::Straight,
            w::Sign::Left => crate::level50::Sign::Left,
            w::Sign::Right => crate::level50::Sign::Right,
            w::Sign::Finish => crate::level50::Sign::Finish,
        }
    }
}
impl From<w::Color> for crate::level51::Color {
    fn from(c: w::Color) -> crate::level51::Color {
        match c {
            w::Color::Red => crate::level51::Color::Red,
            w::Color::Blue => crate::level51::Color::Blue,
        }
    }
}
impl From<w::Sign> for crate::level51::Sign {
    fn from(s: w::Sign) -> crate::level51::Sign {
        match s {
            w::Sign::Straight => crate::level51::Sign::Straight,
            w::Sign::Left => crate::level51::Sign::Left,
            w::Sign::Right => crate::level51::Sign::Right,
            w::Sign::Finish => crate::level51::Sign::Finish,
        }
    }
}
