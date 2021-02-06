#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Game {
    start_cnt: usize,
    turn: usize,
    pub numbers: Vec<usize>,
}

impl From<&str> for Game {
    fn from(starting_num: &str) -> Self {
        let numbers: Vec<usize> = starting_num
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        Game {
            start_cnt: numbers.len(),
            turn: 0,
            numbers,
        }
    }
}

impl Game {
    pub fn turn(&mut self) -> Option<usize> {
        self.turn += 1;

        if self.turn <= self.start_cnt {
            return self.numbers.get(self.turn - 1).map(|&num| num);
        }

        let last = self.numbers.last()?;

        let next = self.numbers
            .iter()
            .enumerate()
            .rev()
            .position(|(i, num)| i != self.numbers.len() - 1 && num == last)
            .map(|rev_idx| {
                let last_spoken_round = self.numbers.len() - rev_idx;

                let last_round = self.turn - 1;

                last_round - last_spoken_round
            })
            .unwrap_or(0);

        self.numbers.push(next);

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let g = Game::from("2,0,1,9,5,19");

        assert_eq!(g.numbers, vec![
            2,
            0,
            1,
            9,
            5,
            19,
        ]);
    }

    #[test]
    fn test_turn() {
        let mut g = Game::from("0,3,6");

        for _ in 0..2019 {
            g.turn();
        }

        assert_eq!(g.turn(), Some(436usize));
    }
}