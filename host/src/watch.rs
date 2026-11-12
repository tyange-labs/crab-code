// 플레이어가 IDE 에서 레벨 파일을 저장했는지 알아챈다 (수정 시각을 비교한다).

use crate::pack::Pack;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Default)]
pub struct Watcher {
    seen: HashMap<PathBuf, (SystemTime, u64)>,
}

impl Watcher {
    /// 지난번 이후 바뀐 레벨 id 들. 처음 보는 파일은 바뀐 것으로 치지 않는다.
    pub fn poll(&mut self, root: &Path, pack: &Pack) -> Vec<String> {
        let mut changed = Vec::new();
        for level in &pack.levels {
            let file = root.join(pack.level_file(level));
            let Ok(meta) = std::fs::metadata(&file) else { continue };
            let stamp = (meta.modified().unwrap_or(SystemTime::UNIX_EPOCH), meta.len());
            match self.seen.insert(file, stamp) {
                Some(old) if old != stamp => changed.push(level.id.clone()),
                _ => {}
            }
        }
        changed
    }

    /// 게임이 직접 쓴 파일(처음 코드로 되돌리기 등)은 바뀐 것으로 치지 않게 지금 상태를 기억한다.
    pub fn forget_changes(&mut self, root: &Path, pack: &Pack) {
        self.poll(root, pack);
    }
}
