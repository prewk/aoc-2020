use regex::{Regex};

pub struct PasswordEntry {
    pub min: usize,
    pub max: usize,
    pub char: char,
    pub password: String,
}

impl PasswordEntry {
    pub fn from_line(line: &str) -> PasswordEntry {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)+$").unwrap();
        let captures = re.captures(line).unwrap();

        let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let char = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str().to_string();

        PasswordEntry {
            min,
            max,
            char,
            password
        }
    }

    pub fn validate(&self) -> bool {
        let cnt = self.password
            .chars()
            .filter(|c| c == &self.char)
            .count();

        cnt >= self.min  && cnt <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        let entry = PasswordEntry::from_line(&String::from("1-3 a: abcde"));

        assert_eq!(entry.min, 1);
        assert_eq!(entry.max, 3);
        assert_eq!(entry.char, 'a');
        assert_eq!(entry.password, "abcde".to_string());
    }

    #[test]
    fn test_validate() {
        let valid1 = PasswordEntry::from_line(&String::from("1-3 a: abcde"));
        let invalid1 = PasswordEntry::from_line(&String::from("1-3 a: aaaae"));
        let invalid2 = PasswordEntry::from_line(&String::from("1-3 a: xxxxx"));

        assert!(valid1.validate());
        assert!(!invalid1.validate());
        assert!(!invalid2.validate());
    }
}