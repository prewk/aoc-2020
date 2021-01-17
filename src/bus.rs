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

#[derive(Debug, PartialOrd, PartialEq)]
pub struct OffsetBus {
    offset: usize,
    id: u64,
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

    pub fn get_offset_list(&self) -> Vec<OffsetBus> {
        let offset_list = self.buses
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (offset, b)| {
                if let Some(id) = b.id {
                    acc.push(OffsetBus {
                        offset,
                        id,
                    });
                }

                acc
            });

        offset_list
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

    #[test]
    fn test_get_offset_list() {
        let t = Table::from("7,13,x,x,59,x,31,19");

        assert_eq!(t.get_offset_list(), vec![
            OffsetBus { offset: 0, id: 7 },
            OffsetBus { offset: 1, id: 13 },
            OffsetBus { offset: 4, id: 59 },
            OffsetBus { offset: 6, id: 31 },
            OffsetBus { offset: 7, id: 19 },
        ]);
    }
}