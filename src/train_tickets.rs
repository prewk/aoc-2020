use std::collections::HashMap;
use std::ops::{RangeInclusive};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Ticket {
    values: Vec<usize>,
}

impl From<&str> for Ticket {
    fn from(input: &str) -> Self {
        Ticket {
            values: input.split(",").map(|num| num.parse::<usize>().unwrap()).collect()
        }
    }
}

impl Ticket {
    pub fn get_invalid_sum(&self, rules: &Rules) -> usize {
        self.values
            .iter()
            .fold(0, |acc, num|
                match rules.test_against_all(&num) {
                    true => acc,
                    false => acc + num
                }
            )
    }
}

#[derive(Debug, PartialEq)]
pub struct Rules {
    props: HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Rules {
    pub fn test_against_all(&self, cand: &usize) -> bool {
        self
            .props
            .iter()
            .fold(None, |acc, (_, (r1, r2))|
                match acc {
                    Some(true) => Some(true),
                    _ => Some((cand >= r1.start() && cand <= r1.end()) || (cand >= r2.start() && cand <= r2.end())),
                }
            )
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct Notes {
    props: Rules,
    your: Ticket,
    nearby: Vec<Ticket>,
}

enum InputState {
    Props,
    Your,
    Nearby,
}

impl From<&str> for Notes {
    fn from(input: &str) -> Self {
        let mut props: HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)> = HashMap::new();
        let mut your: Option<Ticket> = None;
        let mut nearby: Vec<Ticket> = vec![];
        let mut state = InputState::Props;

        let prop_re = Regex::new("^([a-z ]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();

        for line in input.lines() {
            match state {
                InputState::Props => {
                    if line.trim().len() == 0 {
                        state = InputState::Your;
                        continue;
                    }

                    let caps = prop_re.captures(line).unwrap();
                    let prop = caps.get(1).unwrap().as_str();
                    let range1 = RangeInclusive::new(
                        caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                        caps.get(3).unwrap().as_str().parse::<usize>().unwrap()
                    );
                    let range2 = RangeInclusive::new(
                        caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                        caps.get(5).unwrap().as_str().parse::<usize>().unwrap()
                    );

                    props.insert(prop.to_string(), (range1, range2));
                }
                InputState::Your => {
                    if line.trim() == "your ticket:" {
                        continue;
                    } else if line.trim().len() == 0 {
                        state = InputState::Nearby;
                        continue;
                    }

                    your = Some(Ticket::from(line));
                }
                InputState::Nearby => {
                    if line.trim() == "nearby tickets:" {
                        continue;
                    }

                    nearby.push(Ticket::from(line));
                }
            }
        }

        Notes {
            props: Rules { props },
            your: your.unwrap(),
            nearby,
        }
    }
}

impl Notes {
    pub fn get_invalid_sum(&self) -> usize {
        self.nearby
            .iter()
            .fold(0, |acc, ticket|
                acc + ticket.get_invalid_sum(&self.props)
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notes() {
        let input = "class: 1-3 or 5-7\n\
                           row: 6-11 or 33-44\n\
                           seat: 13-40 or 45-50\n\
                           \n\
                           your ticket:\n\
                           7,1,14\n\
                           \n\
                           nearby tickets:\n\
                           7,3,47\n\
                           40,4,50\n\
                           55,2,20\n\
                           38,6,12";

        let notes = Notes::from(input);

        let mut props = HashMap::new();

        props.insert("class".to_string(), (RangeInclusive::new(1, 3), RangeInclusive::new(5, 7)));
        props.insert("row".to_string(), (RangeInclusive::new(6, 11), RangeInclusive::new(33, 44)));
        props.insert("seat".to_string(), (RangeInclusive::new(13, 40), RangeInclusive::new(45, 50)));

        assert_eq!(notes, Notes {
            props: Rules { props },
            your: Ticket { values: vec![7, 1, 14] },
            nearby: vec![
                Ticket { values: vec![7, 3, 47] },
                Ticket { values: vec![40, 4, 50] },
                Ticket { values: vec![55, 2, 20] },
                Ticket { values: vec![38, 6, 12] },
            ],
        });

        assert_eq!(notes.get_invalid_sum(), 71);
    }

    #[test]
    fn test_rules() {
        let mut props = HashMap::new();

        props.insert("class".to_string(), (RangeInclusive::new(1, 3), RangeInclusive::new(5, 7)));
        props.insert("row".to_string(), (RangeInclusive::new(6, 11), RangeInclusive::new(33, 44)));
        props.insert("seat".to_string(), (RangeInclusive::new(13, 40), RangeInclusive::new(45, 50)));

        let rules = Rules { props };

        assert_eq!(rules.test_against_all(&3), true);
        assert_eq!(rules.test_against_all(&4), false);
        assert_eq!(rules.test_against_all(&55), false);
        assert_eq!(rules.test_against_all(&38), true);
        assert_eq!(rules.test_against_all(&12), false);
    }

    #[test]
    fn test_get_invalid_sum() {
        let mut props = HashMap::new();

        props.insert("class".to_string(), (RangeInclusive::new(1, 3), RangeInclusive::new(5, 7)));
        props.insert("row".to_string(), (RangeInclusive::new(6, 11), RangeInclusive::new(33, 44)));
        props.insert("seat".to_string(), (RangeInclusive::new(13, 40), RangeInclusive::new(45, 50)));

        let rules = Rules { props };

        let ticket = Ticket { values: vec![40, 4, 50] };

        assert_eq!(ticket.get_invalid_sum(&rules), 4);
    }
}