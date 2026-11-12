match note.parse::<u32>() { Ok(n) => unlock(n), Err(_) => unlock(0), } 로 바꾸세요.
