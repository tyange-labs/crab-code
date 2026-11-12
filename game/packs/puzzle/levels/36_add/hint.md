impl Add for Score { type Output = Score; fn add(self, other: Score) -> Score { Score { red: self.red + other.red, blue: self.blue + other.blue } } }
