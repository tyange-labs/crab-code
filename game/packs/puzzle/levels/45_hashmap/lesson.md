# HashMap — 키로 찾기

`HashMap<K, V>` 는 키로 값을 찾는 컬렉션이에요. `use std::collections::HashMap;` 으로 가져옵니다.

개수 세기의 정석:

`*counts.entry(n).or_insert(0) += 1;`

`entry(n)` 은 키 n 의 자리를 찾고, `or_insert(0)` 은 없으면 0 을 넣은 뒤 그 값의 `&mut` 를 돌려줘요.

`for (key, value) in &counts` 로 전체를 돌 수 있습니다. 순서는 정해져 있지 않아요.

목표: 비밀번호는 두루마리에서 가장 여러 번 나온 숫자예요.
