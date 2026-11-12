`Sweep::next` 만 채우면 돼요: `let point = self.points[self.next_index]; self.next_index = (self.next_index + 1) % self.points.len(); Some(point)`. 나머지(보이는 코어 찾기, 지점으로 걸어가기)는 이미 짜여 있어요.
