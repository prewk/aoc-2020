use std::collections::HashSet;

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
        let answers = group.lines().map(|line| Answers::from(line)).collect();

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

    pub fn count_questions(&self) -> u64 {
        self.gather_yes().len() as u64
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
    fn test_groups() {
        let input1 = "a\n\
                           b\n\
                           c";

        let g1 = Group::from(input1);

        let input2 = "ab\n\
                           ac";

        let g2 = Group::from(input2);

        assert_eq!(g1.count_questions(), 3);
        assert_eq!(g2.count_questions(), 3);
    }
}