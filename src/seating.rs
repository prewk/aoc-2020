#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum State {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for State {
    fn from(letter: char) -> Self {
        match letter {
            '.' => State::Floor,
            '#' => State::Occupied,
            'L' => State::Empty,
            _ => panic!("Invalid char: {}", letter),
        }
    }
}

#[derive(Debug)]
pub struct Room {
    pub coords: Vec<State>,
    pub width: usize,
    pub height: usize,
}

impl From<&str> for Room {
    fn from(input: &str) -> Self {
        let mut width = 0;
        let mut coords = vec![];

        input.lines().for_each(|line| {
            width = line.len();
            line.chars().for_each(|letter| {
                coords.push(State::from(letter));
            });
        });

        Room {
            width,
            height: coords.len() / width,
            coords,
        }
    }
}

pub struct Adjacent {
    up: Option<State>,
    up_right: Option<State>,
    right: Option<State>,
    down_right: Option<State>,
    down: Option<State>,
    down_left: Option<State>,
    left: Option<State>,
    up_left: Option<State>,
}

impl Room {
    pub fn get(&self, x: usize, y: usize) -> Option<State> {
        self.coords.get(y * self.width + x).map(|state| state.clone())
    }
    pub fn get_adjacent(&self, x: usize, y: usize) -> Adjacent {
        Adjacent {
            up: self.get(x, y - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let input = "L.LL.LL.LL\n\
                           LLLLLLL.LL\n\
                           L.L.L..L..\n\
                           LLLL.LL.LL\n\
                           L.LL.LL.LL\n\
                           L.LLLLL.LL\n\
                           ..L.L.....\n\
                           LLLLLLLLLL\n\
                           L.LLLLLL.L\n\
                           L.LLLLL.LL";

        let room = Room::from(input);

        assert_eq!(room.get(1, 1), Some(State::Empty));
        assert_eq!(room.get(7, 1), Some(State::Floor));
    }
}
