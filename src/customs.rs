use std::collections::{HashSet, HashMap};

pub struct Answers {
    pub yes: HashSet<char>,
}

impl From<&str> for Answers {
    fn from(answers: &str) -> Self {
        let mut yes = HashSet::new();

        for char in answers.chars() {
            yes.insert(char);
        }

        Answers {
            yes,
        }
    }
}

pub struct Group {
    answers: Vec<Answers>,
}

impl From<String> for Group {
    fn from(group: String) -> Self {
        let answers = group
            .lines()
            .filter(|line| line.trim().len() > 0)
            .map(|line| Answers::from(line)).collect();

        Group {
            answers,
        }
    }
}

impl Group {
    fn gather_yes(&self) -> HashSet<&char> {
        let mut yes = HashSet::new();

        for answers in &self.answers {
            for char in &answers.yes {
                yes.insert(char);
            }
        }

        yes
    }

    pub fn count_yes_questions(&self) -> u64 {
        self.gather_yes().len() as u64
    }

    fn gather_all_yes(&self) -> HashMap<&char, usize> {
        let mut all_yes = HashMap::new();

        for answers in &self.answers {
            for char in &answers.yes {
                if all_yes.contains_key(char) {
                    all_yes.insert(char, all_yes.get(char).unwrap() + 1);
                } else {
                    all_yes.insert(char, 1);
                }
            }
        }

        all_yes
    }

    pub fn count_all_yes_questions(&self) -> u64 {
        self.gather_all_yes()
            .iter()
            .fold(0, |acc, (_, &count)| {
                if count == self.answers.len() {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        let a1 = Answers::from("abcx");

        assert_eq!(a1.yes.len(), 4);
        assert!(a1.yes.contains(&'a'));
        assert!(a1.yes.contains(&'b'));
        assert!(a1.yes.contains(&'c'));
        assert!(a1.yes.contains(&'x'));
    }

    #[test]
    fn test_count_yes_questions() {
        let input1 = "a\n\
                           b\n\
                           c";

        let g1 = Group::from(input1.to_string());

        let input2 = "ab\n\
                           ac";

        let g2 = Group::from(input2.to_string());

        assert_eq!(g1.count_yes_questions(), 3);
        assert_eq!(g2.count_yes_questions(), 3);
    }

    #[test]
    fn test_count_all_yes_questions() {
        let input1 = "abc";

        let g1 = Group::from(input1.to_string());

        let input2 = "a\n\
                           b\n\
                           c";

        let g2 = Group::from(input2.to_string());

        let input3 = "ab\n\
                           ac";

        let g3 = Group::from(input3.to_string());

        assert_eq!(g1.count_all_yes_questions(), 3);
        assert_eq!(g2.count_all_yes_questions(), 0);
        assert_eq!(g3.count_all_yes_questions(), 1);
    }
}