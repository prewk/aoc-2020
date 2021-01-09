use anyhow::{Result, Context};
use std::fmt::{Display, Formatter};

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

impl Adjacent {
    fn are_all_unoccupied(&self) -> bool {
        !vec![
            self.up,
            self.up_right,
            self.right,
            self.down_right,
            self.down,
            self.down_left,
            self.left,
            self.up_left,
        ]
            .iter()
            .any(|state| match state {
                None => false,
                Some(State::Occupied) => true,
                Some(State::Empty) => false,
                Some(State::Floor) => false,
            })
    }

    fn are_4_or_more_occupied(&self) -> bool {
        vec![
            self.up,
            self.up_right,
            self.right,
            self.down_right,
            self.down,
            self.down_left,
            self.left,
            self.up_left,
        ]
            .iter()
            .filter(|state| {
                if let Some(v) = state {
                    v == &State::Occupied
                } else {
                    false
                }
            })
            .count()
            >= 4
    }

    pub fn tick(&self, seat: &State) -> State {
        match seat {
            State::Floor => State::Floor,
            State::Empty => match self.are_all_unoccupied() {
                true => State::Occupied,
                false => State::Empty,
            }
            State::Occupied => match self.are_4_or_more_occupied() {
                true => State::Empty,
                false => State::Occupied,
            }
        }
    }
}

#[derive(Debug)]
pub struct Room {
    pub coords: Vec<State>,
    pub width: usize,
    pub height: usize,
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec![];

        for (i, state) in self.coords.iter().enumerate() {
            if i % self.width == 0 {
                chars.push(vec![]);
            }

            chars.last_mut().unwrap().push(match state {
                State::Floor => ".",
                State::Empty => "L",
                State::Occupied => "#",
            })
        }

        let mut formatted = vec![];

        for states in chars {
            formatted.push(states.join(""));
        }

        write!(f, "{}", formatted.join("\n"))
    }
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

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.coords.eq(&other.coords)
    }

    fn ne(&self, other: &Self) -> bool {
        self.coords.ne(&other.coords)
    }
}

impl Room {
    pub fn get(&self, x: usize, y: usize) -> Option<State> {
        self.coords
            .get(y * self.width + x)
            .map(|state| state.clone())
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> Adjacent {
        Adjacent {
            up: match y == 0 { true => None, false => self.get(x, y - 1) },
            up_right: match x >= (self.width - 1) || y == 0 { true => None, false => self.get(x + 1, y - 1) },
            right: match x >= (self.width - 1) { true => None, false => self.get(x + 1, y) },
            down_right: match x >= (self.width - 1) || y >= (self.height - 1) { true => None, false => self.get(x + 1, y + 1) },
            down: match y >= (self.height - 1) { true => None, false => self.get(x, y + 1) },
            down_left: match x == 0 || y >= (self.height - 1) { true => None, false => self.get(x - 1, y + 1) },
            left: match x == 0 { true => None, false => self.get(x - 1, y) },
            up_left: match x == 0 || y == 0 { true => None, false =>  self.get(x - 1, y - 1) },
        }
    }

    pub fn tick(&self) -> Result<Room> {
        let mut coords = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let adjacent = self.get_adjacent(x, y);
                coords.push(adjacent.tick(&self.get(x, y).context("Nothing found")?));
            }
        }

        Ok(Room {
            coords,
            width: self.width,
            height: self.height,
        })
    }

    pub fn count_occupied(&self) -> usize {
        self.coords.iter().filter(|&&state| match state {
            State::Occupied => true,
            _ => false,
        }).count()
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

    #[test]
    fn test_rules() {
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

        let t1 = room.tick().unwrap();
        assert_eq!(format!("{}", t1), "#.##.##.##\n\
                                       #######.##\n\
                                       #.#.#..#..\n\
                                       ####.##.##\n\
                                       #.##.##.##\n\
                                       #.#####.##\n\
                                       ..#.#.....\n\
                                       ##########\n\
                                       #.######.#\n\
                                       #.#####.##");

        let t2 = t1.tick().unwrap();
        assert_eq!(format!("{}", t2), "#.LL.L#.##\n\
                                       #LLLLLL.L#\n\
                                       L.L.L..L..\n\
                                       #LLL.LL.L#\n\
                                       #.LL.LL.LL\n\
                                       #.LLLL#.##\n\
                                       ..L.L.....\n\
                                       #LLLLLLLL#\n\
                                       #.LLLLLL.L\n\
                                       #.#LLLL.##");

        let t3 = t2.tick().unwrap();
        assert_eq!(format!("{}", t3), "#.##.L#.##\n\
                                       #L###LL.L#\n\
                                       L.#.#..#..\n\
                                       #L##.##.L#\n\
                                       #.##.LL.LL\n\
                                       #.###L#.##\n\
                                       ..#.#.....\n\
                                       #L######L#\n\
                                       #.LL###L.L\n\
                                       #.#L###.##");

        let t4 = t3.tick().unwrap();
        assert_eq!(format!("{}", t4), "#.#L.L#.##\n\
                                       #LLL#LL.L#\n\
                                       L.L.L..#..\n\
                                       #LLL.##.L#\n\
                                       #.LL.LL.LL\n\
                                       #.LL#L#.##\n\
                                       ..L.L.....\n\
                                       #L#LLLL#L#\n\
                                       #.LLLLLL.L\n\
                                       #.#L#L#.##");

        let t5 = t4.tick().unwrap();
        assert_eq!(format!("{}", t5), "#.#L.L#.##\n\
                                       #LLL#LL.L#\n\
                                       L.#.L..#..\n\
                                       #L##.##.L#\n\
                                       #.#L.LL.LL\n\
                                       #.#L#L#.##\n\
                                       ..L.L.....\n\
                                       #L#L##L#L#\n\
                                       #.LLLLLL.L\n\
                                       #.#L#L#.##");

        let t6 = t5.tick().unwrap();
        assert_eq!(t6, t5);

        assert_eq!(t6.count_occupied(), 37);
    }
}
