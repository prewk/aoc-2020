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

pub enum TobogganDir {
    Right,
    Down,
}

pub struct TobogganTraveler {
    path: Vec<TobogganDir>,
    next: usize,
    x: usize,
    y: usize,
    first: bool,
}

impl TobogganTraveler {
    pub fn new(path: Vec<TobogganDir>) -> TobogganTraveler {
        TobogganTraveler {
            path,
            next: 0,
            x: 0,
            y: 0,
            first: true,
        }
    }
}

impl Iterator for TobogganTraveler {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let mut hit = false;

        if !self.first {
            let next = self.path.get(self.next).unwrap();

            if self.path.len() - 1 == self.next {
                hit = true;
                self.next = 0;
            } else {
                self.next += 1;
            }

            match next {
                TobogganDir::Right => { self.x += 1; }
                TobogganDir::Down => { self.y += 1; }
            }
        } else {
            self.first = false;
        }

        Some((self.x, self.y, hit))
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

    pub fn count_trees_by_traveler(&self, traveler: TobogganTraveler) -> u64 {
        let mut trees = 0;

        for (x, y, touched) in traveler {
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
    fn test_traveller() {
        let traveler = TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Down,
        ]);

        let steps: Vec<(usize, usize, bool)> = traveler.take(10).collect();

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

        assert_eq!(map.count_trees_by_traveler(TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Down,
        ])), 2);
        assert_eq!(map.count_trees_by_traveler(TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Down,
        ])), 7);
        assert_eq!(map.count_trees_by_traveler(TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Down,
        ])), 3);
        assert_eq!(map.count_trees_by_traveler(TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Right,
            TobogganDir::Down,
        ])), 4);
        assert_eq!(map.count_trees_by_traveler(TobogganTraveler::new(vec![
            TobogganDir::Right,
            TobogganDir::Down,
            TobogganDir::Down,
        ])), 2);
    }
}