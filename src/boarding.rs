
#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum Row {
    Front,
    Back,
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum Col {
    Left,
    Right,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Seat {
    row: [Row; 7],
    col: [Col; 3],
}

impl From<&str> for Seat {
    fn from(seat: &str) -> Self {
        let mut row = [Row::Front; 7];
        let mut col = [Col::Left; 3];

        seat.chars().take(7).enumerate().for_each(|(i, c)| {
            match c {
                'F' => { row[i] = Row::Front; },
                'B' => { row[i] = Row::Back; },
                _ => { panic!("Invalid char encountered: {}", c) },
            }
        });

        seat.chars().skip(7).take(3).enumerate().for_each(|(i, c)| {
            match c {
                'L' => { col[i] = Col::Left; },
                'R' => { col[i] = Col::Right; },
                _ => { panic!("Invalid char encountered: {}", c) },
            }
        });

        Seat {
            row,
            col,
        }
    }
}

impl Seat {
    pub fn find_row_no(&self) -> u64 {
        let mut lower: u64 = 0;
        let mut upper: u64 = 127;

        for i in 0..7 {
            let middle = ((upper - lower) / 2) + lower;

            match self.row[i] {
                Row::Front => {
                    upper = middle;
                },
                Row::Back => {
                    lower = middle + 1;
                },
            }
        }

        lower
    }

    pub fn find_col_no(&self) -> u64 {
        let mut lower: u64 = 0;
        let mut upper: u64 = 7;

        for i in 0..3 {
            let middle = ((upper - lower) / 2) + lower;

            match self.col[i] {
                Col::Left => {
                    upper = middle;
                },
                Col::Right => {
                    lower = middle + 1;
                },
            }
        }

        lower
    }

    pub fn get_seat_id(&self) -> u64 {
        (self.find_row_no() * 8) + self.find_col_no()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_from() {
        let s1 = Seat::from("FBFBBFFRLR");

        let mut r1: [Row; 7] = [Row::Back; 7];
        r1[0] = Row::Front;
        r1[1] = Row::Back;
        r1[2] = Row::Front;
        r1[3] = Row::Back;
        r1[4] = Row::Back;
        r1[5] = Row::Front;
        r1[6] = Row::Front;

        let mut c1: [Col; 3] = [Col::Left; 3];
        c1[0] = Col::Right;
        c1[1] = Col::Left;
        c1[2] = Col::Right;

        assert_eq!(s1, Seat {
            row: r1,
            col: c1,
        });

        assert_eq!(44, s1.find_row_no());
        assert_eq!(5, s1.find_col_no());
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(567, Seat::from("BFFFBBFRRR").get_seat_id());
        assert_eq!(119, Seat::from("FFFBBBFRRR").get_seat_id());
        assert_eq!(820, Seat::from("BBFFBBFRLL").get_seat_id());
    }
}