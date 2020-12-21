#[derive(Debug, PartialOrd, PartialEq)]
pub enum MapTile {
    Open,
    Tree,
}

#[derive(Debug)]
pub struct MapSegment {
    tiles: Vec<MapTile>,
}

impl MapSegment {
    pub fn from_line(line: &str) -> MapSegment {
        MapSegment {
            tiles: line.chars().map(|char| match char {
                '.' => MapTile::Open,
                '#' => MapTile::Tree,
                _ => panic!("Invalid tile"),
            }).collect()
        }
    }

    pub fn probe(&self, x: usize) -> &MapTile {
        self.tiles.get(x % self.tiles.len()).unwrap()
    }
}

#[derive(PartialEq, Debug)]
enum TobogganStep {
    Start,
    Right1,
    Right2,
    Right3,
    Down1,
}

#[derive(PartialEq, Debug)]
struct Toboggan {
    x: usize,
    y: usize,
    next_step: TobogganStep,
}

impl Toboggan {
    fn new() -> Toboggan {
        Toboggan {
            x: 0,
            y: 0,
            next_step: TobogganStep::Start,
        }
    }
}

impl Iterator for Toboggan {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let mut touched = false;

        match self.next_step {
            TobogganStep::Start => {
                self.next_step = TobogganStep::Right1;
            }
            TobogganStep::Right1 => {
                self.next_step = TobogganStep::Right2;
                self.x += 1;
            }
            TobogganStep::Right2 => {
                self.next_step = TobogganStep::Right3;
                self.x += 1;
            }
            TobogganStep::Right3 => {
                self.next_step = TobogganStep::Down1;
                self.x += 1;
            }
            TobogganStep::Down1 => {
                self.next_step = TobogganStep::Right1;
                self.y += 1;
                touched = true;
            }
        }

        Some((self.x, self.y, touched))
    }
}

#[derive(Debug)]
pub struct Map {
    segments: Vec<MapSegment>,
}

impl Map {
    pub fn from_segments(segments: Vec<MapSegment>) -> Map {
        Map {
            segments,
        }
    }

    pub fn probe(&self, x: usize, y: usize) -> Option<&MapTile> {
        if y >= self.segments.len() {
            return None;
        }

        Some(self.segments.get(y).unwrap().probe(x))
    }

    pub fn count_trees_by_toboggan(&self) -> u64 {
        let toboggan = Toboggan::new();
        let mut trees = 0;

        for (x, y, touched) in toboggan {
            if y == self.segments.len() {
                break;
            }

            let segment = self.segments.get(y).unwrap();

            match segment.probe(x) {
                MapTile::Open => {}
                MapTile::Tree => {
                    if touched {
                        trees += 1;
                    }
                }
            }
        }

        trees
    }

    pub fn print(&self) {
        for s in &self.segments {
            for _ in 0..5 {
                for t in &s.tiles {
                    match t {
                        MapTile::Open => { print!("."); }
                        MapTile::Tree => { print!("#"); }
                    }
                }
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toboggan() {
        let toboggan = Toboggan::new();

        let steps: Vec<(usize, usize, bool)> = toboggan.take(10).collect();

        let v = vec![
            (0, 0, false),
            (1, 0, false),
            (2, 0, false),
            (3, 0, false),
            (3, 1, true),
            (4, 1, false),
            (5, 1, false),
            (6, 1, false),
            (6, 2, true),
            (7, 2, false),
        ];

        assert_eq!(&steps, &v)
    }

    #[test]
    fn test_infinite_map_segment() {
        let segment = MapSegment::from_line(&"..##..".to_string());

        assert_eq!(*segment.probe(0), MapTile::Open);
        assert_eq!(*segment.probe(1), MapTile::Open);
        assert_eq!(*segment.probe(2), MapTile::Tree);
        assert_eq!(*segment.probe(3), MapTile::Tree);
        assert_eq!(*segment.probe(4), MapTile::Open);
        assert_eq!(*segment.probe(5), MapTile::Open);

        assert_eq!(*segment.probe(0 + 6), MapTile::Open);
        assert_eq!(*segment.probe(1 + 6), MapTile::Open);
        assert_eq!(*segment.probe(2 + 6), MapTile::Tree);
        assert_eq!(*segment.probe(3 + 6), MapTile::Tree);
        assert_eq!(*segment.probe(4 + 6), MapTile::Open);
        assert_eq!(*segment.probe(5 + 6), MapTile::Open);
    }

    #[test]
    fn test_map_example1() {
        let input = "..##.......\n\
                           #...#...#..\n\
                           .#....#..#.\n\
                           ..#.#...#.#\n\
                           .#...##..#.\n\
                           ..#.##.....\n\
                           .#.#.#....#\n\
                           .#........#\n\
                           #.##...#...\n\
                           #...##....#\n\
                           .#..#...#.#";

        let map = Map::from_segments(input.lines().map(|line| MapSegment::from_line(line)).collect());

        assert_eq!(map.count_trees_by_toboggan(), 7);
    }
}