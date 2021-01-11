pub enum Degree {
    Deg90,
    Deg180,
    Deg270,
}

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
    fn from(input: &str) -> Self {
        match input {

        }
    }
}