// 퍼즐 50 · 글자로 된 명령
// 저장하면 Crab Code 게임 창에서 바로 실행돼요. 터미널에서는: cargo run --bin commands

#[allow(unused_imports)]
use puzzle::level50::*;

fn main() {
    let note = read_note();
    for c in note.chars() {
        match c {
            'F' => move_forward(),
            'L' => turn_left(),
        }
    }
}
