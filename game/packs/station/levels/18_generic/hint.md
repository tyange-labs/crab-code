`nearest` 만 채우면 돼요: `items.iter().min_by_key(|item| { let (x, y) = item.at(); … })`. 나머지(코어·슬롯 목록 만들기, 걸어가기)는 이미 짜여 있어요.
