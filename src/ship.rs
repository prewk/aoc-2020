#[derive(Debug, Copy, Clone)]
pub enum CompDir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
pub enum RelDir {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum Degree {
    Deg90,
    Deg180,
    Deg270,
}

impl From<&str> for Degree {
    fn from(deg: &str) -> Self {
        match deg {
            "90" => Degree::Deg90,
            "180" => Degree::Deg180,
            "270" => Degree::Deg270,
            _ => panic!("Invalid degrees encountered"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    North(u64),
    South(u64),
    East(u64),
    West(u64),
    Left(Degree),
    Right(Degree),
    Forward(u64),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        match line.chars().next() {
            Some('N') => Instruction::North(*&line[1..].parse::<u64>().unwrap()),
            Some('S') => Instruction::South(*&line[1..].parse::<u64>().unwrap()),
            Some('E') => Instruction::East(*&line[1..].parse::<u64>().unwrap()),
            Some('W') => Instruction::West(*&line[1..].parse::<u64>().unwrap()),
            Some('L') => Instruction::Left(Degree::from(&line[1..])),
            Some('R') => Instruction::Right(Degree::from(&line[1..])),
            Some('F') => Instruction::Forward(*&line[1..].parse::<u64>().unwrap()),
            _ => panic!("Invalid instruction prefix"),
        }
    }
}

#[derive(Debug)]
pub struct Boat1 {
    dir: CompDir,
    pos: (i64, i64),
    trail: Vec<(i64, i64)>,
}

impl Boat1 {
    pub fn new() -> Boat1 {
        Boat1 {
            dir: CompDir::East,
            pos: (0, 0),
            trail: vec![],
        }
    }

    fn turn(&self, next_deg: Degree, next_rel_dir: RelDir) -> CompDir {
        match (next_deg, next_rel_dir, &self.dir) {
            (Degree::Deg90, RelDir::Left, &CompDir::North) => CompDir::West,
            (Degree::Deg90, RelDir::Left, &CompDir::East) => CompDir::North,
            (Degree::Deg90, RelDir::Left, &CompDir::South) => CompDir::East,
            (Degree::Deg90, RelDir::Left, &CompDir::West) => CompDir::South,
            (Degree::Deg90, RelDir::Right, &CompDir::North) => CompDir::East,
            (Degree::Deg90, RelDir::Right, &CompDir::East) => CompDir::South,
            (Degree::Deg90, RelDir::Right, &CompDir::South) => CompDir::West,
            (Degree::Deg90, RelDir::Right, &CompDir::West) => CompDir::North,
            (Degree::Deg180, RelDir::Left, &CompDir::North) => CompDir::South,
            (Degree::Deg180, RelDir::Left, &CompDir::East) => CompDir::West,
            (Degree::Deg180, RelDir::Left, &CompDir::South) => CompDir::North,
            (Degree::Deg180, RelDir::Left, &CompDir::West) => CompDir::East,
            (Degree::Deg180, RelDir::Right, &CompDir::North) => CompDir::South,
            (Degree::Deg180, RelDir::Right, &CompDir::East) => CompDir::West,
            (Degree::Deg180, RelDir::Right, &CompDir::South) => CompDir::North,
            (Degree::Deg180, RelDir::Right, &CompDir::West) => CompDir::East,
            (Degree::Deg270, RelDir::Left, &CompDir::North) => CompDir::East,
            (Degree::Deg270, RelDir::Left, &CompDir::East) => CompDir::South,
            (Degree::Deg270, RelDir::Left, &CompDir::South) => CompDir::West,
            (Degree::Deg270, RelDir::Left, &CompDir::West) => CompDir::North,
            (Degree::Deg270, RelDir::Right, &CompDir::North) => CompDir::West,
            (Degree::Deg270, RelDir::Right, &CompDir::East) => CompDir::North,
            (Degree::Deg270, RelDir::Right, &CompDir::South) => CompDir::East,
            (Degree::Deg270, RelDir::Right, &CompDir::West) => CompDir::South,
        }
    }

    pub fn tick(&mut self, instr: &Instruction) {
        let (x, y) = self.pos;
        self.trail.push((x, y));

        match instr {
            Instruction::North(dist) => {
                self.pos = (x, y + *dist as i64);
            }
            Instruction::South(dist) => {
                self.pos = (x, y - *dist as i64);
            }
            Instruction::East(dist) => {
                self.pos = (x + *dist as i64, y);
            }
            Instruction::West(dist) => {
                self.pos = (x - *dist as i64, y);
            }
            Instruction::Left(deg) => {
                self.dir = self.turn(*deg, RelDir::Left);
            }
            Instruction::Right(deg) => {
                self.dir = self.turn(*deg, RelDir::Right);
            }
            Instruction::Forward(dist) => {
                self.pos = match self.dir {
                    CompDir::North => (x, y + *dist as i64),
                    CompDir::East => (x + *dist as i64, y),
                    CompDir::South => (x, y - *dist as i64),
                    CompDir::West => (x - *dist as i64, y),
                };
            }
        };
    }

    pub fn get_manhattan_dist(&self) -> u64 {
        (self.pos.0.abs() + self.pos.1.abs()) as u64
    }
}

pub struct Boat2 {
    waypoint: (i64, i64),
    pos: (i64, i64),
}

pub fn poor_mans_transpose(waypoint: (i64, i64), dir: RelDir) -> (i64, i64) {
    match (waypoint.0 < 0, waypoint.1 < 0, dir) {
        // . . .
        // . B .
        // W . .
        (true, true, RelDir::Left) => (waypoint.1.abs(), waypoint.0.abs()),
        // . . .
        // . B .
        // W . .
        (true, true, RelDir::Right) => (waypoint.1, waypoint.0.abs()),
        // W . .
        // . B .
        // . . .
        (true, false, RelDir::Left) => (-1  * waypoint.1, waypoint.0),
        // W . .
        // . B .
        // . . .
        (true, false, RelDir::Right) => (waypoint.1, waypoint.0.abs()),
        // . . .
        // . B .
        // W . .
        (false, false, RelDir::Left) => (waypoint.1.abs(), waypoint.0.abs()),
        // . . .
        // . B .
        // W . .
        (false, false, RelDir::Right) => (waypoint.1, waypoint.0.abs()),
        // . . .
        // . B .
        // . . W
        (false, true, RelDir::Left) => (waypoint.1.abs(), waypoint.0.abs()),
        // . . .
        // . B .
        // . . W
        (false, true, RelDir::Right) => (waypoint.1, waypoint.0),
    }
}

impl Boat2 {
    pub fn new() -> Boat2 {
        Boat2 {
            waypoint: (10, 1),
            pos: (0, 0),
        }
    }

    pub fn tick(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(dist) => {
                self.waypoint = (self.waypoint.0, self.waypoint.1 + *dist as i64);
            }
            Instruction::South(dist) => {
                self.waypoint = (self.waypoint.0, self.waypoint.1 - *dist as i64);
            }
            Instruction::East(dist) => {
                self.waypoint = (self.waypoint.0 + *dist as i64, self.waypoint.1);
            }
            Instruction::West(dist) => {
                self.waypoint = (self.waypoint.0 - *dist as i64, self.waypoint.1);
            }
            Instruction::Left(deg) => {
                self.waypoint = poor_mans_transpose(self.waypoint, RelDir::Left);
            }
            Instruction::Right(deg) => {
                self.waypoint = poor_mans_transpose(self.waypoint, RelDir::Right);
            }
            Instruction::Forward(dist) => {
                let waypoint_real_pos = (self.pos.0 + self.waypoint.0, self.pos.1 + self.waypoint.1);
                let delta = (waypoint_real_pos.0 - self.pos.0, waypoint_real_pos.1 - self.pos.1);
                let mult_delta = (delta.0 * *dist as i64, delta.1 * *dist as i64);
                self.pos = (self.pos.0 + mult_delta.0, self.pos.1 + mult_delta.1);
            }
        };
    }

    pub fn get_manhattan_dist(&self) -> u64 {
        (self.pos.0.abs() + self.pos.1.abs()) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boat1() {
        let mut b = Boat1::new();

        b.tick(&Instruction::from("F10"));
        b.tick(&Instruction::from("N3"));
        b.tick(&Instruction::from("F7"));
        b.tick(&Instruction::from("R90"));
        b.tick(&Instruction::from("F11"));

        assert_eq!(b.get_manhattan_dist(), 25);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(poor_mans_transpose((-2, -4), RelDir::Right), (-4, 2));
        assert_eq!(poor_mans_transpose((-2, -4), RelDir::Left), (4, 2));
        assert_eq!(poor_mans_transpose((2, -4), RelDir::Right), (-4, 2));
        assert_eq!(poor_mans_transpose((2, -4), RelDir::Left), (4, 2));
        assert_eq!(poor_mans_transpose((-2, -4), RelDir::Right), (-4, 2));
        assert_eq!(poor_mans_transpose((-2, -4), RelDir::Left), (4, 2));
        assert_eq!(poor_mans_transpose((-4, 2), RelDir::Right), (2, 4));
        assert_eq!(poor_mans_transpose((-4, 2), RelDir::Left), (-2, -4));
    }

    #[test]
    fn test_boat2() {
        let mut b = Boat2::new();

        b.tick(&Instruction::from("F10"));
        b.tick(&Instruction::from("N3"));
        b.tick(&Instruction::from("F7"));
        b.tick(&Instruction::from("R90"));
        b.tick(&Instruction::from("F11"));

        assert_eq!(b.get_manhattan_dist(), 286);
    }
}