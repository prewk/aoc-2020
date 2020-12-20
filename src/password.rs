use regex::{Regex};

pub struct PasswordEntry {
    pub first: usize,
    pub second: usize,
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
            first: min,
            second: max,
            char,
            password
        }
    }

    pub fn sled_rental_validate(&self) -> bool {
        let cnt = self.password
            .chars()
            .filter(|c| c == &self.char)
            .count();

        cnt >= self.first && cnt <= self.second
    }

    pub fn toboggan_validate(&self) -> bool {
        let first_char = self.password.chars().nth(self.first - 1);
        let second_char = self.password.chars().nth(self.second - 1);

        let first_has = first_char.is_some() && first_char.unwrap() == self.char;
        let second_has = second_char.is_some() && second_char.unwrap() == self.char;

        first_has ^ second_has
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        let entry = PasswordEntry::from_line(&String::from("1-3 a: abcde"));

        assert_eq!(entry.first, 1);
        assert_eq!(entry.second, 3);
        assert_eq!(entry.char, 'a');
        assert_eq!(entry.password, "abcde".to_string());
    }

    #[test]
    fn test_sled_rental_validate() {
        let valid1 = PasswordEntry::from_line(&String::from("1-3 a: abcde"));
        let invalid1 = PasswordEntry::from_line(&String::from("1-3 a: aaaae"));
        let invalid2 = PasswordEntry::from_line(&String::from("1-3 a: xxxxx"));

        assert!(valid1.sled_rental_validate());
        assert!(!invalid1.sled_rental_validate());
        assert!(!invalid2.sled_rental_validate());
    }

    #[test]
    fn test_toboggan_validate() {
        let valid1 = PasswordEntry::from_line(&String::from("1-3 a: abcde"));
        let invalid1 = PasswordEntry::from_line(&String::from("1-3 b: cdefg"));
        let invalid2 = PasswordEntry::from_line(&String::from("2-9 c: ccccccccc"));

        assert!(valid1.toboggan_validate());
        assert!(!invalid1.toboggan_validate());
        assert!(!invalid2.toboggan_validate());
    }
}