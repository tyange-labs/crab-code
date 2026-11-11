// 레슨 내용. 순서와 장 구성은 『프로그래밍 러스트』(개정판)의 목차를 따른다.
// 본문에서 `백틱` 으로 감싼 부분은 코드 글꼴로 그려진다.
// 레슨 순서는 sim.rs 의 SPECS 와 같아야 한다 (--verify 가 id 를 대조한다).

pub struct Lesson {
    pub id: &'static str,
    /// CHAPTERS 의 인덱스.
    pub chapter: usize,
    pub title: &'static str,
    pub body: &'static str,
    pub start: &'static str,
    pub solution: &'static str,
    pub hint: &'static str,
    /// 시작 코드가 일부러 컴파일 에러를 내는 레슨이면 그 에러 코드.
    pub start_error: Option<&'static str>,
}

/// (칩에 쓰는 짧은 이름, 전체 이름)
pub const CHAPTERS: &[(&str, &str)] = &[
    ("2", "2장 러스트 둘러보기"),
    ("3", "3장 기본 타입"),
    ("4", "4장 소유와 이동"),
    ("5", "5장 레퍼런스"),
    ("6", "6장 표현식"),
    ("7", "7장 오류 처리"),
    ("9", "9장 스트럭트"),
    ("10", "10장 이늄과 패턴"),
    ("11", "11장 트레이트와 제네릭"),
    ("12", "12·13장 연산자와 유틸리티 트레이트"),
    ("14", "14장 클로저"),
    ("15", "15장 이터레이터"),
    ("16", "16장 컬렉션"),
    ("17", "17장 문자열과 텍스트"),
];

macro_rules! lesson {
    ($id:expr, $ch:expr, $title:expr, $err:expr, $body:expr, $start:expr, $solution:expr, $hint:expr) => {
        Lesson {
            id: $id,
            chapter: $ch,
            title: $title,
            body: $body,
            start: $start,
            solution: $solution,
            hint: $hint,
            start_error: $err,
        }
    };
}

pub const LESSONS: &[Lesson] = &[
    // ───────────────────────── 2장 러스트 둘러보기 ─────────────────────────
    lesson!(
        "call", 0, "첫 발걸음 — 함수 호출", None,
        r#"꽃게 페리스가 보석을 찾고 있어요! 코드로 페리스를 움직여 봅시다.

Rust 프로그램은 `fn main()` 에서 시작해요. 중괄호 안의 명령이 위에서부터 한 줄씩 실행됩니다.

`move_forward();` 처럼 이름 뒤에 `()` 를 붙이면 함수를 호출해요. 문장 끝의 세미콜론 `;` 도 잊지 마세요!

• `move_forward()` 앞으로 한 칸
• `collect()` 발밑의 보석 줍기

목표: 보석까지 걸어가서 주우세요."#,
        r#"fn main() {
    move_forward();
    // 보석이 있는 칸까지 더 걸어간 다음
    // collect(); 로 주워 보세요!
}
"#,
        r#"fn main() {
    move_forward();
    move_forward();
    move_forward();
    collect();
}
"#,
        "보석은 세 칸 앞에 있어요. move_forward(); 를 세 번 쓰고 collect(); 를 부르세요."
    ),
    lesson!(
        "turn", 0, "모퉁이 돌기", None,
        r#"`turn_left()` 와 `turn_right()` 로 제자리에서 90도 돌 수 있어요. 방향은 페리스가 바라보는 쪽 기준입니다.

벽에 부딪히면 프로그램이 멈추니 조심하세요.

`//` 뒤의 글은 주석이에요. 컴퓨터는 무시하고 사람만 읽습니다.

목표: 모퉁이를 돌아 보석을 주우세요."#,
        r#"fn main() {
    move_forward();
    move_forward();
    // 여기서 어느 쪽으로 돌아야 할까요?

}
"#,
        r#"fn main() {
    move_forward();
    move_forward();
    turn_right();
    move_forward();
    move_forward();
    collect();
}
"#,
        "두 칸 간 뒤 turn_right(); 로 아래쪽을 보고, 두 칸 더 가서 collect();"
    ),
    lesson!(
        "for", 0, "반복은 for 에게", None,
        r#"같은 코드를 세 번 복사해 붙이는 건 지루하죠. `for` 반복문을 써 봅시다.

`for _ in 0..3 { ... }` 는 중괄호 안을 3번 반복해요. `0..3` 은 0, 1, 2 를 뜻하는 범위(range)입니다. 몇 번째인지 필요하면 `_` 대신 `i` 같은 이름을 붙이세요.

`println!("{}", i);` 로 값을 출력 창에 찍어 볼 수도 있어요.

목표: 보석 3개를 모두 주우세요. 보석은 두 칸마다 하나씩 있어요."#,
        r#"fn main() {
    for _ in 0..1 {
        move_forward();
        move_forward();
        collect();
    }
}
"#,
        r#"fn main() {
    for i in 0..3 {
        move_forward();
        move_forward();
        collect();
        println!("{}번째 보석!", i + 1);
    }
}
"#,
        "지금은 0..1 이라 한 번만 반복해요. 범위를 0..3 으로 바꿔 보세요."
    ),
    lesson!(
        "fn", 0, "나만의 함수 — fn", None,
        r#"반복되는 동작에 이름을 붙여 함수로 만들 수 있어요. `fn 이름() { ... }` 으로 정의하고 `이름();` 으로 부릅니다.

함수는 `main` 위에 써도, 아래에 써도 괜찮아요.

목표: 계단을 올라가며 보석 4개를 주우세요. 계단 한 칸을 오르는 `step` 함수를 완성하세요."#,
        r#"// 계단 한 칸: 앞으로 가서 보석을 줍고, 위로 한 칸 올라가 다시 오른쪽을 본다.
fn step() {
    move_forward();
    collect();
    turn_left();
    // 여기를 채워 주세요

}

fn main() {
    for _ in 0..4 {
        step();
    }
}
"#,
        r#"fn step() {
    move_forward();
    collect();
    turn_left();
    move_forward();
    turn_right();
}

fn main() {
    for _ in 0..4 {
        step();
    }
}
"#,
        "왼쪽(위)을 본 다음에는 move_forward(); 로 올라가고 turn_right(); 로 다시 오른쪽을 보세요."
    ),
    lesson!(
        "param", 0, "매개변수 — 값을 받는 함수", None,
        r#"함수는 값을 받을 수 있어요. 괄호 안에 `이름: 타입` 을 적습니다.

`fn walk(n: u32)` 의 `u32` 는 0 이상의 정수 타입이에요. Rust 는 매개변수의 타입을 반드시 적게 합니다. 덕분에 함수만 봐도 무엇을 넘겨야 하는지 알 수 있죠.

목표: `walk` 가 n칸 걷도록 완성해서 보석을 주우세요. `main` 은 이미 완성돼 있어요."#,
        r#"// n칸 앞으로 걷는 함수
fn walk(n: u32) {
    // for 로 n번 반복해 보세요

}

fn main() {
    walk(3);
    turn_right();
    walk(4);
    turn_left();
    walk(2);
    collect();
}
"#,
        r#"fn walk(n: u32) {
    for _ in 0..n {
        move_forward();
    }
}

fn main() {
    walk(3);
    turn_right();
    walk(4);
    turn_left();
    walk(2);
    collect();
}
"#,
        "walk 안에 for _ in 0..n { move_forward(); } 를 넣으세요."
    ),
    lesson!(
        "return", 0, "반환값과 세미콜론", Some("E0308"),
        r#"함수가 값을 돌려주려면 `-> 타입` 을 적어요. 그리고 함수 본문의 마지막 식이 곧 반환값입니다. `return` 을 안 써도 돼요.

단, 마지막 식에 `;` 를 붙이면 안 됩니다. 세미콜론은 값을 버린다는 뜻이라, 돌려줄 값이 없어져요.

• `read_dial()` 다이얼의 숫자 읽기 (맵 위에 표시돼요)
• `unlock(숫자)` 바로 앞의 문에 비밀번호 입력

목표: 문의 비밀번호는 다이얼 숫자의 제곱이에요. 컴파일 에러를 고치세요."#,
        r#"fn square(x: u32) -> u32 {
    x * x;
}

fn main() {
    let n = read_dial();
    move_forward();
    unlock(square(n));
}
"#,
        r#"fn square(x: u32) -> u32 {
    x * x
}

fn main() {
    let n = read_dial();
    move_forward();
    unlock(square(n));
}
"#,
        "x * x; 의 세미콜론을 지우세요. 컴파일러의 help 줄도 같은 말을 하고 있어요."
    ),
    // ───────────────────────── 3장 기본 타입 ─────────────────────────
    lesson!(
        "mut", 1, "컴파일러는 친구 — let mut", Some("E0384"),
        r#"`let` 으로 변수를 만들어요. 그런데 Rust 변수는 기본적으로 값을 바꿀 수 없어요(불변)! 바꾸려면 `let mut` 으로 선언해야 합니다.

이번 코드는 컴파일 에러가 나요. 당황하지 말고 출력 창을 읽어 보세요. Rust 컴파일러는 어디가 왜 틀렸는지, 어떻게 고치면 되는지까지 알려 주는 친절한 친구입니다.

목표: 주운 보석 개수가 문의 비밀번호예요. 에러를 고쳐서 문을 여세요."#,
        r#"fn main() {
    let count = 0;
    for _ in 0..4 {
        move_forward();
        collect();
        count += 1;
    }
    move_forward();
    unlock(count);
}
"#,
        r#"fn main() {
    let mut count = 0;
    for _ in 0..4 {
        move_forward();
        collect();
        count += 1;
    }
    move_forward();
    unlock(count);
}
"#,
        "에러 메시지의 help 줄을 보세요. let count 를 let mut count 로 바꾸면 됩니다."
    ),
    lesson!(
        "int", 1, "정수 타입과 오버플로", None,
        r#"Rust 의 정수는 크기가 정해져 있어요. `u8` 은 0~255, `u32` 는 0~약 42억, `i32` 는 음수도 됩니다. 타입은 `let x: u8 = 5;` 처럼 적어요.

담을 수 없는 값이 되면(오버플로) Rust 는 조용히 틀린 값을 내는 대신 프로그램을 멈춥니다. 이걸 패닉(panic)이라고 해요.

`값 as 타입` 은 타입을 바꾸는 캐스팅이에요.

이 코드는 컴파일은 되지만 실행하면 패닉이 납니다. 출력 창에서 이유를 확인하세요.

목표: 비밀번호는 다이얼 × 100 이에요."#,
        r#"fn main() {
    let small: u8 = read_dial() as u8;
    let code = small * 100;
    move_forward();
    unlock(code as u32);
}
"#,
        r#"fn main() {
    let n: u32 = read_dial();
    let code = n * 100;
    move_forward();
    unlock(code);
}
"#,
        "u8 은 255 까지만 담아요. u8 로 바꾸지 말고 u32 인 채로 곱하세요."
    ),
    lesson!(
        "tuple", 1, "튜플 — 값 묶음", None,
        r#"튜플은 값 여러 개를 괄호로 묶은 거예요. `(3, true)` 처럼 타입이 달라도 됩니다.

`let (a, b) = 튜플;` 로 한 번에 풀어서 받을 수 있고, `t.0`, `t.1` 로 하나씩 꺼낼 수도 있어요.

• `gem_offset()` 은 `(u32, u32)` 를 돌려줘요: 보석이 앞으로 몇 칸, 오른쪽으로 몇 칸 떨어져 있는지

목표: 보석 위치가 실행할 때마다 달라요. 튜플 값을 써서 찾아가 주우세요."#,
        r#"fn main() {
    let (ahead, right) = gem_offset();
    println!("앞으로 {}칸, 오른쪽으로 {}칸", ahead, right);
    // ahead 칸 걷고, 오른쪽으로 돌아서 right 칸 걸으세요

}
"#,
        r#"fn main() {
    let (ahead, right) = gem_offset();
    println!("앞으로 {}칸, 오른쪽으로 {}칸", ahead, right);
    for _ in 0..ahead {
        move_forward();
    }
    turn_right();
    for _ in 0..right {
        move_forward();
    }
    collect();
}
"#,
        "for _ in 0..ahead { move_forward(); } 다음에 turn_right(); 그리고 right 만큼 한 번 더."
    ),
    lesson!(
        "array", 1, "배열", None,
        r#"배열은 같은 타입의 값을 정해진 개수만큼 늘어놓은 거예요: `[5, 3, 4]`

`legs[0]` 처럼 번호(인덱스)로 꺼내는데, 번호는 0부터 시작합니다. `legs.len()` 은 길이예요.

`for n in legs { ... }` 로 값을 하나씩 꺼내며 반복할 수 있어요.

목표: 길이 5, 3, 4, 2 인 길을 차례로 걷고, 길 끝마다 오른쪽으로 돌면 보석에 닿아요."#,
        r#"fn main() {
    let legs = [5, 3];
    for n in legs {
        for _ in 0..n {
            move_forward();
        }
        turn_right();
    }
    collect();
}
"#,
        r#"fn main() {
    let legs = [5, 3, 4, 2];
    for n in legs {
        for _ in 0..n {
            move_forward();
        }
        turn_right();
    }
    collect();
}
"#,
        "배열에 나머지 두 길이를 넣으세요: [5, 3, 4, 2]"
    ),
    lesson!(
        "vec", 1, "벡터 Vec", None,
        r#"배열은 길이가 고정이지만, 벡터 `Vec<T>` 는 길이가 늘었다 줄었다 할 수 있어요. 가장 자주 쓰는 컬렉션입니다.

인덱스와 `len()` 은 배열과 똑같이 써요. 마지막 원소의 번호는 `v.len() - 1` 이죠.

`{:?}` 는 디버그 출력이에요. `println!("{:?}", v);` 로 벡터를 통째로 볼 수 있습니다.

• `read_scroll()` 두루마리의 숫자들을 `Vec<u32>` 로 읽기

목표: 비밀번호는 두루마리의 첫 숫자 + 마지막 숫자예요."#,
        r#"fn main() {
    let v = read_scroll();
    println!("{:?}", v);
    let first = v[0];
    // 마지막 숫자는 어떻게 꺼낼까요?
    move_forward();
    unlock(first);
}
"#,
        r#"fn main() {
    let v = read_scroll();
    println!("{:?}", v);
    let first = v[0];
    let last = v[v.len() - 1];
    move_forward();
    unlock(first + last);
}
"#,
        "let last = v[v.len() - 1]; 을 만들고 unlock(first + last); 로 바꾸세요."
    ),
    lesson!(
        "string", 1, "문자열과 usize", Some("E0308"),
        r#"글자를 담는 타입은 `String` 이에요. `"hello"` 같은 리터럴은 `&str` (문자열 슬라이스)이고요. 지금은 '`String` 은 내가 가진 글, `&str` 은 빌려 보는 글' 정도로만 알아 두세요.

`s.len()` 은 길이를 `usize` 로 돌려줘요. `usize` 는 크기와 인덱스에 쓰는 정수 타입입니다.

Rust 는 정수 타입끼리도 자동으로 바꿔 주지 않아요. `as u32` 로 직접 바꿔야 합니다.

• `read_note()` 쪽지의 글을 `String` 으로 읽기

목표: 비밀번호는 쪽지에 적힌 단어의 글자 수예요."#,
        r#"fn main() {
    let note = read_note();
    move_forward();
    unlock(note.len());
}
"#,
        r#"fn main() {
    let note = read_note();
    move_forward();
    unlock(note.len() as u32);
}
"#,
        "unlock 은 u32 를 받는데 len() 은 usize 예요. note.len() as u32 로 바꾸세요."
    ),
    // ───────────────────────── 4장 소유와 이동 ─────────────────────────
    lesson!(
        "move", 2, "소유권 — 주인은 하나", Some("E0382"),
        r#"Rust 의 핵심, 소유권(ownership)입니다. 모든 값에는 주인이 딱 하나 있어요.

`let gem = collect();` 하면 변수 `gem` 이 보석의 주인이 돼요. `deposit(gem)` 으로 넘기면 소유권이 함수로 이동(move)하고, `gem` 은 더 이상 쓸 수 없습니다. 이미 받침대에 올린 보석을 또 올릴 수는 없으니까요!

• `collect()` 는 주운 보석 `Gem` 을 돌려줘요
• `deposit(gem)` 발밑 받침대에 보석 올리기

목표: 받침대 두 개에 보석을 하나씩 올리세요."#,
        r#"fn main() {
    move_forward();
    let gem = collect();
    move_forward();
    // 두 번째 보석도 주워서 변수에 담아야 할 것 같은데...

    move_forward();
    move_forward();
    deposit(gem);
    move_forward();
    deposit(gem); // 같은 보석을 두 번 올릴 수 있을까요?
}
"#,
        r#"fn main() {
    move_forward();
    let gem = collect();
    move_forward();
    let gem2 = collect();
    move_forward();
    move_forward();
    deposit(gem);
    move_forward();
    deposit(gem2);
}
"#,
        "두 번째 보석을 let gem2 = collect(); 로 담고, 마지막 줄을 deposit(gem2); 로 바꾸세요."
    ),
    lesson!(
        "bag", 2, "벡터로 옮겨 담기", None,
        r#"값을 벡터에 넣는 것도 이동이에요.

`let mut bag = Vec::new();` 로 빈 벡터를 만들고 `bag.push(값)` 으로 넣습니다. 넣는 순간 값의 주인은 `bag` 이 돼요.

`for gem in bag { ... }` 는 가방에서 보석을 하나씩 꺼내 줍니다. 꺼낸 보석의 주인은 `gem` 이니 `deposit(gem)` 할 수 있어요. 대신 반복이 끝나면 `bag` 은 다 써 버린 거라 더는 쓸 수 없습니다.

목표: 보석 3개를 모두 가방에 담은 뒤, 받침대 3개에 하나씩 올리세요."#,
        r#"fn main() {
    let mut bag = Vec::new();
    for _ in 0..3 {
        move_forward();
        bag.push(collect());
    }
    move_forward();
    // 가방에서 보석을 하나씩 꺼내 받침대에 올려 보세요.

}
"#,
        r#"fn main() {
    let mut bag = Vec::new();
    for _ in 0..3 {
        move_forward();
        bag.push(collect());
    }
    move_forward();
    for gem in bag {
        move_forward();
        deposit(gem);
    }
}
"#,
        "for gem in bag { move_forward(); deposit(gem); } 를 추가해 보세요."
    ),
    lesson!(
        "clone", 2, "Copy 와 clone — 이동의 예외", Some("E0382"),
        r#"`let b = a;` 는 값을 a 에서 b 로 이동시켜요. 그 뒤로 a 는 못 씁니다.

그런데 정수, `bool`, `char` 같은 단순한 값은 예외예요. 이런 Copy 타입은 이동 대신 그냥 복사돼서 둘 다 쓸 수 있습니다.

`String`, `Vec`, `Gem` 은 Copy 가 아니에요. 복사본이 필요하면 `.clone()` 으로 직접 만들어야 합니다. 비용이 드는 복사는 눈에 보이게 하자는 게 Rust 의 생각이에요.

목표: 비밀번호는 쪽지의 글자 수예요. 에러를 고치세요."#,
        r#"fn main() {
    let note = read_note();
    let backup = note;
    println!("쪽지: {}", note);
    move_forward();
    unlock(backup.len() as u32);
}
"#,
        r#"fn main() {
    let note = read_note();
    let backup = note.clone();
    println!("쪽지: {}", note);
    move_forward();
    unlock(backup.len() as u32);
}
"#,
        "let backup = note.clone(); 으로 복사본을 만들면 note 도 계속 쓸 수 있어요."
    ),
    // ───────────────────────── 5장 레퍼런스 ─────────────────────────
    lesson!(
        "borrow", 3, "빌림 & — 보기만 할게요", Some("E0308"),
        r#"보석 색을 확인하고 싶은데, `color_of` 에 보석을 통째로 넘기면 소유권이 가 버려서 나중에 받침대에 올릴 수 없어요.

그래서 빌려줍니다. `&gem` 은 보석을 가리키는 레퍼런스(reference)예요. 함수는 보기만 하고, 주인은 여전히 나입니다.

• `color_of(&gem)` 은 `Color::Red` 또는 `Color::Blue`

`match` 는 값에 따라 갈래를 나누는 문법이에요. 6장에서 자세히 배웁니다.

목표: 보석 색은 실행할 때마다 달라요. 빨강은 왼쪽, 파랑은 오른쪽 받침대에 올리세요."#,
        r#"fn main() {
    move_forward();
    let gem = collect();
    match color_of(gem) {
        Color::Red => turn_left(),
        Color::Blue => turn_right(),
    }
    move_forward();
    move_forward();
    deposit(gem);
}
"#,
        r#"fn main() {
    move_forward();
    let gem = collect();
    match color_of(&gem) {
        Color::Red => turn_left(),
        Color::Blue => turn_right(),
    }
    move_forward();
    move_forward();
    deposit(gem);
}
"#,
        "color_of(gem) 을 color_of(&gem) 으로. & 하나면 '주는' 게 아니라 '빌려주는' 게 됩니다."
    ),
    lesson!(
        "mutborrow", 3, "가변 빌림 &mut — 광내기", Some("E0596"),
        r#"`&gem` 으로 빌려주면 볼 수만 있어요. 고치게 하려면 `&mut gem` 으로 변경할 수 있는 레퍼런스를 빌려줘야 합니다.

조건이 하나 더 있어요. 애초에 변수가 `let mut` 이어야 `&mut` 로 빌려줄 수 있습니다.

• `polish(&mut gem)` 보석에 광내기

목표: 이 보석은 흐릿해서 그대로는 받침대가 받아 주지 않아요. 광을 낸 뒤 올리세요."#,
        r#"fn main() {
    move_forward();
    let gem = collect();
    polish(&mut gem);
    move_forward();
    move_forward();
    move_forward();
    deposit(gem);
}
"#,
        r#"fn main() {
    move_forward();
    let mut gem = collect();
    polish(&mut gem);
    move_forward();
    move_forward();
    move_forward();
    deposit(gem);
}
"#,
        "let gem 을 let mut gem 으로 바꾸세요. 바꿀 수 있는 변수만 &mut 로 빌려줄 수 있어요."
    ),
    lesson!(
        "shared", 3, "공유 vs. 변경", Some("E0502"),
        r#"빌림에는 규칙이 있어요. 어떤 값에 대해

• 읽기용 `&` 는 동시에 여러 개 있어도 되지만
• 쓰기용 `&mut` 는 딱 하나만, 그것도 `&` 가 하나도 없을 때만 됩니다.

`first` 가 가방 속 보석을 가리키는 동안 `bag.push` 로 가방을 바꾸면, 가방이 커지면서 내용물이 옮겨져 `first` 가 엉뚱한 곳을 가리킬 수 있어요. Rust 는 이런 버그를 컴파일할 때 막아 줍니다.

레퍼런스는 마지막으로 쓰인 곳까지만 살아 있어요.

목표: 줄 순서를 바꿔 에러를 없애고, 보석 2개를 받침대에 올리세요."#,
        r#"fn main() {
    let mut bag = Vec::new();
    move_forward();
    bag.push(collect());
    let first = &bag[0];
    move_forward();
    bag.push(collect());
    println!("첫 보석 색: {:?}", color_of(first));
    move_forward();
    for gem in bag {
        move_forward();
        deposit(gem);
    }
}
"#,
        r#"fn main() {
    let mut bag = Vec::new();
    move_forward();
    bag.push(collect());
    let first = &bag[0];
    println!("첫 보석 색: {:?}", color_of(first));
    move_forward();
    bag.push(collect());
    move_forward();
    for gem in bag {
        move_forward();
        deposit(gem);
    }
}
"#,
        "println! 줄을 두 번째 bag.push 보다 위로 올리세요. first 를 다 쓴 뒤에는 가방을 바꿔도 됩니다."
    ),
    lesson!(
        "lifetime", 3, "수명 — 레퍼런스는 얼마나 사는가", Some("E0106"),
        r#"레퍼런스는 가리키는 값보다 오래 살 수 없어요. 이 '살아 있는 기간' 을 수명(lifetime)이라고 합니다.

함수가 레퍼런스를 돌려줄 때, 그게 어느 인수에서 온 건지 컴파일러가 알 수 없으면 직접 알려 줘야 해요. `'a` 같은 수명 매개변수를 씁니다:

`fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`

'돌려주는 레퍼런스는 a, b 둘 다 살아 있는 동안만 유효하다' 는 뜻이에요.

• `read_two_notes()` 쪽지 두 장을 `(String, String)` 으로 읽기

목표: 비밀번호는 두 단어 중 긴 쪽의 글자 수예요."#,
        r#"fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let (a, b) = read_two_notes();
    let word = longer(&a, &b);
    move_forward();
    unlock(word.len() as u32);
}
"#,
        r#"fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let (a, b) = read_two_notes();
    let word = longer(&a, &b);
    move_forward();
    unlock(word.len() as u32);
}
"#,
        "컴파일러의 help 대로 fn longer<'a>(a: &'a str, b: &'a str) -> &'a str 로 바꾸세요."
    ),
    // ───────────────────────── 6장 표현식 ─────────────────────────
    lesson!(
        "while", 4, "while 과 if — 조건", None,
        r#"이번엔 보석 위치가 실행할 때마다 달라져요. 상황을 보고 판단해야 합니다.

`while 조건 { ... }` 은 조건이 참인 동안 반복하고, `if 조건 { ... }` 은 참일 때만 실행해요. Rust 에서는 조건에 괄호를 두르지 않습니다.

• `front_is_clear()` 앞으로 갈 수 있으면 `true`
• `on_gem()` 발밑에 보석이 있으면 `true`

`!` 는 반대, `&&` 는 그리고, `||` 는 또는 이에요.

목표: 복도 끝까지 가면서 보석을 전부 주우세요."#,
        r#"fn main() {
    while front_is_clear() {
        move_forward();
        // 보석이 있을 때만 주워야 해요.
        // 없는데 collect() 하면 프로그램이 멈춰요!

    }
}
"#,
        r#"fn main() {
    while front_is_clear() {
        move_forward();
        if on_gem() {
            collect();
        }
    }
}
"#,
        "반복문 안에 if on_gem() { collect(); } 를 넣어 보세요."
    ),
    lesson!(
        "ifexpr", 4, "if 는 값을 만든다", Some("E0317"),
        r#"Rust 는 표현식 언어예요. `if` 도 값을 만들어 냅니다:

`let n = if 조건 { 1 } else { 0 };`

블록 `{ ... }` 의 값은 마지막 식(세미콜론 없는)이에요. 그 앞에 문장을 여러 개 써도 됩니다.

값을 만드는 `if` 는 `else` 가 꼭 있어야 해요. 조건이 거짓일 때의 값도 있어야 하니까요.

목표: 주운 보석 개수가 비밀번호예요. 에러를 고치세요."#,
        r#"fn main() {
    let mut count = 0;
    while front_is_clear() {
        move_forward();
        count += if on_gem() {
            collect();
            1
        };
    }
    unlock(count);
}
"#,
        r#"fn main() {
    let mut count = 0;
    while front_is_clear() {
        move_forward();
        count += if on_gem() {
            collect();
            1
        } else {
            0
        };
    }
    unlock(count);
}
"#,
        "닫는 중괄호 뒤에 else { 0 } 을 붙이세요. 보석이 없으면 0 을 더하는 거예요."
    ),
    lesson!(
        "match", 4, "match — 빠짐없이 나누기", Some("E0004"),
        r#"`match` 는 값에 따라 갈래를 나눠요. 중요한 규칙: 모든 경우를 빠짐없이 다뤄야 합니다. 하나라도 빼먹으면 컴파일러가 잡아 줘요!

바닥의 표지판은 네 가지 중 하나예요:

`enum Sign { Straight, Left, Right, Finish }`

• `read_sign()` 발밑의 표지판 읽기. 없으면 `Sign::Straight`

`loop` 는 `break` 를 만날 때까지 계속 도는 반복문입니다.

목표: 표지판을 따라 깃발까지 가세요. 빠진 갈래를 채워 넣으세요."#,
        r#"fn main() {
    loop {
        match read_sign() {
            Sign::Left => turn_left(),
            Sign::Finish => break,
        }
        move_forward();
    }
}
"#,
        r#"fn main() {
    loop {
        match read_sign() {
            Sign::Left => turn_left(),
            Sign::Right => turn_right(),
            Sign::Straight => {}
            Sign::Finish => break,
        }
        move_forward();
    }
}
"#,
        "Sign::Right => turn_right(), 와 Sign::Straight => {} 두 갈래가 빠져 있어요. {} 는 '아무것도 안 함' 입니다."
    ),
    lesson!(
        "loop", 4, "loop 와 break 값", None,
        r#"`loop` 도 표현식이에요. `break 값;` 으로 빠져나오면 그 값이 `loop` 전체의 값이 됩니다:

`let total = loop { ... break steps; ... };`

'찾을 때까지 돌고, 찾은 것을 들고 나온다' 는 패턴에 딱 맞아요.

목표: 복도 길이가 실행할 때마다 달라요. 벽까지 걸은 칸 수가 비밀번호이고, 문은 복도 끝의 오른쪽에 있어요."#,
        r#"fn main() {
    let mut steps = 0;
    let total = loop {
        if !front_is_clear() {
            break 0; // 여기서 무엇을 들고 나와야 할까요?
        }
        move_forward();
        steps += 1;
    };
    turn_right();
    unlock(total);
}
"#,
        r#"fn main() {
    let mut steps = 0;
    let total = loop {
        if !front_is_clear() {
            break steps;
        }
        move_forward();
        steps += 1;
    };
    turn_right();
    unlock(total);
}
"#,
        "break 0; 을 break steps; 로 바꾸세요."
    ),
    // ───────────────────────── 7장 오류 처리 ─────────────────────────
    lesson!(
        "option", 5, "Option — 있을 수도, 없을 수도", Some("E0277"),
        r#"Rust 에는 null 이 없어요. 대신 값이 없을 수도 있으면 `Option` 을 씁니다. `Some(값)` 아니면 `None` 이죠.

`read_number()` 는 `Option<u32>` 를 돌려줘요. 숫자 타일 위면 `Some(n)`, 아니면 `None`. 상자를 열어 보기 전에는 안의 값을 쓸 수 없습니다:

`if let Some(n) = read_number() { ... }`

`match` 로 `Some(n)` 과 `None` 을 나눠도 돼요.

목표: 바닥 숫자의 합이 문의 비밀번호예요."#,
        r#"fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        sum += read_number();
    }
    unlock(sum);
}
"#,
        r#"fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        if let Some(n) = read_number() {
            sum += n;
        }
    }
    unlock(sum);
}
"#,
        "sum += read_number(); 를 if let Some(n) = read_number() { sum += n; } 으로 바꿔 보세요."
    ),
    lesson!(
        "panic", 5, "패닉 — 일어나면 안 되는 일", None,
        r#"Rust 의 오류는 두 종류예요. 패닉은 '프로그램 자체의 버그' 일 때 일어납니다. 범위 밖 인덱스, 0으로 나누기, `None` 에 `.unwrap()` 하기 같은 것들이죠.

패닉이 나면 Rust 는 엉뚱한 메모리를 읽는 대신, 무엇이 어디서 잘못됐는지 말하고 멈춥니다.

이 코드는 컴파일은 되지만 실행하면 패닉이 나요. 출력 창의 메시지를 읽고 고쳐 보세요.

목표: 비밀번호는 두루마리의 마지막 숫자예요."#,
        r#"fn main() {
    let v = read_scroll();
    let last = v[v.len()];
    move_forward();
    unlock(last);
}
"#,
        r#"fn main() {
    let v = read_scroll();
    let last = v[v.len() - 1];
    move_forward();
    unlock(last);
}
"#,
        "인덱스는 0부터라서 마지막 번호는 v.len() - 1 이에요."
    ),
    lesson!(
        "result", 5, "Result — 실패할 수 있는 일", Some("E0308"),
        r#"패닉과 달리 '일어날 수 있는 실패' 는 `Result` 로 다뤄요. 성공이면 `Ok(값)`, 실패면 `Err(이유)` 입니다.

`note.parse::<u32>()` 는 글을 숫자로 바꿔 보는데, 숫자가 아닌 글일 수 있으니 `Result` 를 돌려줘요. `Option` 처럼 열어 보기 전엔 값을 쓸 수 없어서, 실패를 깜빡하고 넘어갈 수가 없습니다.

`match` 로 `Ok(n)` 과 `Err(e)` 를 나눠 처리하세요. 안 쓰는 값은 `Err(_)` 처럼 `_` 로 받으면 돼요.

목표: 쪽지가 숫자면 그 숫자, 숫자가 아니면 0 이 비밀번호예요."#,
        r#"fn main() {
    let note = read_note();
    move_forward();
    let n: u32 = note.parse::<u32>();
    unlock(n);
}
"#,
        r#"fn main() {
    let note = read_note();
    move_forward();
    match note.parse::<u32>() {
        Ok(n) => unlock(n),
        Err(_) => unlock(0),
    }
}
"#,
        "match note.parse::<u32>() { Ok(n) => unlock(n), Err(_) => unlock(0), } 로 바꾸세요."
    ),
    lesson!(
        "question", 5, "? 연산자 — 오류 떠넘기기", Some("E0369"),
        r#"실패할 수 있는 일을 여러 번 하면 `match` 가 겹겹이 쌓여요. 그래서 `?` 가 있습니다.

`let x = a.parse::<u32>()?;`

성공이면 `Ok` 안의 값을 꺼내 주고, 실패면 그 `Err` 를 들고 함수에서 바로 빠져나가요. 그래서 `?` 는 `Result` 를 돌려주는 함수 안에서만 쓸 수 있습니다.

오류 처리는 부른 쪽(`main`)이 한 번만 하면 돼요.

목표: 비밀번호는 두 쪽지 숫자의 합이에요. 하나라도 숫자가 아니면 0 입니다."#,
        r#"use std::num::ParseIntError;

fn sum(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = a.parse::<u32>();
    let y = b.parse::<u32>();
    Ok(x + y)
}

fn main() {
    let (a, b) = read_two_notes();
    move_forward();
    match sum(&a, &b) {
        Ok(n) => unlock(n),
        Err(e) => {
            println!("읽기 실패: {}", e);
            unlock(0);
        }
    }
}
"#,
        r#"use std::num::ParseIntError;

fn sum(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = a.parse::<u32>()?;
    let y = b.parse::<u32>()?;
    Ok(x + y)
}

fn main() {
    let (a, b) = read_two_notes();
    move_forward();
    match sum(&a, &b) {
        Ok(n) => unlock(n),
        Err(e) => {
            println!("읽기 실패: {}", e);
            unlock(0);
        }
    }
}
"#,
        "parse::<u32>() 뒤에 ? 를 붙이세요. 두 줄 모두요."
    ),
    // ───────────────────────── 9장 스트럭트 ─────────────────────────
    lesson!(
        "fields", 6, "스트럭트의 필드", None,
        r#"스트럭트(struct)는 이름 붙은 값들을 하나로 묶은 타입이에요. `inspect` 는 이런 스트럭트를 돌려줍니다:

`struct GemInfo { color: Color, weight: u32, shiny: bool }`

각 값을 필드라고 하고, `info.weight` 처럼 점으로 꺼내요.

• `inspect(&gem)` 보석을 살펴보고 `GemInfo` 를 돌려줘요

목표: 비밀번호는 보석 3개의 무게 합이에요. 무게는 실행할 때마다 달라집니다."#,
        r#"fn main() {
    let mut total = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let info = inspect(&gem);
        println!("{:?}", info);
        // info 의 weight 필드를 total 에 더하세요
    }
    move_forward();
    unlock(total);
}
"#,
        r#"fn main() {
    let mut total = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let info = inspect(&gem);
        println!("{:?}", info);
        total += info.weight;
    }
    move_forward();
    unlock(total);
}
"#,
        "주석 자리에 total += info.weight; 를 쓰세요."
    ),
    lesson!(
        "struct", 6, "나만의 스트럭트", Some("E0560"),
        r#"스트럭트를 직접 만들어 봅시다.

`struct Counter { red: u32, blue: u32 }` 로 타입을 정의하고, `Counter { red: 0, blue: 0 }` 으로 값을 만들어요. 타입 이름은 대문자로 시작하는 게 관례입니다.

필드를 바꾸려면 변수가 `let mut` 이어야 해요: `counter.red += 1;`

목표: 빨강 보석 수를 십의 자리, 파랑 보석 수를 일의 자리로 한 수가 비밀번호예요 (빨강 3, 파랑 2 → 32). 스트럭트 정의를 고치세요."#,
        r#"struct Counter {
    red: u32,
}

fn main() {
    let mut counter = Counter { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            match color_of(&gem) {
                Color::Red => counter.red += 1,
                Color::Blue => counter.blue += 1,
            }
        }
    }
    unlock(counter.red * 10 + counter.blue);
}
"#,
        r#"struct Counter {
    red: u32,
    blue: u32,
}

fn main() {
    let mut counter = Counter { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            match color_of(&gem) {
                Color::Red => counter.red += 1,
                Color::Blue => counter.blue += 1,
            }
        }
    }
    unlock(counter.red * 10 + counter.blue);
}
"#,
        "Counter 정의에 blue: u32, 필드를 추가하세요."
    ),
    lesson!(
        "impl", 6, "impl 로 메서드 정의하기", Some("E0594"),
        r#"스트럭트에 딸린 함수는 `impl` 블록에 써요.

첫 매개변수가 `self` 면 메서드예요. `pack.pick()` 처럼 점으로 부릅니다.
• `&self` 는 읽기만 하는 메서드
• `&mut self` 는 자신을 바꾸는 메서드

`self` 가 없는 `fn new()` 같은 함수는 `Backpack::new()` 로 불러요. 값을 만들어 주는 함수에 흔히 `new` 라는 이름을 붙입니다.

목표: 비밀번호는 보석 무게의 합이에요. 메서드의 `self` 를 올바르게 고치세요."#,
        r#"struct Backpack {
    count: u32,
    weight: u32,
}

impl Backpack {
    fn new() -> Backpack {
        Backpack { count: 0, weight: 0 }
    }

    // 발밑의 보석을 줍고 기록한다.
    fn pick(&self) {
        let gem = collect();
        self.count += 1;
        self.weight += inspect(&gem).weight;
    }
}

fn main() {
    let mut pack = Backpack::new();
    for _ in 0..3 {
        move_forward();
        pack.pick();
    }
    println!("보석 {}개, 무게 {}", pack.count, pack.weight);
    move_forward();
    unlock(pack.weight);
}
"#,
        r#"struct Backpack {
    count: u32,
    weight: u32,
}

impl Backpack {
    fn new() -> Backpack {
        Backpack { count: 0, weight: 0 }
    }

    fn pick(&mut self) {
        let gem = collect();
        self.count += 1;
        self.weight += inspect(&gem).weight;
    }
}

fn main() {
    let mut pack = Backpack::new();
    for _ in 0..3 {
        move_forward();
        pack.pick();
    }
    println!("보석 {}개, 무게 {}", pack.count, pack.weight);
    move_forward();
    unlock(pack.weight);
}
"#,
        "pick 은 자신의 필드를 바꾸니까 fn pick(&mut self) 여야 해요."
    ),
    lesson!(
        "derive", 6, "#[derive] — 공통 기능 얻기", Some("E0277"),
        r#"직접 만든 스트럭트는 처음엔 아무것도 못 해요. `{:?}` 로 출력하는 것조차 안 됩니다.

정의 위에 `#[derive(Debug)]` 를 붙이면 컴파일러가 디버그 출력 기능을 만들어 줘요. `#[derive(Debug, Clone, PartialEq)]` 처럼 여러 개를 한꺼번에 얻을 수도 있습니다. (`PartialEq` 는 `==` 비교예요.)

목표: 에러를 고치고 보석을 주우세요."#,
        r#"struct Report {
    steps: u32,
    found: bool,
}

fn main() {
    let mut report = Report { steps: 0, found: false };
    for _ in 0..3 {
        move_forward();
        report.steps += 1;
    }
    collect();
    report.found = true;
    println!("{:?}", report);
}
"#,
        r#"#[derive(Debug)]
struct Report {
    steps: u32,
    found: bool,
}

fn main() {
    let mut report = Report { steps: 0, found: false };
    for _ in 0..3 {
        move_forward();
        report.steps += 1;
    }
    collect();
    report.found = true;
    println!("{:?}", report);
}
"#,
        "struct Report 바로 윗줄에 #[derive(Debug)] 를 쓰세요."
    ),
    // ───────────────────────── 10장 이늄과 패턴 ─────────────────────────
    lesson!(
        "enumdata", 7, "데이터를 품은 이늄", Some("E0004"),
        r#"이늄(enum)은 '여러 가지 중 하나' 인 타입이에요. Rust 의 이늄은 갈래마다 데이터를 담을 수 있습니다:

`enum Move { Forward(u32), Left, Right }`

`Move::Forward(3)` 은 '앞으로 3칸' 이라는 값이에요. `match` 에서 `Move::Forward(n) => ...` 으로 받으면 안에 든 값이 `n` 에 담깁니다.

`vec![a, b, c]` 는 벡터를 바로 만드는 매크로예요.

목표: 이동 계획을 차례로 실행해 보석을 주우세요. `run` 의 빠진 갈래를 채우세요."#,
        r#"enum Move {
    Forward(u32),
    Left,
    Right,
}

fn run(m: Move) {
    match m {
        Move::Left => turn_left(),
        Move::Right => turn_right(),
    }
}

fn main() {
    let plan = vec![
        Move::Forward(2),
        Move::Left,
        Move::Forward(3),
        Move::Right,
        Move::Forward(3),
    ];
    for m in plan {
        run(m);
    }
    collect();
}
"#,
        r#"enum Move {
    Forward(u32),
    Left,
    Right,
}

fn run(m: Move) {
    match m {
        Move::Forward(n) => {
            for _ in 0..n {
                move_forward();
            }
        }
        Move::Left => turn_left(),
        Move::Right => turn_right(),
    }
}

fn main() {
    let plan = vec![
        Move::Forward(2),
        Move::Left,
        Move::Forward(3),
        Move::Right,
        Move::Forward(3),
    ];
    for m in plan {
        run(m);
    }
    collect();
}
"#,
        "Move::Forward(n) => { for _ in 0..n { move_forward(); } } 갈래를 추가하세요."
    ),
    lesson!(
        "pattern", 7, "패턴 — 가드와 범위", None,
        r#"`match` 의 왼쪽은 패턴이에요. 패턴은 생각보다 많은 걸 할 수 있습니다.

• `Some(n) if n > 5 => ...` 조건(가드) 붙이기
• `Some(1..=3) => ...` 범위
• `Some(1) | Some(2) => ...` 여러 개 중 하나
• `_ => ...` 나머지 전부

갈래는 위에서부터 차례로 맞춰 보니, 좁은 패턴을 위에 두세요.

`%` 는 나머지 연산이에요. `n % 2 == 0` 이면 짝수입니다.

목표: 비밀번호는 바닥 숫자 중 짝수만 더한 값이에요."#,
        r#"fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        match read_number() {
            Some(n) => sum += n,
            None => {}
        }
    }
    unlock(sum);
}
"#,
        r#"fn main() {
    let mut sum = 0;
    while front_is_clear() {
        move_forward();
        match read_number() {
            Some(n) if n % 2 == 0 => sum += n,
            _ => {}
        }
    }
    unlock(sum);
}
"#,
        "Some(n) if n % 2 == 0 => sum += n, 으로 바꾸고, 나머지는 _ => {} 로 받으세요."
    ),
    // ───────────────────────── 11장 트레이트와 제네릭 ─────────────────────────
    lesson!(
        "trait", 8, "트레이트 — 공통된 능력", Some("E0599"),
        r#"트레이트(trait)는 '이런 걸 할 수 있다' 는 약속이에요.

`trait Value { fn value(&self) -> u32; }`

'값어치를 알려 줄 수 있다' 는 능력을 정의했어요. `impl Value for 타입 { ... }` 으로 타입마다 따로 구현합니다. `u32` 같은 기존 타입에도 내 트레이트를 구현할 수 있어요!

`*self` 의 `*` 는 레퍼런스가 가리키는 값을 꺼내는 연산자입니다.

목표: 보석은 무게 × 10, 숫자 타일은 숫자 그대로가 값어치예요. 전부 더한 값이 비밀번호입니다. `u32` 에도 `Value` 를 구현하세요."#,
        r#"trait Value {
    fn value(&self) -> u32;
}

impl Value for GemInfo {
    fn value(&self) -> u32 {
        self.weight * 10
    }
}

// u32 에도 Value 를 구현해 주세요.

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total += inspect(&gem).value();
        }
        if let Some(n) = read_number() {
            total += n.value();
        }
    }
    unlock(total);
}
"#,
        r#"trait Value {
    fn value(&self) -> u32;
}

impl Value for GemInfo {
    fn value(&self) -> u32 {
        self.weight * 10
    }
}

impl Value for u32 {
    fn value(&self) -> u32 {
        *self
    }
}

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total += inspect(&gem).value();
        }
        if let Some(n) = read_number() {
            total += n.value();
        }
    }
    unlock(total);
}
"#,
        "impl Value for u32 { fn value(&self) -> u32 { *self } } 를 추가하세요."
    ),
    lesson!(
        "generic", 8, "제네릭과 트레이트 바운드", Some("E0599"),
        r#"`fn add<T>(item: T)` 의 `T` 는 '어떤 타입이든' 이라는 뜻의 타입 매개변수예요. 이런 함수를 제네릭 함수라고 합니다.

그런데 '어떤 타입이든' 이면 그 값으로 할 수 있는 게 없어요. `value()` 를 부르려면 '`Value` 를 구현한 타입만 받겠다' 고 제한해야 합니다. 이게 트레이트 바운드예요:

`fn add<T: Value>(total: &mut u32, item: T)`

`*total += ...` 은 `&mut` 레퍼런스가 가리키는 값을 바꿉니다.

목표: 지난 레벨과 같아요. `add` 에 바운드를 붙이세요."#,
        r#"trait Value {
    fn value(&self) -> u32;
}

impl Value for GemInfo {
    fn value(&self) -> u32 {
        self.weight * 10
    }
}

impl Value for u32 {
    fn value(&self) -> u32 {
        *self
    }
}

fn add<T>(total: &mut u32, item: T) {
    *total += item.value();
}

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            add(&mut total, inspect(&gem));
        }
        if let Some(n) = read_number() {
            add(&mut total, n);
        }
    }
    unlock(total);
}
"#,
        r#"trait Value {
    fn value(&self) -> u32;
}

impl Value for GemInfo {
    fn value(&self) -> u32 {
        self.weight * 10
    }
}

impl Value for u32 {
    fn value(&self) -> u32 {
        *self
    }
}

fn add<T: Value>(total: &mut u32, item: T) {
    *total += item.value();
}

fn main() {
    let mut total = 0;
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            add(&mut total, inspect(&gem));
        }
        if let Some(n) = read_number() {
            add(&mut total, n);
        }
    }
    unlock(total);
}
"#,
        "fn add<T> 를 fn add<T: Value> 로 바꾸세요."
    ),
    // ───────────────────── 12·13장 연산자와 유틸리티 트레이트 ─────────────────────
    lesson!(
        "add", 9, "연산자 오버로딩 — Add", Some("E0369"),
        r#"`a + b` 는 사실 `a.add(b)` 를 부르는 거예요. `+` 연산자는 `std::ops::Add` 트레이트로 정해집니다. 내 타입에 `Add` 를 구현하면 `+` 를 쓸 수 있어요.

`type Output = Score;` 는 더한 결과의 타입을 정하는 자리입니다.

마찬가지로 `==` 는 `PartialEq`, `<` 는 `PartialOrd`, `[]` 는 `Index` 트레이트예요.

목표: 빨강 보석 무게 합 × 100 + 파랑 보석 무게 합이 비밀번호예요. `Score` 끼리 더할 수 있게 `Add` 를 구현하세요."#,
        r#"use std::ops::Add;

struct Score {
    red: u32,
    blue: u32,
}

// impl Add for Score { ... } 를 써 주세요.
// type Output = Score;
// fn add(self, other: Score) -> Score { ... }

fn score_of(info: GemInfo) -> Score {
    match info.color {
        Color::Red => Score { red: info.weight, blue: 0 },
        Color::Blue => Score { red: 0, blue: info.weight },
    }
}

fn main() {
    let mut total = Score { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total = total + score_of(inspect(&gem));
        }
    }
    unlock(total.red * 100 + total.blue);
}
"#,
        r#"use std::ops::Add;

struct Score {
    red: u32,
    blue: u32,
}

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        Score {
            red: self.red + other.red,
            blue: self.blue + other.blue,
        }
    }
}

fn score_of(info: GemInfo) -> Score {
    match info.color {
        Color::Red => Score { red: info.weight, blue: 0 },
        Color::Blue => Score { red: 0, blue: info.weight },
    }
}

fn main() {
    let mut total = Score { red: 0, blue: 0 };
    while front_is_clear() {
        move_forward();
        if on_gem() {
            let gem = collect();
            total = total + score_of(inspect(&gem));
        }
    }
    unlock(total.red * 100 + total.blue);
}
"#,
        "impl Add for Score { type Output = Score; fn add(self, other: Score) -> Score { Score { red: self.red + other.red, blue: self.blue + other.blue } } }"
    ),
    lesson!(
        "copy", 9, "내 타입을 Copy 로", Some("E0382"),
        r#"4장에서 정수는 Copy 라서 이동 대신 복사된다고 했죠. 직접 만든 스트럭트는 기본적으로 Copy 가 아니에요. 함수에 넘기면 이동해 버립니다.

필드가 전부 Copy 인 작은 스트럭트라면 `#[derive(Clone, Copy)]` 로 Copy 타입으로 만들 수 있어요. `Copy` 는 `Clone` 이 있어야 해서 둘을 같이 씁니다.

`String` 이나 `Vec` 을 품은 타입은 Copy 가 될 수 없어요.

목표: `leg` 를 두 번 쓸 수 있게 고쳐서 보석을 주우세요."#,
        r#"struct Leg {
    steps: u32,
}

fn walk(leg: Leg) {
    for _ in 0..leg.steps {
        move_forward();
    }
}

fn main() {
    let leg = Leg { steps: 3 };
    walk(leg);
    turn_right();
    walk(leg);
    collect();
}
"#,
        r#"#[derive(Clone, Copy)]
struct Leg {
    steps: u32,
}

fn walk(leg: Leg) {
    for _ in 0..leg.steps {
        move_forward();
    }
}

fn main() {
    let leg = Leg { steps: 3 };
    walk(leg);
    turn_right();
    walk(leg);
    collect();
}
"#,
        "struct Leg 위에 #[derive(Clone, Copy)] 를 붙이세요."
    ),
    lesson!(
        "from", 9, "From 과 Into — 타입 변환", Some("E0277"),
        r#"한 타입을 다른 타입으로 바꾸는 표준 방법이 `From` 트레이트예요.

`impl From<Color> for Digit` 을 구현하면 `Digit::from(color)` 를 쓸 수 있고, 덤으로 반대 방향 표기인 `color.into()` 도 저절로 생깁니다.

`struct Digit(u32);` 는 필드 이름이 없는 튜플형 스트럭트예요. 안의 값은 `d.0` 으로 꺼냅니다.

목표: 빨강은 1, 파랑은 2 예요. 보석 3개의 색을 차례로 이어 붙인 세 자리 수가 비밀번호입니다 (빨강-파랑-빨강 → 121)."#,
        r#"struct Digit(u32);

// impl From<Color> for Digit { fn from(color: Color) -> Digit { ... } }

fn main() {
    let mut code = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let d: Digit = color_of(&gem).into();
        code = code * 10 + d.0;
    }
    move_forward();
    unlock(code);
}
"#,
        r#"struct Digit(u32);

impl From<Color> for Digit {
    fn from(color: Color) -> Digit {
        match color {
            Color::Red => Digit(1),
            Color::Blue => Digit(2),
        }
    }
}

fn main() {
    let mut code = 0;
    for _ in 0..3 {
        move_forward();
        let gem = collect();
        let d: Digit = color_of(&gem).into();
        code = code * 10 + d.0;
    }
    move_forward();
    unlock(code);
}
"#,
        "impl From<Color> for Digit { fn from(color: Color) -> Digit { match color { Color::Red => Digit(1), Color::Blue => Digit(2) } } }"
    ),
    // ───────────────────────── 14장 클로저 ─────────────────────────
    lesson!(
        "closure", 10, "클로저 — 변수를 붙잡는 함수", Some("E0596"),
        r#"클로저는 이름 없는 작은 함수예요. `|매개변수| 본문` 으로 쓰고, 변수에 담아 `grab()` 처럼 부릅니다.

보통 함수와 다른 점은 주변 변수를 붙잡아(캡처) 쓸 수 있다는 거예요. 아래의 `grab` 은 바깥의 `count` 를 붙잡아 값을 바꿉니다.

붙잡은 변수를 바꾸는 클로저는 부를 때마다 상태가 변하니, 클로저를 담은 변수도 `let mut` 이어야 해요.

목표: 주운 보석 개수가 비밀번호예요. 에러를 고치세요."#,
        r#"fn main() {
    let mut count = 0;
    let grab = || {
        move_forward();
        collect();
        count += 1;
    };
    grab();
    grab();
    grab();
    grab();
    move_forward();
    unlock(count);
}
"#,
        r#"fn main() {
    let mut count = 0;
    let mut grab = || {
        move_forward();
        collect();
        count += 1;
    };
    grab();
    grab();
    grab();
    grab();
    move_forward();
    unlock(count);
}
"#,
        "let grab 을 let mut grab 으로 바꾸세요."
    ),
    lesson!(
        "callback", 10, "클로저를 받는 함수", None,
        r#"클로저는 다른 함수에 넘길 수 있어요. 받는 쪽은 타입을 `impl Fn()` 으로 적습니다. '인수 없이 부를 수 있는 무언가' 라는 뜻이에요.

`fn repeat(n: u32, f: impl Fn())`

받은 클로저는 `f();` 로 부릅니다. 클로저 안에서 또 `repeat` 을 부를 수도 있어요.

목표: 네모난 길을 돌며 모퉁이의 보석 3개를 주우세요. `repeat` 이 `f` 를 n번 부르도록 완성하세요."#,
        r#"fn repeat(n: u32, f: impl Fn()) {
    // f 를 n번 불러 주세요

}

fn main() {
    repeat(3, || {
        repeat(3, || move_forward());
        collect();
        turn_right();
    });
}
"#,
        r#"fn repeat(n: u32, f: impl Fn()) {
    for _ in 0..n {
        f();
    }
}

fn main() {
    repeat(3, || {
        repeat(3, || move_forward());
        collect();
        turn_right();
    });
}
"#,
        "repeat 안에 for _ in 0..n { f(); } 를 쓰세요."
    ),
    // ───────────────────────── 15장 이터레이터 ─────────────────────────
    lesson!(
        "sum", 11, "이터레이터 — iter 와 sum", None,
        r#"이터레이터는 값을 하나씩 내놓는 것이에요. `for` 반복문도 속으로는 이터레이터를 씁니다.

`v.iter()` 는 벡터의 원소를 차례로 빌려주는 이터레이터를 만들어요. 여기에 `.sum()` 을 부르면 전부 더해 줍니다. `.count()`, `.max()`, `.min()` 도 있어요.

`sum` 은 결과 타입을 알려 줘야 해요: `let total: u32 = v.iter().sum();`

목표: 비밀번호는 두루마리 숫자의 합이에요. 반복문 없이 구해 보세요."#,
        r#"fn main() {
    let v = read_scroll();
    let total: u32 = 0; // v.iter() 로 합을 구해 보세요
    move_forward();
    unlock(total);
}
"#,
        r#"fn main() {
    let v = read_scroll();
    let total: u32 = v.iter().sum();
    move_forward();
    unlock(total);
}
"#,
        "let total: u32 = v.iter().sum();"
    ),
    lesson!(
        "filter", 11, "어댑터 — filter 와 map", None,
        r#"이터레이터에는 어댑터를 줄줄이 이어 붙일 수 있어요.

• `.filter(|x| 조건)` 조건에 맞는 것만 통과
• `.map(|x| 식)` 값을 하나씩 바꾸기

`v.iter()` 가 내놓는 건 레퍼런스 `&u32` 이고, `filter` 는 그걸 또 빌려서 `&&u32` 로 받아요. 그래서 값을 비교하려면 `**x > 50` 처럼 `*` 를 두 번 씁니다.

어댑터는 게을러서, `.count()` 나 `.sum()` 같은 소비자가 불러야 비로소 일해요.

목표: 비밀번호는 두루마리에서 50보다 큰 숫자의 개수예요."#,
        r#"fn main() {
    let v = read_scroll();
    let big = v.iter().count() as u32; // 50보다 큰 것만 세야 해요
    move_forward();
    unlock(big);
}
"#,
        r#"fn main() {
    let v = read_scroll();
    let big = v.iter().filter(|x| **x > 50).count() as u32;
    move_forward();
    unlock(big);
}
"#,
        "v.iter().filter(|x| **x > 50).count() as u32"
    ),
    lesson!(
        "enumerate", 11, "enumerate 와 max_by_key", None,
        r#"`.enumerate()` 는 값마다 번호를 붙여 `(번호, 값)` 튜플로 내놓아요.

`.max_by_key(|(_, n)| **n)` 은 '이 기준으로 가장 큰 것' 을 골라 줍니다. 비어 있을 수도 있으니 `Option` 을 돌려주고요. 여기선 비어 있지 않은 걸 아니까 `.unwrap()` 으로 꺼내도 괜찮아요.

`0..=idx` 는 idx 까지 포함하는 범위입니다.

목표: 보석은 두루마리에서 가장 큰 숫자가 있는 자리에 있어요. 첫 숫자 자리가 한 칸 앞, 둘째 자리가 두 칸 앞입니다. 보석까지 가서 주우세요."#,
        r#"fn main() {
    let v = read_scroll();
    let idx = 0; // 가장 큰 숫자의 번호를 찾아야 해요
    for _ in 0..=idx {
        move_forward();
    }
    collect();
}
"#,
        r#"fn main() {
    let v = read_scroll();
    let (idx, _) = v.iter().enumerate().max_by_key(|(_, n)| **n).unwrap();
    for _ in 0..=idx {
        move_forward();
    }
    collect();
}
"#,
        "let (idx, _) = v.iter().enumerate().max_by_key(|(_, n)| **n).unwrap();"
    ),
    lesson!(
        "fold", 11, "범위와 fold", None,
        r#"범위 `1..=n` 도 이터레이터예요. 벡터 없이 바로 어댑터를 붙일 수 있습니다.

`.fold(시작값, |누적, x| 식)` 은 값을 하나씩 받아 누적해 가는 만능 도구예요. `sum` 도 `fold` 로 만들 수 있죠:

`(1..=n).fold(0, |acc, x| acc + x)`

목표: 비밀번호는 1부터 다이얼 숫자까지 전부 곱한 값이에요 (다이얼이 4면 1×2×3×4 = 24). 아래 코드는 더하고 있네요."#,
        r#"fn main() {
    let n = read_dial();
    let code = (1..=n).fold(0, |acc, x| acc + x);
    move_forward();
    unlock(code);
}
"#,
        r#"fn main() {
    let n = read_dial();
    let code = (1..=n).fold(1, |acc, x| acc * x);
    move_forward();
    unlock(code);
}
"#,
        "곱셈이니 시작값은 1, 식은 acc * x 예요. (1..=n).product::<u32>() 도 됩니다."
    ),
    // ───────────────────────── 16장 컬렉션 ─────────────────────────
    lesson!(
        "hashmap", 12, "HashMap — 키로 찾기", None,
        r#"`HashMap<K, V>` 는 키로 값을 찾는 컬렉션이에요. `use std::collections::HashMap;` 으로 가져옵니다.

개수 세기의 정석:

`*counts.entry(n).or_insert(0) += 1;`

`entry(n)` 은 키 n 의 자리를 찾고, `or_insert(0)` 은 없으면 0 을 넣은 뒤 그 값의 `&mut` 를 돌려줘요.

`for (key, value) in &counts` 로 전체를 돌 수 있습니다. 순서는 정해져 있지 않아요.

목표: 비밀번호는 두루마리에서 가장 여러 번 나온 숫자예요."#,
        r#"use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    for n in read_scroll() {
        *counts.entry(n).or_insert(0) += 1;
    }
    println!("{:?}", counts);

    let mut best = 0;
    let mut best_count = 0;
    for (n, count) in &counts {
        // count 가 best_count 보다 크면 best 와 best_count 를 바꾸세요
    }
    move_forward();
    unlock(best);
}
"#,
        r#"use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    for n in read_scroll() {
        *counts.entry(n).or_insert(0) += 1;
    }
    println!("{:?}", counts);

    let mut best = 0;
    let mut best_count = 0;
    for (n, count) in &counts {
        if *count > best_count {
            best = *n;
            best_count = *count;
        }
    }
    move_forward();
    unlock(best);
}
"#,
        "if *count > best_count { best = *n; best_count = *count; }"
    ),
    lesson!(
        "hashset", 12, "HashSet 과 collect", None,
        r#"`HashSet<T>` 는 중복 없는 값들의 모음이에요. 같은 값을 여러 번 넣어도 하나만 남습니다.

이터레이터의 `.collect()` 는 값들을 컬렉션으로 모아 줘요. 어떤 컬렉션으로 모을지는 타입으로 정합니다:

`let set: HashSet<u32> = v.into_iter().collect();`

`into_iter()` 는 원소를 빌려주는 게 아니라 소유권째 꺼내 주는 이터레이터예요.

목표: 비밀번호는 두루마리에 나오는 서로 다른 숫자의 개수예요."#,
        r#"use std::collections::HashSet;

fn main() {
    let v = read_scroll();
    // v 를 HashSet<u32> 로 모아서 len() 을 세어 보세요
    let kinds = v.len() as u32;
    move_forward();
    unlock(kinds);
}
"#,
        r#"use std::collections::HashSet;

fn main() {
    let v = read_scroll();
    let set: HashSet<u32> = v.into_iter().collect();
    let kinds = set.len() as u32;
    move_forward();
    unlock(kinds);
}
"#,
        "let set: HashSet<u32> = v.into_iter().collect(); 한 다음 set.len() as u32"
    ),
    lesson!(
        "sort", 12, "정렬", Some("E0596"),
        r#"`v.sort()` 는 벡터를 작은 것부터 차례로 정렬해요. 새 벡터를 만드는 게 아니라 그 자리에서 순서를 바꿉니다.

그러니 `sort` 는 `&mut self` 메서드이고, 벡터는 `let mut` 이어야 해요.

거꾸로는 `v.reverse()`, 기준을 정하려면 `v.sort_by_key(|x| 기준)` 을 씁니다.

목표: 비밀번호는 두루마리 숫자 5개 중 크기순으로 가운데인 값이에요."#,
        r#"fn main() {
    let v = read_scroll();
    v.sort();
    println!("{:?}", v);
    move_forward();
    unlock(v[v.len() / 2]);
}
"#,
        r#"fn main() {
    let mut v = read_scroll();
    v.sort();
    println!("{:?}", v);
    move_forward();
    unlock(v[v.len() / 2]);
}
"#,
        "let v 를 let mut v 로 바꾸세요."
    ),
    // ───────────────────────── 17장 문자열과 텍스트 ─────────────────────────
    lesson!(
        "chars", 13, "문자 char", None,
        r#"문자열은 글자 하나하나로 볼 수 있어요. `note.chars()` 는 `char` 를 하나씩 내놓는 이터레이터입니다.

`char` 리터럴은 작은따옴표로 써요: `'a'`. 큰따옴표 `"a"` 는 문자열이라 타입이 다릅니다.

Rust 문자열은 UTF-8 이라 `len()` 은 글자 수가 아니라 바이트 수예요. 한글 한 글자는 3바이트죠. 글자 수는 `.chars().count()` 로 셉니다.

목표: 비밀번호는 쪽지에 들어 있는 글자 `a` 의 개수예요."#,
        r#"fn main() {
    let note = read_note();
    let count = note.len() as u32; // 'a' 만 세야 해요
    move_forward();
    unlock(count);
}
"#,
        r#"fn main() {
    let note = read_note();
    let count = note.chars().filter(|c| *c == 'a').count() as u32;
    move_forward();
    unlock(count);
}
"#,
        "note.chars().filter(|c| *c == 'a').count() as u32"
    ),
    lesson!(
        "split", 13, "쪼개고 변환하기", None,
        r#"`note.split_whitespace()` 는 문자열을 공백으로 쪼갠 조각(`&str`)들을 내놓는 이터레이터예요. 쉼표로 쪼개려면 `note.split(',')` 를 씁니다.

조각은 7장에서 배운 `parse::<u32>()` 로 숫자로 바꿀 수 있어요. 결과는 `Result` 였죠.

반대로 값을 문자열로 만들 때는 `format!("{}-{}", a, b)` 를 써요. `println!` 과 같은 문법인데 출력하는 대신 `String` 을 돌려줍니다.

목표: 쪽지에 숫자가 공백으로 나뉘어 적혀 있어요. 그 합이 비밀번호입니다."#,
        r#"fn main() {
    let note = read_note();
    let mut total = 0;
    for word in note.split_whitespace() {
        println!("조각: {}", word);
        // word 를 숫자로 바꿔 total 에 더하세요
    }
    move_forward();
    unlock(total);
}
"#,
        r#"fn main() {
    let note = read_note();
    let mut total = 0;
    for word in note.split_whitespace() {
        println!("조각: {}", word);
        if let Ok(n) = word.parse::<u32>() {
            total += n;
        }
    }
    move_forward();
    unlock(total);
}
"#,
        "if let Ok(n) = word.parse::<u32>() { total += n; }"
    ),
    lesson!(
        "commands", 13, "글자로 된 명령", Some("E0004"),
        r#"이번 쪽지에는 길 안내가 글자로 적혀 있어요. `F` 는 앞으로, `L` 은 왼쪽으로 돌기, `R` 은 오른쪽으로 돌기입니다.

`char` 도 `match` 로 나눌 수 있어요. 그런데 `char` 가 될 수 있는 값은 백만 가지가 넘으니, 나머지를 전부 받는 `_` 갈래가 꼭 필요합니다.

목표: 쪽지의 명령을 한 글자씩 실행해 깃발에 도착하세요. 쪽지는 실행할 때마다 달라져요."#,
        r#"fn main() {
    let note = read_note();
    for c in note.chars() {
        match c {
            'F' => move_forward(),
            'L' => turn_left(),
        }
    }
}
"#,
        r#"fn main() {
    let note = read_note();
    for c in note.chars() {
        match c {
            'F' => move_forward(),
            'L' => turn_left(),
            'R' => turn_right(),
            _ => {}
        }
    }
}
"#,
        "'R' => turn_right(), 갈래와 나머지를 받는 _ => {} 갈래를 추가하세요."
    ),
    lesson!(
        "final", 13, "졸업 시험 — 소용돌이 미로", None,
        r#"마지막 관문! 지금까지 배운 걸 모아 처음부터 직접 써 보세요.

미로는 한 방향으로 말려 들어가는 소용돌이예요. 길이 막히면 왼쪽으로 돌면 됩니다.

쓸 만한 도구: `while`, `if` / `else`, `front_is_clear()`, `on_gem()`, `read_sign()`

`!=` 는 '같지 않다' 예요: `read_sign() != Sign::Finish`

목표: 보석을 모두 줍고 깃발에 도착하세요."#,
        r#"fn main() {
    // 깃발(Sign::Finish)에 도착할 때까지 반복해 보세요.

}
"#,
        r#"fn main() {
    while read_sign() != Sign::Finish {
        if on_gem() {
            collect();
        }
        if front_is_clear() {
            move_forward();
        } else {
            turn_left();
        }
    }
}
"#,
        "while read_sign() != Sign::Finish { ... } 안에서: 보석이 있으면 줍고, 앞이 비었으면 전진, 아니면 turn_left()."
    ),
];
