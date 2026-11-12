// 게임과 똑같이 작업 폴더를 준비한다 (디버깅용). 게임팩이 바뀌었으면 옮기는 일까지 한다.
//   cargo run -p crab-host --example ensure -- <작업 폴더> <팩>
use crab_host::{pack, workspace};
use std::path::Path;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p = pack::find(Path::new("game/packs"), &a[2]).expect("팩");
    match workspace::ensure(Path::new(&a[1]), &p, None) {
        Ok(v) => println!("fresh={} migrated={} note={}", v["fresh"], v["migrated"], v["note"]),
        Err(e) => println!("에러: {e}"),
    }
}
