use anyhow::{Result, Context};

pub struct Bus {
    id: Option<u64>,
}

impl From<&str> for Bus {
    fn from(id: &str) -> Self {
        Bus {
            id: match id {
                "x" => None,
                _ => Some(id.parse::<u64>().unwrap()),
            },
        }
    }
}

pub struct Table {
    buses: Vec<Bus>,
}

impl From<&str> for Table {
    fn from(line: &str) -> Self {
        Table {
            buses: line.split(",").map(|id| Bus::from(id)).collect(),
        }
    }
}

impl Table {
    pub fn find_earliest_bus(&self, ts: u64) -> Result<(u64, u64)> {
        self.buses
            .iter()
            .filter(|bus| bus.id.is_some())
            .map(|bus| (bus.id.unwrap(), (ts as f64 / bus.id.unwrap() as f64).ceil() as u64 * bus.id.unwrap()))
            .min_by(|(_, approx_a), (_, approx_b)| (approx_a - &ts).cmp(&(approx_b - &ts)))
            .map(|(id, approx)| (id, approx - ts))
            .context("Couldn't find a bus")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_earliest_bus() {
        let t = Table::from("7,13,x,x,59,x,31,19");

        assert_eq!(t.find_earliest_bus(939).unwrap(), (59, 5));
    }
}